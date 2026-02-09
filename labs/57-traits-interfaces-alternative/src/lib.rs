//! # Lab 57: Traits & Interfaces (Alternative) - Student API
//!
//! Implement trait-driven polymorphism below.
//! See `src/solution.rs` for reference.

use std::fmt;

pub trait Describe {
    fn describe(&self) -> String;

    fn label(&self) -> &str {
        todo!("Provide default label")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(_name: &str, _age: u32) -> Person {
        todo!("Construct person")
    }
}

impl Describe for Person {
    fn describe(&self) -> String {
        let _ = self;
        todo!("Describe person")
    }

    fn label(&self) -> &str {
        todo!("Person label")
    }
}

impl fmt::Display for Person {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Display person")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Car {
    pub brand: String,
    pub year: u32,
}

impl Car {
    pub fn new(_brand: &str, _year: u32) -> Car {
        todo!("Construct car")
    }
}

impl Describe for Car {
    fn describe(&self) -> String {
        let _ = self;
        todo!("Describe car")
    }

    fn label(&self) -> &str {
        todo!("Car label")
    }
}

impl fmt::Display for Car {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Display car")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub title: String,
    pub pages: u32,
}

impl Book {
    pub fn new(_title: &str, _pages: u32) -> Book {
        todo!("Construct book")
    }
}

impl Describe for Book {
    fn describe(&self) -> String {
        let _ = self;
        todo!("Describe book")
    }

    fn label(&self) -> &str {
        todo!("Book label")
    }
}

pub fn get_description<T: Describe>(_item: &T) -> String {
    todo!("Call Describe::describe")
}

pub fn labeled_description<T: Describe>(_item: &T) -> String {
    todo!("Compose label + description")
}

pub fn describe_all(_items: &[&dyn Describe]) -> Vec<String> {
    todo!("Describe heterogeneous values via trait objects")
}

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        todo!("Default author summary")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Article {
    pub author: String,
    pub title: String,
    pub content: String,
}

impl Article {
    pub fn new(_author: &str, _title: &str, _content: &str) -> Article {
        todo!("Construct article")
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        let _ = self;
        todo!("Summarize article")
    }

    fn summarize_author(&self) -> String {
        let _ = self;
        todo!("Summarize article author")
    }
}

pub trait Animal {
    fn make_sound(&self) -> String;
    fn animal_type(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dog {
    pub name: String,
}

impl Dog {
    pub fn new(_name: &str) -> Dog {
        todo!("Construct dog")
    }
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        let _ = self;
        todo!("Dog sound")
    }

    fn animal_type(&self) -> &str {
        todo!("Dog type")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cat {
    pub name: String,
}

impl Cat {
    pub fn new(_name: &str) -> Cat {
        todo!("Construct cat")
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        let _ = self;
        todo!("Cat sound")
    }

    fn animal_type(&self) -> &str {
        todo!("Cat type")
    }
}

pub fn collect_sounds(_animals: &[Box<dyn Animal>]) -> Vec<String> {
    todo!("Collect sounds from trait objects")
}

#[doc(hidden)]
pub mod solution;
