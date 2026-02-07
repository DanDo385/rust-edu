// Lab 27: GUI with egui - Data Model
//
// This module contains the application state and business logic for an
// egui-based GUI application. By separating the data model from the GUI
// rendering code (in main.rs), we achieve:
//
// 1. Testable business logic without a windowing system
// 2. Clear separation of concerns (state vs. presentation)
// 3. The ability to reuse this model with different frontends
//
// Key Concept: Immediate-mode GUIs rebuild the entire UI every frame
// from the application state. The state struct IS the single source of
// truth. The GUI just reads and mutates it.
//
// # Memory Model
// ```text
// Stack (MyApp):                     Heap:
// ┌────────────────────────┐        ┌──────────────────────────┐
// │ counter:  i32 (4 bytes)│        │ "Type something here..." │
// │ text:     ptr+len+cap  │───────>│ (24 bytes + capacity)    │
// │ show_settings: bool    │        └──────────────────────────┘
// │ slider_value:  f32     │        ┌──────────────────────────┐
// │ dark_mode:     bool    │        │ "This is a simple..."    │
// │ notes:    ptr+len+cap  │───────>│ (multi-line string)      │
// └────────────────────────┘        └──────────────────────────┘
// ```
// All String fields own heap-allocated UTF-8 data.
// Primitive fields (i32, f32, bool) live entirely on the stack.

// ============================================================================
// APPLICATION STATE
// ============================================================================

/// The core application state for the egui GUI demo.
///
/// In an immediate-mode GUI, this struct holds ALL mutable state.
/// The UI is a pure function of this state: State -> Pixels.
/// Every frame, the GUI reads these fields and renders accordingly.
///
/// This pattern (state struct + methods) is the recommended way to
/// architect egui applications. Business logic lives here in lib.rs,
/// and rendering lives in the `eframe::App::update()` impl in main.rs.
#[derive(Debug, Clone)]
pub struct MyApp {
    /// Counter value for the increment/decrement demo.
    pub counter: i32,

    /// Single-line text input field.
    pub text: String,

    /// Whether the settings window/panel is visible.
    pub show_settings: bool,

    /// Slider value (0.0 to 100.0) for the slider demo.
    pub slider_value: f32,

    /// Whether dark mode is active (true = dark, false = light).
    pub dark_mode: bool,

    /// Multi-line notepad content.
    pub notes: String,
}

impl Default for MyApp {
    /// Creates a new MyApp with sensible default values.
    ///
    /// This is called once when the application starts. The Default trait
    /// is used by eframe to initialize the app state.
    fn default() -> Self {
        Self {
            counter: 0,
            text: String::from("Type something here..."),
            show_settings: false,
            slider_value: 50.0,
            dark_mode: true,
            notes: String::from(
                "This is a simple notepad.\nYou can edit this text.\n\nTry the buttons below!",
            ),
        }
    }
}

// ============================================================================
// COUNTER OPERATIONS
// ============================================================================

impl MyApp {
    /// Creates a new MyApp with default values.
    ///
    /// Equivalent to `MyApp::default()` but more explicit.
    pub fn new() -> Self {
        Self::default()
    }

    /// Increments the counter by 1.
    ///
    /// In the GUI, this is triggered by clicking the "+" button.
    /// We use wrapping addition to avoid panic on overflow.
    pub fn increment(&mut self) {
        self.counter = self.counter.wrapping_add(1);
    }

    /// Decrements the counter by 1.
    ///
    /// In the GUI, this is triggered by clicking the "-" button.
    /// We use wrapping subtraction to avoid panic on underflow.
    pub fn decrement(&mut self) {
        self.counter = self.counter.wrapping_sub(1);
    }

    /// Resets the counter to zero.
    pub fn reset_counter(&mut self) {
        self.counter = 0;
    }

    // ========================================================================
    // TEXT / NOTES OPERATIONS
    // ========================================================================

    /// Clears all notes content.
    ///
    /// This sets the notes string to empty without deallocating the
    /// underlying buffer. The capacity is preserved for efficiency --
    /// the user will likely type new content immediately.
    pub fn clear_notes(&mut self) {
        self.notes.clear();
    }

    /// Appends text to the notes, preceded by a newline if notes is non-empty.
    pub fn append_to_notes(&mut self, text: &str) {
        if !self.notes.is_empty() {
            self.notes.push('\n');
        }
        self.notes.push_str(text);
    }

    /// Returns the number of characters in the notes.
    ///
    /// This counts Unicode scalar values (chars), not bytes.
    /// For ASCII text, chars == bytes. For multi-byte UTF-8 text,
    /// chars < bytes.
    pub fn character_count(&self) -> usize {
        self.notes.chars().count()
    }

    /// Returns the number of lines in the notes.
    ///
    /// An empty string has 0 lines. A string with no newlines has 1 line.
    /// Each newline adds one more line.
    ///
    /// # Edge Cases
    /// - Empty string: 0 lines
    /// - "hello": 1 line
    /// - "hello\n": 1 line (trailing newline, str::lines() trims it)
    /// - "hello\nworld": 2 lines
    pub fn line_count(&self) -> usize {
        if self.notes.is_empty() {
            0
        } else {
            self.notes.lines().count()
        }
    }

    /// Returns the number of words in the notes.
    ///
    /// Words are defined as whitespace-separated tokens. This uses
    /// Rust's `split_whitespace()` which handles multiple spaces,
    /// tabs, and newlines correctly.
    pub fn word_count(&self) -> usize {
        self.notes.split_whitespace().count()
    }

    // ========================================================================
    // THEME OPERATIONS
    // ========================================================================

    /// Toggles between dark and light mode.
    ///
    /// In the GUI, this changes the egui Visuals applied each frame.
    /// Here in the model, we just flip the boolean.
    pub fn toggle_theme(&mut self) {
        self.dark_mode = !self.dark_mode;
    }

    /// Returns the current theme name as a string.
    pub fn theme_name(&self) -> &'static str {
        if self.dark_mode {
            "dark"
        } else {
            "light"
        }
    }

    // ========================================================================
    // SETTINGS OPERATIONS
    // ========================================================================

    /// Toggles the settings panel visibility.
    pub fn toggle_settings(&mut self) {
        self.show_settings = !self.show_settings;
    }

    // ========================================================================
    // SLIDER OPERATIONS
    // ========================================================================

    /// Sets the slider value, clamping to the valid range [0.0, 100.0].
    ///
    /// The f32::clamp method ensures the value stays within bounds,
    /// which prevents invalid state in the GUI progress bar.
    pub fn set_slider_value(&mut self, value: f32) {
        self.slider_value = value.clamp(0.0, 100.0);
    }

    /// Returns the slider value as a normalized progress (0.0 to 1.0).
    ///
    /// Useful for rendering progress bars where 0.0 = empty, 1.0 = full.
    pub fn slider_progress(&self) -> f32 {
        self.slider_value / 100.0
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. SEPARATION OF CONCERNS
//    The data model (this file) has NO dependency on eframe or egui.
//    It can be tested, serialized, or used with any frontend.
//    The GUI code in main.rs depends on this model + egui.
//
// 2. IMMEDIATE MODE PATTERN
//    State -> Render -> User Input -> Mutate State -> Repeat
//    The entire UI is rebuilt every frame (60 FPS).
//    No stale widgets, no widget-owns-state problems.
//
// 3. STRING OPERATIONS
//    String::clear() sets len to 0 but keeps capacity.
//    This is O(1) and avoids reallocation when the user types again.
//    lines() and split_whitespace() are lazy iterators.
//
// 4. WRAPPING ARITHMETIC
//    wrapping_add/sub prevent panic on overflow/underflow.
//    In debug builds, Rust panics on integer overflow by default.
//    wrapping_* makes the behavior explicit and safe.
//
// 5. CLAMP PATTERN
//    f32::clamp(min, max) is a single comparison + branch.
//    It guarantees the value is always valid for downstream use.

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Separate state (lib.rs) from rendering (main.rs) for testability
// 2. Use Default trait to define initial application state
// 3. Business logic methods on the state struct, not in UI callbacks
// 4. wrapping_add/sub for safe integer arithmetic
// 5. String::clear() preserves capacity (efficient for repeated use)
// 6. lines() and split_whitespace() are lazy iterators
// 7. f32::clamp() enforces value ranges
// 8. This model can be reused with CLI, TUI, web, or native GUI
