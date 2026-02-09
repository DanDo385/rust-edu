//! # GUI egui Model Demo

use gui_egui::solution::MyApp;

fn main() {
    println!("=== GUI Model Demo ===\n");

    let mut app = MyApp::new();
    app.increment();
    app.increment();
    app.decrement();
    app.set_slider_value(72.5);
    app.append_to_notes("Added from demo");

    println!("counter: {}", app.counter);
    println!("theme: {}", app.theme_name());
    println!("slider: {} ({:.2})", app.slider_value, app.slider_progress());
    println!("notes: {} chars, {} words", app.character_count(), app.word_count());
}
