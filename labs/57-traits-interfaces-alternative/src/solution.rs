//! # Lab 57: Traits & Interfaces (Alternative)
//!
//! Alternative implementation demonstrating Rust's trait system: defining traits,
//! implementing them for multiple types, default methods, trait bounds, and
//! dynamic dispatch with trait objects.
//!
//! ## Ownership Commentary
//! - Traits define shared behavior across types (like interfaces in other languages)
//! - `impl Trait for Type` adds behavior without modifying the type's definition
//! - `dyn Trait` enables runtime polymorphism via vtable-based dispatch
//! - Generic functions with trait bounds use static dispatch (monomorphized)

use std::fmt;

// ============================================================================
// DESCRIBE TRAIT: Core trait for self-description
// ============================================================================

/// A trait for types that can describe themselves.
///
/// # Teaching Note
/// Traits are Rust's primary abstraction mechanism. Unlike inheritance in OOP,
/// traits define behavior that can be shared across unrelated types.
pub trait Describe {
    /// Returns a human-readable description of this value.
    fn describe(&self) -> String;

    /// Returns a short label for the type (default implementation).
    ///
    /// # Teaching Note
    /// Default methods provide fallback behavior. Implementors can override
    /// this or use the default. This is similar to default methods in Java interfaces.
    fn label(&self) -> &str {
        "unknown"
    }
}

// ============================================================================
// STRUCTS IMPLEMENTING DESCRIBE
// ============================================================================

/// A person with a name and age.
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    /// Creates a new Person.
    pub fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

impl Describe for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }

    fn label(&self) -> &str {
        "person"
    }
}

/// A car with a brand and year.
#[derive(Debug, Clone, PartialEq)]
pub struct Car {
    pub brand: String,
    pub year: u32,
}

impl Car {
    /// Creates a new Car.
    pub fn new(brand: &str, year: u32) -> Car {
        Car {
            brand: brand.to_string(),
            year,
        }
    }
}

impl Describe for Car {
    fn describe(&self) -> String {
        format!("{} car from {}", self.brand, self.year)
    }

    fn label(&self) -> &str {
        "car"
    }
}

/// A book with a title and page count.
#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub title: String,
    pub pages: u32,
}

impl Book {
    pub fn new(title: &str, pages: u32) -> Book {
        Book {
            title: title.to_string(),
            pages,
        }
    }
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("\"{}\" ({} pages)", self.title, self.pages)
    }

    fn label(&self) -> &str {
        "book"
    }
}

// ============================================================================
// GENERIC FUNCTION WITH TRAIT BOUNDS
// ============================================================================

/// Returns the description of any type implementing Describe.
///
/// # Memory Model
/// This function uses STATIC DISPATCH (monomorphization). The compiler generates
/// a separate version of this function for each concrete type it's called with.
/// This is zero-cost: no vtable lookup at runtime.
pub fn get_description<T: Describe>(item: &T) -> String {
    item.describe()
}

/// Returns a formatted string with label and description.
pub fn labeled_description<T: Describe>(item: &T) -> String {
    format!("[{}] {}", item.label(), item.describe())
}

/// Returns descriptions of all items in a slice using dynamic dispatch.
///
/// # Memory Model
/// `&[&dyn Describe]` uses DYNAMIC DISPATCH. Each element is a fat pointer
/// (data pointer + vtable pointer = 16 bytes). The vtable is looked up at
/// runtime to call the correct `describe()` implementation.
pub fn describe_all(items: &[&dyn Describe]) -> Vec<String> {
    items.iter().map(|item| item.describe()).collect()
}

// ============================================================================
// SUMMARY TRAIT: Default implementations
// ============================================================================

/// A trait with a required method and a default method.
pub trait Summary {
    /// Required: must be implemented by each type.
    fn summarize_author(&self) -> String;

    /// Default implementation that uses summarize_author().
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

/// An article that implements Summary.
#[derive(Debug, Clone)]
pub struct Article {
    pub author: String,
    pub title: String,
    pub content: String,
}

impl Article {
    pub fn new(author: &str, title: &str, content: &str) -> Article {
        Article {
            author: author.to_string(),
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    // Uses default summarize() implementation
}

// ============================================================================
// ANIMAL TRAIT: Dynamic dispatch example
// ============================================================================

/// A trait for animals that make sounds.
pub trait Animal {
    fn make_sound(&self) -> String;
    fn animal_type(&self) -> &str;
}

/// A dog that says "Woof!".
#[derive(Debug)]
pub struct Dog {
    pub name: String,
}

impl Dog {
    pub fn new(name: &str) -> Dog {
        Dog {
            name: name.to_string(),
        }
    }
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("Woof!")
    }

    fn animal_type(&self) -> &str {
        "dog"
    }
}

/// A cat that says "Meow!".
#[derive(Debug)]
pub struct Cat {
    pub name: String,
}

impl Cat {
    pub fn new(name: &str) -> Cat {
        Cat {
            name: name.to_string(),
        }
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        String::from("Meow!")
    }

    fn animal_type(&self) -> &str {
        "cat"
    }
}

/// Collects sounds from a slice of boxed animal trait objects.
pub fn collect_sounds(animals: &[Box<dyn Animal>]) -> Vec<String> {
    animals.iter().map(|a| a.make_sound()).collect()
}

// ============================================================================
// Display TRAIT IMPLEMENTATION
// ============================================================================

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}

impl fmt::Display for Car {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.brand, self.year)
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_describe() {
        let p = Person::new("Alice", 30);
        assert_eq!(p.describe(), "Alice is 30 years old");
    }

    #[test]
    fn test_car_describe() {
        let c = Car::new("Toyota", 2020);
        assert_eq!(c.describe(), "Toyota car from 2020");
    }

    #[test]
    fn test_get_description_generic() {
        let p = Person::new("Bob", 25);
        assert_eq!(get_description(&p), "Bob is 25 years old");
    }
}
