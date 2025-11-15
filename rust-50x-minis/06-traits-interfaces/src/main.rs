// Project 06: Traits and Polymorphism
//
// Traits define shared behavior. They're like interfaces but more powerful.

use std::fmt;

fn main() {
    println!("=== Traits and Polymorphism ===\n");

    // ============================================================================
    // DEFINING AND IMPLEMENTING TRAITS
    // ============================================================================

    // Define a trait (like an interface)
    trait Describe {
        fn describe(&self) -> String;
    }

    struct Person {
        name: String,
        age: u32,
    }

    struct Car {
        brand: String,
        year: u32,
    }

    // Implement the trait for Person
    impl Describe for Person {
        fn describe(&self) -> String {
            format!("{} is {} years old", self.name, self.age)
        }
    }

    // Implement the trait for Car
    impl Describe for Car {
        fn describe(&self) -> String {
            format!("{} car from {}", self.brand, self.year)
        }
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let car = Car {
        brand: String::from("Toyota"),
        year: 2020,
    };

    println!("{}", person.describe());
    println!("{}", car.describe());

    println!();

    // ============================================================================
    // TRAIT BOUNDS
    // ============================================================================

    // Function that accepts any type implementing Describe
    fn print_description<T: Describe>(item: T) {
        println!("Description: {}", item.describe());
    }

    print_description(person);
    print_description(car);

    println!();

    // ============================================================================
    // DEFAULT IMPLEMENTATIONS
    // ============================================================================

    trait Summary {
        // Method without default (must be implemented)
        fn summarize_author(&self) -> String;

        // Method with default implementation
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    struct Article {
        author: String,
        content: String,
    }

    impl Summary for Article {
        fn summarize_author(&self) -> String {
            self.author.clone()
        }
        // Can use default summarize() or override it
    }

    let article = Article {
        author: String::from("Bob"),
        content: String::from("Rust is awesome!"),
    };

    println!("{}", article.summarize());

    println!();

    // ============================================================================
    // OPERATOR OVERLOADING
    // ============================================================================

    use std::ops::Add;

    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    // Implement Add trait to use + operator
    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;  // Uses our Add implementation!

    println!("p1 + p2 = {:?}", p3);

    println!();

    // ============================================================================
    // COMMON DERIVED TRAITS
    // ============================================================================

    #[derive(Debug, Clone, PartialEq)]
    struct Book {
        title: String,
        pages: u32,
    }

    let book1 = Book {
        title: String::from("Rust Book"),
        pages: 500,
    };

    let book2 = book1.clone();  // Clone trait
    println!("Books equal? {}", book1 == book2);  // PartialEq trait
    println!("Book: {:?}", book1);  // Debug trait

    println!();

    // ============================================================================
    // TRAIT OBJECTS (DYNAMIC DISPATCH)
    // ============================================================================

    trait Animal {
        fn make_sound(&self) -> String;
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn make_sound(&self) -> String {
            String::from("Woof!")
        }
    }

    impl Animal for Cat {
        fn make_sound(&self) -> String {
            String::from("Meow!")
        }
    }

    // Vector of trait objects (different types, same trait)
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for animal in animals.iter() {
        println!("{}", animal.make_sound());
    }

    println!();
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Traits define shared behavior (like interfaces)
// 2. impl Trait for Type adds behavior to types
// 3. Trait bounds enable generic programming
// 4. Default implementations provide fallback behavior
// 5. Derive macros auto-implement common traits
// 6. Operator overloading uses traits (Add, Sub, etc.)
// 7. dyn Trait enables runtime polymorphism
// 8. Traits are Rust's primary abstraction mechanism
