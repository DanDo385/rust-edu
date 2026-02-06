// Project 11: UTXO Model
//
// This program implements the UTXO (Unspent Transaction Output) model used by
// Bitcoin and other cryptocurrencies. We'll learn how UTXOs work, how to prevent
// double-spending, and how this model differs from account-based systems.
//
// UTXO Model Core Concept:
// - Instead of account balances, blockchain tracks individual "coins" (UTXOs)
// - Each transaction CONSUMES some UTXOs and CREATES new UTXOs
// - Once a UTXO is spent, it's removed from the "UTXO set"
// - Your balance = sum of all UTXOs you can spend

use std::collections::HashMap;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Represents a unique identifier for a transaction output
/// In real Bitcoin: This would be (transaction_hash, output_index)
/// For simplicity: We use a string like "tx1:0" meaning "transaction tx1, output 0"
type UtxoId = String;

/// Represents an address that can own UTXOs
/// In real Bitcoin: This would be a public key hash (e.g., "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa")
/// For simplicity: We use plain strings like "Alice"
type Address = String;

/// Represents a UTXO (Unspent Transaction Output)
/// This is a "coin" that exists in the blockchain and can be spent
#[derive(Debug, Clone)]
struct Utxo {
    /// The address that owns this UTXO (who can spend it)
    owner: Address,

    /// The amount of cryptocurrency in this UTXO
    /// In Bitcoin, this would be in satoshis (1 BTC = 100,000,000 satoshis)
    amount: u64,

    /// In real blockchains, there would also be a "locking script" here
    /// (Bitcoin Script, Plutus, etc.) that defines spending conditions
    /// For simplicity, we just check ownership by address
}

impl Utxo {
    /// Creates a new UTXO
    fn new(owner: Address, amount: u64) -> Self {
        Utxo { owner, amount }
    }
}

/// Represents a transaction input - a reference to a UTXO being spent
#[derive(Debug, Clone)]
struct TxInput {
    /// The ID of the UTXO being spent (which "coin" are we consuming?)
    utxo_id: UtxoId,

    /// The address spending this UTXO (in real blockchain, this would be a signature)
    /// We need to verify that this matches the UTXO's owner
    spender: Address,
}

impl TxInput {
    fn new(utxo_id: UtxoId, spender: Address) -> Self {
        TxInput { utxo_id, spender }
    }
}

/// Represents a transaction output - a new UTXO being created
#[derive(Debug, Clone)]
struct TxOutput {
    /// Who will own this new UTXO?
    recipient: Address,

    /// How much cryptocurrency in this new UTXO?
    amount: u64,
}

impl TxOutput {
    fn new(recipient: Address, amount: u64) -> Self {
        TxOutput { recipient, amount }
    }
}

/// Represents a complete transaction
/// A transaction CONSUMES inputs (old UTXOs) and CREATES outputs (new UTXOs)
#[derive(Debug, Clone)]
struct Transaction {
    /// Unique identifier for this transaction
    id: String,

    /// List of UTXOs being spent (consumed)
    inputs: Vec<TxInput>,

    /// List of new UTXOs being created
    outputs: Vec<TxOutput>,
}

impl Transaction {
    fn new(id: String, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Transaction { id, inputs, outputs }
    }
}

// ============================================================================
// UTXO SET MANAGEMENT
// ============================================================================

/// The UTXO set is the collection of ALL unspent transaction outputs
/// This is what blockchain full nodes maintain in memory/disk
///
/// Key Properties:
/// - If a UTXO is in this set, it CAN be spent
/// - If a UTXO is NOT in this set, it either:
///   1. Doesn't exist, OR
///   2. Has already been spent (double-spend prevention!)
///
/// HashMap choice: O(1) lookups are critical for blockchain performance
/// Bitcoin's UTXO set has MILLIONS of entries!
type UtxoSet = HashMap<UtxoId, Utxo>;

/// Validates and applies a transaction to the UTXO set
///
/// This is the CORE of the UTXO model! This function:
/// 1. Checks that all inputs exist in the UTXO set
/// 2. Checks that the spender owns the UTXOs
/// 3. Checks that input sum >= output sum (no money creation!)
/// 4. REMOVES spent UTXOs from the set
/// 5. ADDS new UTXOs to the set
///
/// Returns: Result<(), String> - Ok if valid, Err with reason if invalid
fn apply_transaction(utxo_set: &mut UtxoSet, tx: &Transaction) -> Result<(), String> {
    println!("\n--- Validating Transaction: {} ---", tx.id);

    // STEP 1: Validate all inputs exist and calculate total input amount
    // This prevents double-spending and ensures UTXOs exist
    let mut total_input: u64 = 0;

    for input in &tx.inputs {
        println!("  Checking input: {}", input.utxo_id);

        // Does this UTXO exist in our set?
        // If not, either it never existed or it was already spent!
        let utxo = utxo_set.get(&input.utxo_id)
            .ok_or(format!("UTXO {} not found (already spent or invalid)", input.utxo_id))?;

        // OWNERSHIP CHECK: Does the spender actually own this UTXO?
        // In real blockchain, this would be signature verification
        if utxo.owner != input.spender {
            return Err(format!(
                "Ownership violation: {} tried to spend UTXO owned by {}",
                input.spender, utxo.owner
            ));
        }

        println!("    ✓ Valid UTXO: {} owns {} units", utxo.owner, utxo.amount);
        total_input += utxo.amount;
    }

    // STEP 2: Calculate total output amount
    let mut total_output: u64 = 0;
    for output in &tx.outputs {
        total_output += output.amount;
    }

    println!("  Total input:  {}", total_input);
    println!("  Total output: {}", total_output);

    // STEP 3: Conservation of value check
    // You can't create money out of thin air!
    // In real blockchains: total_input - total_output = transaction fee
    if total_input < total_output {
        return Err(format!(
            "Invalid transaction: outputs ({}) exceed inputs ({})",
            total_output, total_input
        ));
    }

    // The difference is the transaction fee (goes to miners)
    let fee = total_input - total_output;
    if fee > 0 {
        println!("  Transaction fee: {}", fee);
    }

    // STEP 4: CRITICAL - Remove spent UTXOs from the set
    // This is what PREVENTS DOUBLE-SPENDING!
    // Once removed, these UTXOs can never be spent again
    println!("  Removing spent UTXOs...");
    for input in &tx.inputs {
        utxo_set.remove(&input.utxo_id);
        println!("    ✗ Removed: {}", input.utxo_id);
    }

    // STEP 5: Add new UTXOs to the set
    // These are now available to be spent in future transactions
    println!("  Creating new UTXOs...");
    for (index, output) in tx.outputs.iter().enumerate() {
        // Create a unique ID for this new UTXO
        // Format: "transaction_id:output_index"
        let utxo_id = format!("{}:{}", tx.id, index);

        let utxo = Utxo::new(output.recipient.clone(), output.amount);
        utxo_set.insert(utxo_id.clone(), utxo);

        println!("    ✓ Created: {} -> {} ({})", utxo_id, output.recipient, output.amount);
    }

    println!("  Transaction {} applied successfully!", tx.id);
    Ok(())
}

/// Calculate the balance of an address by summing all UTXOs they own
///
/// This is how wallets calculate "your balance"!
/// In the UTXO model, there's no single "account balance" variable.
/// Instead, your balance is the SUM of all UTXOs you control.
fn get_balance(utxo_set: &UtxoSet, address: &Address) -> u64 {
    utxo_set
        .values()
        .filter(|utxo| &utxo.owner == address)
        .map(|utxo| utxo.amount)
        .sum()
}

/// Get all UTXOs owned by an address
/// Useful for wallet software to show which "coins" you have
fn get_utxos_for_address(utxo_set: &UtxoSet, address: &Address) -> Vec<(UtxoId, Utxo)> {
    utxo_set
        .iter()
        .filter(|(_, utxo)| &utxo.owner == address)
        .map(|(id, utxo)| (id.clone(), utxo.clone()))
        .collect()
}

/// Print the current state of the UTXO set
fn print_utxo_set(utxo_set: &UtxoSet) {
    println!("\n=== Current UTXO Set ===");
    if utxo_set.is_empty() {
        println!("  (empty)");
    } else {
        for (utxo_id, utxo) in utxo_set {
            println!("  {} -> {} owns {}", utxo_id, utxo.owner, utxo.amount);
        }
    }
    println!("  Total UTXOs: {}", utxo_set.len());
}

/// Print balances for all addresses
fn print_balances(utxo_set: &UtxoSet, addresses: &[Address]) {
    println!("\n=== Account Balances ===");
    for address in addresses {
        let balance = get_balance(utxo_set, address);
        println!("  {}: {}", address, balance);
    }
}

// ============================================================================
// MAIN DEMONSTRATION
// ============================================================================

fn main() {
    println!("=== UTXO Model Implementation ===\n");
    println!("This demonstrates how Bitcoin-style UTXO transactions work.");
    println!("We'll create UTXOs, spend them, and see how double-spending is prevented.\n");

    // Initialize an empty UTXO set
    // In a real blockchain, this would be loaded from disk
    let mut utxo_set: UtxoSet = HashMap::new();

    // ========================================================================
    // SCENARIO 1: Genesis - Create initial UTXOs (like mining)
    // ========================================================================
    println!("--- SCENARIO 1: Genesis/Coinbase Transactions ---");
    println!("Creating initial UTXOs (like mining rewards)...");

    // In Bitcoin, the first transaction in each block is a "coinbase" transaction
    // that creates new coins out of thin air (block reward + fees)
    // Only coinbase transactions can create value without inputs!

    // Create genesis UTXO for Alice (like a mining reward)
    let genesis_utxo_1 = Utxo::new("Alice".to_string(), 100);
    utxo_set.insert("genesis:0".to_string(), genesis_utxo_1);

    // Create another genesis UTXO for Bob
    let genesis_utxo_2 = Utxo::new("Bob".to_string(), 50);
    utxo_set.insert("genesis:1".to_string(), genesis_utxo_2);

    print_utxo_set(&utxo_set);
    print_balances(&utxo_set, &["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()]);

    // ========================================================================
    // SCENARIO 2: Simple Transfer - Alice sends 30 to Charlie
    // ========================================================================
    println!("\n--- SCENARIO 2: Simple Transfer ---");
    println!("Alice wants to send 30 to Charlie");
    println!("Alice has a UTXO of 100, so she needs to create:");
    println!("  - Output 1: 30 to Charlie (the payment)");
    println!("  - Output 2: 70 back to herself (the CHANGE)");
    println!("This is how UTXO transactions work - you must spend the ENTIRE input!");

    let tx1 = Transaction::new(
        "tx1".to_string(),
        vec![
            // Alice spends her 100 UTXO
            TxInput::new("genesis:0".to_string(), "Alice".to_string()),
        ],
        vec![
            // 30 goes to Charlie
            TxOutput::new("Charlie".to_string(), 30),
            // 70 goes back to Alice as "change"
            TxOutput::new("Alice".to_string(), 70),
        ],
    );

    apply_transaction(&mut utxo_set, &tx1).expect("Transaction should be valid");

    print_utxo_set(&utxo_set);
    print_balances(&utxo_set, &["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()]);

    // ========================================================================
    // SCENARIO 3: Multiple Inputs - Charlie combines UTXOs
    // ========================================================================
    println!("\n--- SCENARIO 3: Transaction with Multiple Inputs ---");
    println!("Bob sends 20 to Charlie");

    let tx2 = Transaction::new(
        "tx2".to_string(),
        vec![
            TxInput::new("genesis:1".to_string(), "Bob".to_string()),
        ],
        vec![
            TxOutput::new("Charlie".to_string(), 20),
            TxOutput::new("Bob".to_string(), 30),  // Bob's change
        ],
    );

    apply_transaction(&mut utxo_set, &tx2).expect("Transaction should be valid");

    print_utxo_set(&utxo_set);
    print_balances(&utxo_set, &["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()]);

    println!("\nNow Charlie has TWO UTXOs (30 from Alice, 20 from Bob)");
    println!("To spend MORE than either UTXO, Charlie must use BOTH as inputs:");

    let tx3 = Transaction::new(
        "tx3".to_string(),
        vec![
            // Charlie spends both of his UTXOs
            TxInput::new("tx1:0".to_string(), "Charlie".to_string()),  // 30
            TxInput::new("tx2:0".to_string(), "Charlie".to_string()),  // 20
        ],
        vec![
            // Send 45 to Alice
            TxOutput::new("Alice".to_string(), 45),
            // 5 back to Charlie as change
            TxOutput::new("Charlie".to_string(), 5),
        ],
    );

    apply_transaction(&mut utxo_set, &tx3).expect("Transaction should be valid");

    print_utxo_set(&utxo_set);
    print_balances(&utxo_set, &["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()]);

    // ========================================================================
    // SCENARIO 4: Double-Spend Prevention
    // ========================================================================
    println!("\n--- SCENARIO 4: Double-Spend Prevention ---");
    println!("What happens if someone tries to spend the same UTXO twice?");
    println!("Let's try to have Alice spend tx1:1 (which was already spent in tx3)...");

    let double_spend_tx = Transaction::new(
        "double_spend".to_string(),
        vec![
            // Try to spend a UTXO that no longer exists (was removed from set)
            TxInput::new("tx1:0".to_string(), "Charlie".to_string()),
        ],
        vec![
            TxOutput::new("Charlie".to_string(), 30),
        ],
    );

    match apply_transaction(&mut utxo_set, &double_spend_tx) {
        Ok(_) => println!("  ❌ ERROR: Double-spend succeeded (this should NOT happen!)"),
        Err(e) => println!("  ✓ Double-spend prevented! Error: {}", e),
    }

    // ========================================================================
    // SCENARIO 5: Invalid Transaction - Insufficient Funds
    // ========================================================================
    println!("\n--- SCENARIO 5: Insufficient Funds ---");
    println!("What if someone tries to create more output than input?");

    let invalid_tx = Transaction::new(
        "invalid".to_string(),
        vec![
            TxInput::new("tx2:1".to_string(), "Bob".to_string()),  // Bob has 30
        ],
        vec![
            TxOutput::new("Alice".to_string(), 50),  // Trying to send 50!
        ],
    );

    match apply_transaction(&mut utxo_set, &invalid_tx) {
        Ok(_) => println!("  ❌ ERROR: Money creation succeeded (this should NOT happen!)"),
        Err(e) => println!("  ✓ Invalid transaction rejected! Error: {}", e),
    }

    // ========================================================================
    // SCENARIO 6: Ownership Violation
    // ========================================================================
    println!("\n--- SCENARIO 6: Ownership Violation ---");
    println!("What if someone tries to spend someone else's UTXO?");

    let theft_tx = Transaction::new(
        "theft".to_string(),
        vec![
            // Charlie tries to spend Bob's UTXO!
            TxInput::new("tx2:1".to_string(), "Charlie".to_string()),
        ],
        vec![
            TxOutput::new("Charlie".to_string(), 30),
        ],
    );

    match apply_transaction(&mut utxo_set, &theft_tx) {
        Ok(_) => println!("  ❌ ERROR: Theft succeeded (this should NOT happen!)"),
        Err(e) => println!("  ✓ Ownership violation prevented! Error: {}", e),
    }

    // ========================================================================
    // FINAL STATE
    // ========================================================================
    println!("\n--- Final State ---");
    print_utxo_set(&utxo_set);
    print_balances(&utxo_set, &["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()]);

    // ========================================================================
    // DEMONSTRATING BALANCE CALCULATION
    // ========================================================================
    println!("\n--- How Wallets Calculate Balance ---");
    println!("In the UTXO model, your balance is NOT stored anywhere directly.");
    println!("Instead, it's the SUM of all UTXOs you own.\n");

    for address in &["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()] {
        let utxos = get_utxos_for_address(&utxo_set, address);
        println!("{}'s UTXOs:", address);

        if utxos.is_empty() {
            println!("  (none)");
        } else {
            for (utxo_id, utxo) in &utxos {
                println!("  {} -> {}", utxo_id, utxo.amount);
            }
        }

        let balance = get_balance(&utxo_set, address);
        println!("  Total balance: {}", balance);
        println!();
    }

    println!("=== Program Complete ===");
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. UTXO model tracks individual "coins", not account balances
// 2. Transactions CONSUME inputs (remove from UTXO set) and CREATE outputs (add to set)
// 3. Must spend ENTIRE UTXO - extra goes back to yourself as "change"
// 4. Double-spending prevented by removing spent UTXOs from the set
// 5. Your balance = sum of all UTXOs you own
// 6. HashMap provides O(1) lookups - critical for performance
// 7. All validation happens BEFORE modifying the UTXO set
// 8. Conservation of value: sum(inputs) >= sum(outputs)
// 9. Rust's ownership model maps naturally to UTXO ownership
// 10. In production: add signatures, scripts, persistence, and indexing

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Forgetting to remove spent UTXOs (allows double-spending!)
// ❌ Not checking input sum >= output sum (creates money from nothing!)
// ❌ Not validating UTXO ownership before spending
// ❌ Trying to partially spend a UTXO (must spend all, send change back)
// ❌ Not handling the case where a UTXO doesn't exist
// ❌ Forgetting that balance is calculated, not stored
// ❌ Not understanding that each output becomes a NEW UTXO with a unique ID
// ❌ Trying to modify a UTXO instead of consuming and creating new ones

// ============================================================================
// COMPARISON WITH ACCOUNT-BASED MODEL
// ============================================================================
// UTXO Model (Bitcoin):
//   - Track individual coins (UTXOs)
//   - Transactions consume inputs, create outputs
//   - Balance = sum of your UTXOs
//   - Parallel processing (different UTXOs independent)
//   - Better privacy (new address for each tx)
//   - More complex to understand
//
// Account Model (Ethereum):
//   - Track account balances directly
//   - Transactions modify balances: sender -= amount, receiver += amount
//   - Balance stored explicitly
//   - Sequential processing (nonce prevents replay)
//   - Simpler mental model
//   - Less privacy (balance tied to address)
//
// Both prevent double-spending, just differently!
// UTXO: Remove spent coins from set
// Account: Increment nonce, check balance before deducting
