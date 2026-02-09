//! Integration tests for Lab 57: Traits & Interfaces (Alternative)
//!
//! Tests verify trait implementations, generic functions with trait bounds,
//! default methods, dynamic dispatch, and Display implementations.

use traits_interfaces_alternative::solution::*;

// ============================================================================
// DESCRIBE TRAIT TESTS
// ============================================================================

#[test]
fn test_person_describe() {
    let p = Person::new("Alice", 30);
    assert_eq!(p.describe(), "Alice is 30 years old");
}

#[test]
fn test_person_label() {
    let p = Person::new("Alice", 30);
    assert_eq!(p.label(), "person");
}

#[test]
fn test_car_describe() {
    let c = Car::new("Toyota", 2020);
    assert_eq!(c.describe(), "Toyota car from 2020");
}

#[test]
fn test_car_label() {
    let c = Car::new("Toyota", 2020);
    assert_eq!(c.label(), "car");
}

#[test]
fn test_book_describe() {
    let b = Book::new("Rust Book", 500);
    assert_eq!(b.describe(), "\"Rust Book\" (500 pages)");
}

#[test]
fn test_book_label() {
    let b = Book::new("Rust Book", 500);
    assert_eq!(b.label(), "book");
}

// ============================================================================
// GENERIC FUNCTION TESTS (STATIC DISPATCH)
// ============================================================================

#[test]
fn test_get_description_person() {
    let p = Person::new("Bob", 25);
    assert_eq!(get_description(&p), "Bob is 25 years old");
}

#[test]
fn test_get_description_car() {
    let c = Car::new("Honda", 2019);
    assert_eq!(get_description(&c), "Honda car from 2019");
}

#[test]
fn test_get_description_book() {
    let b = Book::new("Learning Rust", 300);
    assert_eq!(get_description(&b), "\"Learning Rust\" (300 pages)");
}

#[test]
fn test_labeled_description_person() {
    let p = Person::new("Alice", 30);
    assert_eq!(labeled_description(&p), "[person] Alice is 30 years old");
}

#[test]
fn test_labeled_description_car() {
    let c = Car::new("Toyota", 2020);
    assert_eq!(labeled_description(&c), "[car] Toyota car from 2020");
}

#[test]
fn test_labeled_description_book() {
    let b = Book::new("Rust Book", 500);
    assert_eq!(
        labeled_description(&b),
        "[book] \"Rust Book\" (500 pages)"
    );
}

// ============================================================================
// DYNAMIC DISPATCH TESTS
// ============================================================================

#[test]
fn test_describe_all_mixed_types() {
    let person = Person::new("Alice", 30);
    let car = Car::new("Toyota", 2020);
    let book = Book::new("Rust", 100);

    let items: Vec<&dyn Describe> = vec![&person, &car, &book];
    let descriptions = describe_all(&items);

    assert_eq!(descriptions.len(), 3);
    assert_eq!(descriptions[0], "Alice is 30 years old");
    assert_eq!(descriptions[1], "Toyota car from 2020");
    assert_eq!(descriptions[2], "\"Rust\" (100 pages)");
}

#[test]
fn test_describe_all_empty() {
    let items: Vec<&dyn Describe> = vec![];
    let descriptions = describe_all(&items);
    assert!(descriptions.is_empty());
}

#[test]
fn test_describe_all_single() {
    let person = Person::new("Charlie", 40);
    let items: Vec<&dyn Describe> = vec![&person];
    let descriptions = describe_all(&items);
    assert_eq!(descriptions.len(), 1);
    assert_eq!(descriptions[0], "Charlie is 40 years old");
}

// ============================================================================
// SUMMARY TRAIT TESTS (DEFAULT METHODS)
// ============================================================================

#[test]
fn test_article_summarize_author() {
    let article = Article::new("Bob", "Rust Traits", "Great content");
    assert_eq!(article.summarize_author(), "Bob");
}

#[test]
fn test_article_summarize_default() {
    let article = Article::new("Bob", "Rust Traits", "Great content");
    assert_eq!(article.summarize(), "(Read more from Bob...)");
}

#[test]
fn test_article_fields() {
    let article = Article::new("Alice", "Title", "Content");
    assert_eq!(article.author, "Alice");
    assert_eq!(article.title, "Title");
    assert_eq!(article.content, "Content");
}

// ============================================================================
// ANIMAL TRAIT TESTS (DYNAMIC DISPATCH)
// ============================================================================

#[test]
fn test_dog_make_sound() {
    let dog = Dog::new("Rex");
    assert_eq!(dog.make_sound(), "Woof!");
}

#[test]
fn test_dog_animal_type() {
    let dog = Dog::new("Rex");
    assert_eq!(dog.animal_type(), "dog");
}

#[test]
fn test_cat_make_sound() {
    let cat = Cat::new("Whiskers");
    assert_eq!(cat.make_sound(), "Meow!");
}

#[test]
fn test_cat_animal_type() {
    let cat = Cat::new("Whiskers");
    assert_eq!(cat.animal_type(), "cat");
}

#[test]
fn test_collect_sounds() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog::new("Rex")),
        Box::new(Cat::new("Whiskers")),
        Box::new(Dog::new("Buddy")),
    ];
    let sounds = collect_sounds(&animals);
    assert_eq!(sounds, vec!["Woof!", "Meow!", "Woof!"]);
}

#[test]
fn test_collect_sounds_empty() {
    let animals: Vec<Box<dyn Animal>> = vec![];
    let sounds = collect_sounds(&animals);
    assert!(sounds.is_empty());
}

#[test]
fn test_dog_name_field() {
    let dog = Dog::new("Fido");
    assert_eq!(dog.name, "Fido");
}

#[test]
fn test_cat_name_field() {
    let cat = Cat::new("Luna");
    assert_eq!(cat.name, "Luna");
}

// ============================================================================
// DISPLAY TRAIT TESTS
// ============================================================================

#[test]
fn test_person_display() {
    let p = Person::new("Alice", 30);
    let displayed = format!("{}", p);
    assert_eq!(displayed, "Alice (age 30)");
}

#[test]
fn test_car_display() {
    let c = Car::new("Toyota", 2020);
    let displayed = format!("{}", c);
    assert_eq!(displayed, "Toyota (2020)");
}

// ============================================================================
// STRUCT CONSTRUCTION & EQUALITY TESTS
// ============================================================================

#[test]
fn test_person_equality() {
    let p1 = Person::new("Alice", 30);
    let p2 = Person::new("Alice", 30);
    let p3 = Person::new("Bob", 30);
    assert_eq!(p1, p2);
    assert_ne!(p1, p3);
}

#[test]
fn test_car_equality() {
    let c1 = Car::new("Toyota", 2020);
    let c2 = Car::new("Toyota", 2020);
    let c3 = Car::new("Honda", 2020);
    assert_eq!(c1, c2);
    assert_ne!(c1, c3);
}

#[test]
fn test_book_equality() {
    let b1 = Book::new("Rust", 500);
    let b2 = Book::new("Rust", 500);
    let b3 = Book::new("Rust", 300);
    assert_eq!(b1, b2);
    assert_ne!(b1, b3);
}

#[test]
fn test_person_clone() {
    let p1 = Person::new("Alice", 30);
    let p2 = p1.clone();
    assert_eq!(p1, p2);
    // p1 is still valid after clone (unlike move)
    assert_eq!(p1.name, "Alice");
}

#[test]
fn test_car_clone() {
    let c1 = Car::new("Toyota", 2020);
    let c2 = c1.clone();
    assert_eq!(c1, c2);
}
