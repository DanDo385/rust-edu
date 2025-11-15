// Project 24: GUI with egui
//
// Demonstrates building a desktop GUI application using egui.
// egui is an immediate-mode GUI framework that's easy to use
// and produces clean, functional interfaces.

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Set up logging (optional but helpful for debugging)
    env_logger::init();

    // Configure the native window options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])  // Window size
            .with_min_inner_size([400.0, 300.0])  // Minimum size
            .with_icon(
                // Load app icon (optional)
                eframe::icon_data::from_png_bytes(&include_bytes!("../../../assets/icon-256.png")[..])
                    .unwrap_or_default(),
            ),
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Rust GUI Demo - egui",  // Window title
        options,
        Box::new(|cc| {
            // Configure egui style
            configure_style(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
}

// ============================================================================
// APPLICATION STATE
// ============================================================================

struct MyApp {
    // Counter example
    counter: i32,

    // Text editor example
    text: String,

    // UI state
    show_settings: bool,
    slider_value: f32,

    // Theme
    dark_mode: bool,

    // Multi-line text
    notes: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            counter: 0,
            text: String::from("Type something here..."),
            show_settings: false,
            slider_value: 50.0,
            dark_mode: true,
            notes: String::from("This is a simple notepad.\nYou can edit this text.\n\nTry the buttons below!"),
        }
    }
}

// ============================================================================
// APPLICATION IMPLEMENTATION
// ============================================================================

impl eframe::App for MyApp {
    /// Called each frame to update the UI
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.notes.clear();
                        ui.close_menu();
                    }
                    if ui.button("Settings").clicked() {
                        self.show_settings = !self.show_settings;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Clear").clicked() {
                        self.notes.clear();
                        ui.close_menu();
                    }
                    if ui.button("Reset Counter").clicked() {
                        self.counter = 0;
                        ui.close_menu();
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui.button("Toggle Theme").clicked() {
                        self.dark_mode = !self.dark_mode;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        ui.close_menu();
                    }
                });
            });
        });

        // Side panel for controls
        egui::SidePanel::left("controls_panel")
            .resizable(true)
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Controls");
                ui.separator();

                // Counter section
                ui.group(|ui| {
                    ui.label("Counter Example:");
                    ui.horizontal(|ui| {
                        if ui.button("➖ Decrement").clicked() {
                            self.counter -= 1;
                        }
                        if ui.button("➕ Increment").clicked() {
                            self.counter += 1;
                        }
                    });

                    ui.label(format!("Count: {}", self.counter));

                    if ui.button("Reset").clicked() {
                        self.counter = 0;
                    }
                });

                ui.add_space(10.0);

                // Slider section
                ui.group(|ui| {
                    ui.label("Slider Example:");
                    ui.add(egui::Slider::new(&mut self.slider_value, 0.0..=100.0).text("value"));

                    // Progress bar showing slider value
                    let progress = self.slider_value / 100.0;
                    ui.add(egui::ProgressBar::new(progress).show_percentage());
                });

                ui.add_space(10.0);

                // Text input section
                ui.group(|ui| {
                    ui.label("Text Input:");
                    ui.text_edit_singleline(&mut self.text);

                    if ui.button("Clear Input").clicked() {
                        self.text.clear();
                    }

                    ui.label(format!("Length: {} characters", self.text.len()));
                });

                ui.add_space(10.0);

                // Theme toggle
                ui.group(|ui| {
                    ui.label("Theme:");
                    ui.horizontal(|ui| {
                        if ui.selectable_label(self.dark_mode, "🌙 Dark").clicked() {
                            self.dark_mode = true;
                        }
                        if ui.selectable_label(!self.dark_mode, "☀ Light").clicked() {
                            self.dark_mode = false;
                        }
                    });
                });

                ui.add_space(10.0);

                // Info section
                ui.group(|ui| {
                    ui.label("App Info:");
                    ui.label(format!("FPS: {:.1}", ctx.input(|i| i.stable_dt).recip()));
                    ui.label(format!("Frame: {}", ctx.frame_nr()));
                });
            });

        // Bottom panel for status
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status: Ready");
                ui.separator();
                ui.label(format!("Characters: {}", self.notes.len()));
                ui.separator();
                ui.label(format!("Lines: {}", self.notes.lines().count()));
            });
        });

        // Central panel - main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Notepad");
            ui.separator();

            // Multi-line text editor
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.notes)
                        .desired_width(f32::INFINITY)
                        .desired_rows(20)
                        .font(egui::TextStyle::Monospace)
                );
            });

            ui.add_space(10.0);

            // Action buttons
            ui.horizontal(|ui| {
                if ui.button("📋 Clear All").clicked() {
                    self.notes.clear();
                }

                if ui.button("📝 Add Sample Text").clicked() {
                    self.notes.push_str("\n\nThis is some sample text added by clicking the button!");
                }

                if ui.button("🔢 Add Counter Value").clicked() {
                    self.notes.push_str(&format!("\nCounter: {}", self.counter));
                }
            });

            ui.add_space(10.0);

            // Custom painting example
            ui.heading("Custom Drawing");
            let (response, painter) = ui.allocate_painter(
                egui::vec2(ui.available_width(), 100.0),
                egui::Sense::hover(),
            );

            let rect = response.rect;
            let center = rect.center();

            // Draw background
            painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(40, 40, 60));

            // Draw circles based on counter value
            let num_circles = (self.counter.abs() % 10) + 1;
            for i in 0..num_circles {
                let angle = (i as f32 / num_circles as f32) * std::f32::consts::TAU;
                let radius = 30.0;
                let x = center.x + angle.cos() * radius;
                let y = center.y + angle.sin() * radius;
                let pos = egui::pos2(x, y);

                let color = egui::Color32::from_rgb(
                    (100 + i * 20) as u8,
                    (150 + i * 10) as u8,
                    (200 - i * 10) as u8,
                );

                painter.circle_filled(pos, 10.0, color);
            }

            // Draw text
            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                format!("Counter: {}", self.counter),
                egui::FontId::proportional(16.0),
                egui::Color32::WHITE,
            );
        });

        // Settings window (modal)
        if self.show_settings {
            egui::Window::new("Settings")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Application Settings");
                    ui.separator();

                    ui.checkbox(&mut self.dark_mode, "Dark mode");

                    ui.horizontal(|ui| {
                        ui.label("Slider value:");
                        ui.add(egui::DragValue::new(&mut self.slider_value).speed(0.1));
                    });

                    ui.separator();

                    if ui.button("Close").clicked() {
                        self.show_settings = false;
                    }
                });
        }
    }
}

// ============================================================================
// STYLING CONFIGURATION
// ============================================================================

fn configure_style(ctx: &egui::Context) {
    // Configure default style
    let mut style = (*ctx.style()).clone();

    // Larger text
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::proportional(24.0)),
        (egui::TextStyle::Body, egui::FontId::proportional(16.0)),
        (egui::TextStyle::Monospace, egui::FontId::monospace(14.0)),
        (egui::TextStyle::Button, egui::FontId::proportional(16.0)),
        (egui::TextStyle::Small, egui::FontId::proportional(12.0)),
    ]
    .into();

    // Spacing
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);

    ctx.set_style(style);
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. IMMEDIATE MODE GUI
//    The entire UI is recreated every frame (60 FPS).
//    State lives in MyApp struct, not in UI elements.
//    This makes it impossible to have stale UI state!
//
// 2. EVENT LOOP
//    eframe runs a platform-specific event loop:
//    - Windows: Win32 API
//    - macOS: Cocoa/AppKit
//    - Linux: X11/Wayland
//
// 3. RENDERING
//    egui generates rendering commands (triangles, colors).
//    eframe uses wgpu (WebGPU) or glow (OpenGL) to render.
//    All rendering is GPU-accelerated.
//
// 4. RETAINED STATE
//    egui internally tracks widget state (hovered, clicked, etc.)
//    using IDs derived from widget position and label.
//    You don't manage this - egui does it automatically.
//
// 5. LAYOUT
//    Layout is computed top-to-bottom, left-to-right.
//    Widgets request space, containers allocate it.
//    No separate layout pass needed (unlike HTML/CSS).

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Immediate mode GUI: rebuild UI every frame from state
// 2. Application state stored in app struct
// 3. update() called every frame (60 FPS)
// 4. Widgets return responses (clicked, hovered, etc.)
// 5. Layout with panels: TopBottom, Side, Central
// 6. Easy to reason about: state → UI (one direction)
// 7. Cross-platform: works on Windows, macOS, Linux
// 8. No callback hell: just check widget responses
// 9. GPU-accelerated rendering
// 10. Great for tools, not native-looking apps

// ============================================================================
// EGUI BEST PRACTICES
// ============================================================================
// ✅ DO:
// - Store state in app struct
// - Use unique IDs for duplicate widgets
// - Keep update() logic simple (no heavy computation)
// - Use scroll areas for dynamic content
// - Group related UI elements
// - Provide visual feedback for actions
//
// ❌ DON'T:
// - Do expensive computation in update() (runs every frame!)
// - Create state inside update() (lost between frames)
// - Assume native look and feel
// - Use for large, complex UIs (consider retained mode)
// - Forget to handle window close events
// - Mix business logic with UI code

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ State created inside update() (reset every frame)
// ❌ Heavy computation in update() (60 FPS!)
// ❌ Not using groups for visual organization
// ❌ Forgetting to handle button clicks (.clicked())
// ❌ Not using unique IDs for duplicate widgets
// ❌ Trying to make it look like native OS widgets
// ❌ Not using ScrollArea for long content
// ❌ Mixing UI and business logic
// ❌ Not testing on all platforms
// ❌ Forgetting to set minimum window size

// ============================================================================
// EXTENDING THIS PROJECT
// ============================================================================
// 1. Add file save/load functionality
// 2. Implement syntax highlighting
// 3. Add tabs for multiple documents
// 4. Create custom widgets
// 5. Add drag-and-drop support
// 6. Implement undo/redo
// 7. Add search and replace
// 8. Create a settings file (JSON/TOML)
// 9. Add keyboard shortcuts
// 10. Implement auto-save
