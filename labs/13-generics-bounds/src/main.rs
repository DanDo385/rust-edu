// Project 07: Generics and Trait Bounds
//
// Generics let you write code that works with multiple types.
// Trait bounds constrain what types can be used.

fn main() {
    println!("=== Generics and Trait Bounds ===\n");

    // ============================================================================
    // GENERIC FUNCTIONS
    // ============================================================================

    // Without generics, we'd need separate functions:
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // With generics, one function works for any type!
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));

    println!();

    // ============================================================================
    // GENERIC STRUCTS
    // ============================================================================

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn new(x: T, y: T) -> Point<T> {
            Point { x, y }
        }
    }

    // Can also have multiple generic parameters
    #[derive(Debug)]
    struct Pair<T, U> {
        first: T,
        second: U,
    }

    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);

    println!("Int point: {:?}", int_point);
    println!("Float point: {:?}", float_point);

    let pair = Pair {
        first: 5,
        second: "hello",
    };
    println!("Pair: {:?}", pair);

    println!();

    // ============================================================================
    // TRAIT BOUNDS
    // ============================================================================

    use std::fmt::Display;

    // T must implement Display
    fn print_value<T: Display>(value: T) {
        println!("Value: {}", value);
    }

    print_value(42);
    print_value("hello");
    print_value(3.14);

    // Multiple trait bounds with +
    fn print_and_compare<T: Display + PartialOrd>(a: T, b: T) {
        println!("{} vs {}", a, b);
        if a > b {
            println!("First is larger");
        } else {
            println!("Second is larger or equal");
        }
    }

    print_and_compare(10, 20);
    print_and_compare("abc", "def");

    println!();

    // ============================================================================
    // WHERE CLAUSES
    // ============================================================================

    // For complex bounds, where clauses are more readable
    fn complex_function<T, U>(t: T, u: U) -> String
    where
        T: Display + Clone,
        U: Display + Clone + PartialEq,
    {
        format!("t: {}, u: {}", t, u)
    }

    println!("{}", complex_function(5, "hello"));

    println!();

    // ============================================================================
    // IMPLEMENTING METHODS FOR SPECIFIC TYPES
    // ============================================================================

    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point::new(3.0, 4.0);
    println!("Distance from origin: {}", p.distance_from_origin());

    println!();

    // ============================================================================
    // PRACTICAL EXAMPLE: GENERIC CONTAINER
    // ============================================================================

    #[derive(Debug)]
    struct Container<T> {
        value: T,
    }

    impl<T> Container<T> {
        fn new(value: T) -> Container<T> {
            Container { value }
        }

        fn get(&self) -> &T {
            &self.value
        }
    }

    impl<T: Display> Container<T> {
        fn display(&self) {
            println!("Container holds: {}", self.value);
        }
    }

    let int_container = Container::new(42);
    let string_container = Container::new(String::from("Rust"));

    int_container.display();
    string_container.display();

    println!();

    // ============================================================================
    // MONOMORPHIZATION EXAMPLE
    // ============================================================================

    // This generic function...
    fn double<T: std::ops::Add<Output = T> + Copy>(x: T) -> T {
        x + x
    }

    // ...gets compiled into separate functions:
    // fn double_i32(x: i32) -> i32 { x + x }
    // fn double_f64(x: f64) -> f64 { x + x }

    println!("Double 5: {}", double(5));
    println!("Double 2.5: {}", double(2.5));

    // ZERO RUNTIME COST! The compiler generates optimized code
    // for each type. This is called "monomorphization".

    println!();
}

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Generics enable code reuse across types
// 2. <T> declares a type parameter
// 3. Trait bounds constrain what T can be
// 4. Multiple bounds use + (T: Display + Clone)
// 5. where clauses improve readability
// 6. Monomorphization = compile-time specialization
// 7. Generics have ZERO runtime cost
// 8. Type safety enforced at compile time
