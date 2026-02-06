// Project 04: Structs and Methods
//
// Structs let you create custom types that group related data.
// Methods let you add behavior to your types.

fn main() {
    println!("=== Structs and Methods ===\n");

    // ============================================================================
    // DEFINING AND USING STRUCTS
    // ============================================================================

    // Define a struct to represent a rectangle
    #[derive(Debug)]  // This lets us print the struct with {:?}
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // Create an instance
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1: {:?}", rect1);
    println!("Width: {}, Height: {}", rect1.width, rect1.height);

    // Structs can be mutable
    let mut rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    rect2.width = 15;  // Modify field
    println!("rect2: {:?}", rect2);

    println!();

    // ============================================================================
    // METHODS (Functions associated with structs)
    // ============================================================================

    // Methods are defined in an impl block
    impl Rectangle {
        // Method that borrows self immutably
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // Method that takes another Rectangle as parameter
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        // Mutable method (takes &mut self)
        fn scale(&mut self, factor: u32) {
            self.width *= factor;
            self.height *= factor;
        }

        // Associated function (doesn't take self) - like a static method
        // Often used as constructors
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area of rect3: {}", rect3.area());

    let rect4 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));

    // Associated function (called on the type, not an instance)
    let sq = Rectangle::square(25);
    println!("Square: {:?}", sq);
    println!("Square area: {}", sq.area());

    println!();

    // ============================================================================
    // THE THREE FORMS OF SELF
    // ============================================================================

    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // 1. &self - Borrow immutably (most common)
        fn distance_from_origin(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }

        // 2. &mut self - Borrow mutably (when you need to modify)
        fn translate(&mut self, dx: f64, dy: f64) {
            self.x += dx;
            self.y += dy;
        }

        // 3. self - Take ownership (consumes the value)
        fn into_tuple(self) -> (f64, f64) {
            (self.x, self.y)
        }
    }

    let p1 = Point { x: 3.0, y: 4.0 };
    println!("Distance: {}", p1.distance_from_origin());  // &self

    let mut p2 = Point { x: 1.0, y: 2.0 };
    p2.translate(5.0, 5.0);  // &mut self
    println!("After translate: {:?}", p2);

    let p3 = Point { x: 10.0, y: 20.0 };
    let tuple = p3.into_tuple();  // self (p3 is consumed!)
    println!("Tuple: {:?}", tuple);
    // println!("{:?}", p3);  // ❌ ERROR: p3 was moved

    println!();

    // ============================================================================
    // TUPLE STRUCTS
    // ============================================================================

    // Structs without named fields
    struct Color(u8, u8, u8);
    struct Point3D(f64, f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point3D(0.0, 0.0, 0.0);

    println!("Color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Point: ({}, {}, {})", origin.0, origin.1, origin.2);

    println!();

    // ============================================================================
    // UNIT-LIKE STRUCTS
    // ============================================================================

    // Structs with no fields (useful for traits)
    struct AlwaysEqual;

    let _subject = AlwaysEqual;

    println!();

    // ============================================================================
    // METHOD CHAINING
    // ============================================================================

    #[derive(Debug)]
    struct Counter {
        count: i32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }

        fn increment(&mut self) -> &mut Self {
            self.count += 1;
            self  // Return self for chaining
        }

        fn get(&self) -> i32 {
            self.count
        }
    }

    let mut counter = Counter::new();
    counter.increment().increment().increment();
    println!("Counter: {}", counter.get());

    println!();

    // ============================================================================
    // PRACTICAL EXAMPLE: BANK ACCOUNT
    // ============================================================================

    #[derive(Debug)]
    struct BankAccount {
        owner: String,
        balance: f64,
    }

    impl BankAccount {
        // Constructor (associated function)
        fn new(owner: String, initial_balance: f64) -> BankAccount {
            BankAccount {
                owner,
                balance: initial_balance,
            }
        }

        // Deposit money (&mut self)
        fn deposit(&mut self, amount: f64) {
            if amount > 0.0 {
                self.balance += amount;
                println!("{} deposited ${:.2}", self.owner, amount);
            }
        }

        // Withdraw money (&mut self)
        fn withdraw(&mut self, amount: f64) -> bool {
            if amount > 0.0 && amount <= self.balance {
                self.balance -= amount;
                println!("{} withdrew ${:.2}", self.owner, amount);
                true
            } else {
                println!("Insufficient funds for {}", self.owner);
                false
            }
        }

        // Check balance (&self)
        fn check_balance(&self) -> f64 {
            self.balance
        }

        // Transfer to another account
        fn transfer(&mut self, other: &mut BankAccount, amount: f64) -> bool {
            if self.withdraw(amount) {
                other.deposit(amount);
                true
            } else {
                false
            }
        }
    }

    let mut alice = BankAccount::new(String::from("Alice"), 1000.0);
    let mut bob = BankAccount::new(String::from("Bob"), 500.0);

    println!("Alice's balance: ${:.2}", alice.check_balance());
    println!("Bob's balance: ${:.2}", bob.check_balance());

    alice.deposit(200.0);
    alice.withdraw(150.0);

    alice.transfer(&mut bob, 300.0);

    println!("Final - Alice: ${:.2}", alice.check_balance());
    println!("Final - Bob: ${:.2}", bob.check_balance());
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Structs group related data into custom types
// 2. Methods are defined in `impl` blocks
// 3. &self borrows immutably, &mut self borrows mutably, self takes ownership
// 4. Associated functions don't take self (like static methods)
// 5. #[derive(Debug)] enables printing with {:?}
// 6. Tuple structs have unnamed fields
// 7. Method chaining returns &mut Self
// 8. Ownership rules apply to struct fields
