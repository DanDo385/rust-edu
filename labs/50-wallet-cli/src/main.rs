// Project 47: Wallet CLI (CAPSTONE)
//
// Complete cryptocurrency wallet with key management, UTXO selection,
// transaction construction, and signing. Integrates digital signatures
// and blockchain concepts into a practical application.

use k256::ecdsa::{SigningKey, VerifyingKey, signature::Signer};
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

fn main() {
    println!("=== Cryptocurrency Wallet CLI ===\n");

    // Create a new wallet
    let mut wallet = Wallet::new("MyWallet".to_string());
    println!("--- Creating New Wallet ---");
    println!("✅ Wallet created successfully!");
    println!("Wallet name: {}", wallet.name);
    println!("Root address: {}", wallet.get_root_address());
    println!();

    // Show initial wallet status
    wallet.show_status();

    // Simulate receiving funds
    println!("--- Receiving Funds ---");
    let receive_addr = wallet.generate_address();
    println!("Generated receiving address: {}", receive_addr);

    // Simulate receiving 3 separate payments
    println!("Simulating receiving funds...");
    wallet.receive_funds("txid_001".to_string(), 0, 150_000_000, receive_addr.clone()); // 1.5 BTC
    println!("✅ Received 1.50000000 BTC");

    let receive_addr2 = wallet.generate_address();
    wallet.receive_funds("txid_002".to_string(), 0, 50_000_000, receive_addr2.clone()); // 0.5 BTC
    println!("✅ Received 0.50000000 BTC");

    wallet.receive_funds("txid_003".to_string(), 0, 30_000_000, receive_addr2.clone()); // 0.3 BTC
    println!("✅ Received 0.30000000 BTC");
    println!();

    // Show updated balance
    wallet.show_status();

    // Create and send a transaction
    println!("--- Creating Transaction ---");
    let recipient = "bc1qxyz789recipient".to_string();
    let amount = 80_000_000; // 0.8 BTC
    let fee_rate = 10; // satoshis per vbyte

    println!("Sending {} BTC to {}", format_btc(amount), recipient);
    println!("Fee rate: {} sat/vB", fee_rate);
    println!();

    match wallet.create_transaction(recipient.clone(), amount, fee_rate) {
        Ok(tx) => {
            println!("✅ Transaction created and signed");
            println!("Transaction ID: {}", tx.txid);
            println!();
            tx.display();

            // Mark UTXOs as spent
            wallet.mark_utxos_spent(&tx.inputs);

            // Add change UTXO back to wallet
            if let Some(change_output) = tx.outputs.iter().find(|o| wallet.is_my_address(&o.address)) {
                wallet.receive_funds(
                    tx.txid.clone(),
                    1,
                    change_output.amount,
                    change_output.address.clone()
                );
            }
        }
        Err(e) => println!("❌ Transaction failed: {:?}", e),
    }

    // Show final status
    wallet.show_status();

    // Demonstrate UTXO selection strategies
    demonstrate_utxo_selection();

    // Demonstrate fee calculation
    demonstrate_fee_calculation();
}

// ============================================================================
// WALLET STRUCTURE
// ============================================================================

#[derive(Debug)]
struct Wallet {
    name: String,
    master_key: SigningKey,
    addresses: Vec<WalletAddress>,
    utxos: HashMap<String, UTXO>, // key: "txid:vout"
    address_index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WalletAddress {
    address: String,
    public_key: String,
    index: u32,
    used: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UTXO {
    txid: String,
    vout: u32,
    amount: u64, // satoshis
    address: String,
    confirmations: u32,
}

impl Wallet {
    fn new(name: String) -> Self {
        let master_key = SigningKey::random(&mut OsRng);

        let mut wallet = Wallet {
            name,
            master_key,
            addresses: Vec::new(),
            utxos: HashMap::new(),
            address_index: 0,
        };

        // Generate initial address
        wallet.generate_address();
        wallet
    }

    fn generate_address(&mut self) -> String {
        // In a real wallet, this would use BIP32 key derivation
        // For simplicity, we'll derive from master key + index
        let address_data = format!("{}:{}",
            hex::encode(self.master_key.verifying_key().to_encoded_point(true).as_bytes()),
            self.address_index
        );

        let mut hasher = Sha256::new();
        hasher.update(address_data.as_bytes());
        let hash = hasher.finalize();

        let address = format!("bc1q{}", hex::encode(&hash[..20]));
        let public_key = hex::encode(self.master_key.verifying_key().to_encoded_point(true).as_bytes());

        self.addresses.push(WalletAddress {
            address: address.clone(),
            public_key,
            index: self.address_index,
            used: false,
        });

        self.address_index += 1;
        address
    }

    fn get_root_address(&self) -> String {
        self.addresses[0].address.clone()
    }

    fn is_my_address(&self, address: &str) -> bool {
        self.addresses.iter().any(|a| a.address == address)
    }

    fn receive_funds(&mut self, txid: String, vout: u32, amount: u64, address: String) {
        let utxo = UTXO {
            txid: txid.clone(),
            vout,
            amount,
            address,
            confirmations: 6, // Assume confirmed
        };

        let key = format!("{}:{}", txid, vout);
        self.utxos.insert(key, utxo);
    }

    fn get_balance(&self) -> u64 {
        self.utxos.values().map(|u| u.amount).sum()
    }

    fn show_status(&self) {
        println!("--- Wallet Status ---");
        println!("Balance: {} BTC", format_btc(self.get_balance()));
        println!("Addresses: {}", self.addresses.len());
        println!("UTXOs: {}", self.utxos.len());
        println!();
    }

    fn create_transaction(
        &self,
        recipient: String,
        amount: u64,
        fee_rate: u64,
    ) -> Result<Transaction, WalletError> {
        // Check if we have enough funds
        if amount > self.get_balance() {
            return Err(WalletError::InsufficientFunds);
        }

        // Select UTXOs (using largest-first strategy)
        let selected_utxos = self.select_utxos_largest_first(amount, fee_rate)?;

        // Calculate totals
        let total_input: u64 = selected_utxos.iter().map(|u| u.amount).sum();

        // Estimate transaction size and fee
        let estimated_size = estimate_tx_size(selected_utxos.len(), 2); // 2 outputs (payment + change)
        let fee = estimated_size * fee_rate;

        if amount + fee > total_input {
            return Err(WalletError::InsufficientFunds);
        }

        let change = total_input - amount - fee;

        // Create inputs
        let inputs: Vec<TxInput> = selected_utxos
            .iter()
            .map(|utxo| TxInput {
                txid: utxo.txid.clone(),
                vout: utxo.vout,
                amount: utxo.amount,
                signature: String::new(), // Will be filled during signing
            })
            .collect();

        // Create outputs
        let mut outputs = vec![
            TxOutput {
                address: recipient.clone(),
                amount,
            }
        ];

        // Add change output if significant
        if change > 546 { // Dust threshold (546 satoshis)
            // Generate new change address (privacy best practice)
            let change_address = self.addresses[0].address.clone(); // Simplified
            outputs.push(TxOutput {
                address: change_address,
                amount: change,
            });
        }

        // Create transaction
        let mut tx = Transaction {
            txid: String::new(),
            inputs,
            outputs,
            fee,
            size: estimated_size,
        };

        // Sign transaction
        self.sign_transaction(&mut tx)?;

        println!("UTXO Selection:");
        println!("  Selected {} UTXO(s)", selected_utxos.len());
        println!("  Total input: {} BTC", format_btc(total_input));
        println!("  Payment: {} BTC", format_btc(amount));
        println!("  Change: {} BTC", format_btc(change));
        println!("  Fee: {} BTC ({} sat)", format_btc(fee), fee);
        println!();

        Ok(tx)
    }

    fn select_utxos_largest_first(
        &self,
        target: u64,
        fee_rate: u64,
    ) -> Result<Vec<UTXO>, WalletError> {
        let mut utxos: Vec<UTXO> = self.utxos.values().cloned().collect();

        // Sort by amount (largest first)
        utxos.sort_by(|a, b| b.amount.cmp(&a.amount));

        let mut selected = Vec::new();
        let mut total = 0u64;

        for utxo in utxos {
            selected.push(utxo.clone());
            total += utxo.amount;

            // Estimate fee for current selection
            let estimated_size = estimate_tx_size(selected.len(), 2);
            let fee = estimated_size * fee_rate;

            // Check if we have enough
            if total >= target + fee {
                return Ok(selected);
            }
        }

        Err(WalletError::InsufficientFunds)
    }

    fn sign_transaction(&self, tx: &mut Transaction) -> Result<(), WalletError> {
        // Create transaction hash
        let tx_hash = tx.calculate_hash();

        // Sign each input
        for input in &mut tx.inputs {
            let signature: k256::ecdsa::Signature = self.master_key.sign(tx_hash.as_ref());
            input.signature = hex::encode(signature.to_der().as_bytes());
        }

        // Calculate final transaction ID
        tx.txid = hex::encode(&tx_hash[..16]); // Simplified

        Ok(())
    }

    fn mark_utxos_spent(&mut self, inputs: &[TxInput]) {
        for input in inputs {
            let key = format!("{}:{}", input.txid, input.vout);
            self.utxos.remove(&key);
        }
    }
}

// ============================================================================
// TRANSACTION STRUCTURE
// ============================================================================

#[derive(Debug, Clone)]
struct Transaction {
    txid: String,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    fee: u64,
    size: u64, // in vbytes
}

#[derive(Debug, Clone)]
struct TxInput {
    txid: String,
    vout: u32,
    amount: u64,
    signature: String,
}

#[derive(Debug, Clone)]
struct TxOutput {
    address: String,
    amount: u64,
}

impl Transaction {
    fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();

        // Hash inputs
        for input in &self.inputs {
            hasher.update(input.txid.as_bytes());
            hasher.update(&input.vout.to_le_bytes());
            hasher.update(&input.amount.to_le_bytes());
        }

        // Hash outputs
        for output in &self.outputs {
            hasher.update(output.address.as_bytes());
            hasher.update(&output.amount.to_le_bytes());
        }

        hasher.finalize().to_vec()
    }

    fn display(&self) {
        println!("Transaction Details:");
        println!("  TXID: {}", self.txid);
        println!("  Size: {} vbytes", self.size);
        println!("  Fee: {} sat ({} sat/vB)", self.fee, self.fee / self.size);
        println!();

        println!("  Inputs ({}):  ", self.inputs.len());
        for (i, input) in self.inputs.iter().enumerate() {
            println!("    [{}] {}:{} - {} BTC",
                i, &input.txid[..8], input.vout, format_btc(input.amount));
        }
        println!();

        println!("  Outputs ({}):", self.outputs.len());
        for (i, output) in self.outputs.iter().enumerate() {
            println!("    [{}] {} - {} BTC",
                i, &output.address[..20], format_btc(output.amount));
        }
        println!();
    }
}

// ============================================================================
// UTXO SELECTION DEMONSTRATIONS
// ============================================================================

fn demonstrate_utxo_selection() {
    println!("--- UTXO Selection Strategies ---");

    // Create test UTXOs
    let utxos = vec![
        UTXO {
            txid: "tx1".to_string(),
            vout: 0,
            amount: 100_000_000, // 1.0 BTC
            address: "addr1".to_string(),
            confirmations: 6,
        },
        UTXO {
            txid: "tx2".to_string(),
            vout: 0,
            amount: 50_000_000, // 0.5 BTC
            address: "addr2".to_string(),
            confirmations: 6,
        },
        UTXO {
            txid: "tx3".to_string(),
            vout: 0,
            amount: 25_000_000, // 0.25 BTC
            address: "addr3".to_string(),
            confirmations: 6,
        },
        UTXO {
            txid: "tx4".to_string(),
            vout: 0,
            amount: 10_000_000, // 0.1 BTC
            address: "addr4".to_string(),
            confirmations: 6,
        },
    ];

    let target = 60_000_000; // Want to send 0.6 BTC

    println!("Available UTXOs: {}", format_btc(utxos.iter().map(|u| u.amount).sum()));
    println!("Target amount: {}", format_btc(target));
    println!();

    // Strategy 1: Largest first
    println!("1. Largest First:");
    let selected = select_largest_first(&utxos, target);
    print_selection(&selected, target);

    // Strategy 2: Smallest first
    println!("2. Smallest First:");
    let selected = select_smallest_first(&utxos, target);
    print_selection(&selected, target);

    // Strategy 3: Exact match (if possible)
    println!("3. Exact Match:");
    if let Some(utxo) = find_exact_match(&utxos, target) {
        println!("   ✅ Found exact match: {} BTC", format_btc(utxo.amount));
        println!("   Change: 0 BTC (perfect!)");
    } else {
        println!("   No exact match available");
    }
    println!();
}

fn select_largest_first(utxos: &[UTXO], target: u64) -> Vec<UTXO> {
    let mut sorted = utxos.to_vec();
    sorted.sort_by(|a, b| b.amount.cmp(&a.amount));

    let mut selected = Vec::new();
    let mut total = 0;

    for utxo in sorted {
        if total >= target {
            break;
        }
        total += utxo.amount;
        selected.push(utxo);
    }

    selected
}

fn select_smallest_first(utxos: &[UTXO], target: u64) -> Vec<UTXO> {
    let mut sorted = utxos.to_vec();
    sorted.sort_by(|a, b| a.amount.cmp(&b.amount));

    let mut selected = Vec::new();
    let mut total = 0;

    for utxo in sorted {
        if total >= target {
            break;
        }
        total += utxo.amount;
        selected.push(utxo);
    }

    selected
}

fn find_exact_match(utxos: &[UTXO], target: u64) -> Option<UTXO> {
    utxos.iter().find(|u| u.amount == target).cloned()
}

fn print_selection(selected: &[UTXO], target: u64) {
    let total: u64 = selected.iter().map(|u| u.amount).sum();
    println!("   Selected {} UTXO(s): {} BTC", selected.len(), format_btc(total));
    println!("   Change: {} BTC", format_btc(total.saturating_sub(target)));
    println!();
}

// ============================================================================
// FEE CALCULATION
// ============================================================================

fn demonstrate_fee_calculation() {
    println!("--- Fee Calculation ---");

    // Transaction size estimation (simplified)
    println!("Transaction Size Estimates:");
    println!("  1 input, 2 outputs: ~{} vbytes", estimate_tx_size(1, 2));
    println!("  2 inputs, 2 outputs: ~{} vbytes", estimate_tx_size(2, 2));
    println!("  5 inputs, 2 outputs: ~{} vbytes", estimate_tx_size(5, 2));
    println!();

    // Fee calculation at different rates
    let tx_size = estimate_tx_size(2, 2);
    println!("For a {} vbyte transaction:", tx_size);

    let rates = vec![1, 5, 10, 20, 50, 100];
    for rate in rates {
        let fee = tx_size * rate;
        println!("  {} sat/vB = {} sat ({} BTC)",
            rate, fee, format_btc(fee));
    }
    println!();

    // Fee as percentage of amount
    let amount = 100_000_000; // 1 BTC
    println!("Fee as percentage of 1 BTC transaction:");
    for rate in [10, 50, 100] {
        let fee = tx_size * rate;
        let percentage = (fee as f64 / amount as f64) * 100.0;
        println!("  {} sat/vB = {:.4}%", rate, percentage);
    }
    println!();
}

fn estimate_tx_size(inputs: usize, outputs: usize) -> u64 {
    // Simplified transaction size estimation
    // Real calculation depends on signature type (P2PKH, P2WPKH, etc.)

    const BASE_SIZE: u64 = 10; // Version, locktime, etc.
    const INPUT_SIZE: u64 = 148; // Typical P2PKH input
    const OUTPUT_SIZE: u64 = 34; // Typical P2PKH output

    BASE_SIZE + (inputs as u64 * INPUT_SIZE) + (outputs as u64 * OUTPUT_SIZE)
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

#[derive(Debug)]
enum WalletError {
    InsufficientFunds,
    InvalidAddress,
    FeeTooHigh,
    SigningFailed,
}

// ============================================================================
// UTILITIES
// ============================================================================

fn format_btc(satoshis: u64) -> String {
    let btc = satoshis as f64 / 100_000_000.0;
    format!("{:.8}", btc)
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. OWNERSHIP IN WALLET
//    The Wallet owns all its UTXOs, addresses, and keys.
//    When creating a transaction, we clone() UTXOs (cheap for small data).
//    Could optimize with Rc/Arc if wallet is shared across threads.
//
// 2. HASHMAP PERFORMANCE
//    HashMap<String, UTXO> uses SipHash for security (prevents DoS).
//    Lookup is O(1) average case - fast for checking spent UTXOs.
//    Alternative: BTreeMap for ordered iteration (e.g., by value).
//
// 3. SIGNING OPERATIONS
//    SigningKey lives in Wallet and doesn't escape (good security).
//    Signing creates temporary data that's freed immediately.
//    No garbage collection pauses during time-critical operations.
//
// 4. STRING ALLOCATIONS
//    Address strings are heap-allocated (String, not &str).
//    Frequent cloning in real wallets - consider Arc<str> for optimization.
//    Txid strings are small (< 64 bytes) - stack might be faster.
//
// 5. SERIALIZATION
//    Serde derives make serialization zero-cost at runtime.
//    JSON is human-readable but verbose - consider bincode for production.
//    Wallet file I/O should be encrypted (not shown for simplicity).

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Wallets manage keys, addresses, and UTXOs
// 2. UTXO selection affects fees and privacy
// 3. Change outputs return excess funds to wallet
// 4. Fee = total_inputs - total_outputs
// 5. Always use new addresses for change (privacy)
// 6. Transaction size depends on number of inputs/outputs
// 7. Higher fee rate = faster confirmation
// 8. Wallet security depends on key protection
// 9. HD wallets derive all keys from single seed
// 10. Real wallets need: encryption, backups, recovery

// ============================================================================
// SECURITY BEST PRACTICES
// ============================================================================
// ✅ DO:
//    - Encrypt private keys at rest
//    - Use new addresses for each transaction
//    - Implement proper fee estimation
//    - Validate all addresses before sending
//    - Backup wallet regularly
//    - Use hardware wallets for large amounts
//
// ❌ DON'T:
//    - Store private keys in plaintext
//    - Reuse addresses
//    - Send transactions without verifying fee
//    - Hardcode fee rates
//    - Trust user input without validation
//    - Expose private keys in logs or errors

// ============================================================================
// PRODUCTION CONSIDERATIONS
// ============================================================================
// Real cryptocurrency wallets need:
// 1. BIP39 mnemonic generation (12/24 word seed phrases)
// 2. BIP32 hierarchical deterministic key derivation
// 3. BIP44 multi-account structure (m/44'/0'/0'/0/0)
// 4. Hardware wallet support (Ledger, Trezor integration)
// 5. Multi-signature support (2-of-3, 3-of-5, etc.)
// 6. Replace-by-Fee (RBF) for fee bumping
// 7. Child-Pays-For-Parent (CPFP) fee acceleration
// 8. Coin control (manual UTXO selection)
// 9. Transaction history and labels
// 10. QR code generation for addresses
// 11. Fee estimation API integration
// 12. Network communication (broadcast transactions)
// 13. Block explorer integration
// 14. Exchange rate conversion
// 15. Secure enclave support (iOS/Android)

// ============================================================================
// COMMON MISTAKES TO AVOID
// ============================================================================
// ❌ Not checking for dust (< 546 sat outputs)
// ❌ Creating change outputs smaller than fee
// ❌ Reusing addresses (privacy leak)
// ❌ Not validating recipient addresses
// ❌ Hardcoding fee rates
// ❌ Not handling unconfirmed UTXOs
// ❌ Marking UTXOs as spent before broadcast
// ❌ Not implementing gap limit (BIP44)
// ❌ Storing plaintext private keys
// ❌ Not backing up wallet seed
