# Project 24: GUI with egui

## Overview
Build a desktop GUI application using egui (Embedded GUI), a modern immediate-mode GUI framework for Rust. Learn event handling, state management, and UI development patterns.

## Concepts Taught
- **Immediate mode GUI**: UI is rebuilt every frame
- **Event handling**: Mouse clicks, keyboard input, window events
- **State management**: Persisting application state between frames
- **Layout system**: Organizing UI elements with containers
- **Widgets**: Buttons, text inputs, sliders, labels
- **Custom drawing**: Using the painter API
- **Application loop**: Running the event loop
- **Cross-platform**: Works on Windows, macOS, Linux

## Why egui?

### Immediate Mode vs Retained Mode

**Immediate Mode (egui):**
- UI rebuilt every frame from application state
- Simple mental model: describe what you want right now
- No separate UI state to manage
- Easy to reason about

**Retained Mode (GTK, Qt):**
- UI elements persist as objects
- More complex state management
- Better for very complex UIs
- More traditional approach

### Comparison with Other Frameworks

| Feature | egui | imgui | GTK | Qt |
|---------|------|-------|-----|-----|
| Mode | Immediate | Immediate | Retained | Retained |
| Language | Rust | C++ | C | C++ |
| Cross-platform | ✓ | ✓ | ✓ | ✓ |
| Learning curve | Easy | Easy | Medium | Hard |
| Performance | Fast | Fast | Medium | Fast |
| Native look | ✗ | ✗ | ✓ | ✓ |

## Running This Project

```bash
cd 24-gui-egui
cargo run
```

**Note**: Requires graphics drivers. May need to install OpenGL/Vulkan dependencies on Linux.

## Application Features

This project implements a simple notepad/counter app demonstrating:
1. Text input and editing
2. Button interactions
3. State persistence
4. Multiple windows
5. Custom styling
6. Menu bar

## Performance Considerations

**Frame rate:**
- egui runs at 60 FPS by default
- Only redraws when needed (saves power)
- Very efficient for simple UIs

**Memory:**
- Minimal allocations (immediate mode)
- No UI tree to maintain
- State is your application state only

**Rendering:**
- Uses wgpu (WebGPU) backend by default
- Can use glow (OpenGL) for compatibility
- Hardware accelerated

## Real-World egui Applications

1. **Development tools**: Debuggers, profilers, editors
2. **Game tools**: Level editors, asset browsers
3. **Scientific visualization**: Data plotting, simulations
4. **System monitors**: Performance dashboards
5. **Configuration UIs**: Settings panels, config editors

## Beginner Pitfalls

### Pitfall 1: State Outside Frame Function
```rust
// ❌ State is lost between frames
fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let mut counter = 0;  // Reset every frame!
    if ui.button("Click").clicked() {
        counter += 1;
    }
}
```
**Fix**: Store state in the app struct.

### Pitfall 2: Not Using IDs for Stateful Widgets
```rust
// ❌ May have conflicts if you have multiple of the same widget
ui.text_edit_singleline(&mut self.text);
ui.text_edit_singleline(&mut self.text);  // Conflict!
```
**Fix**: Use `.id_source()` to give unique IDs.

### Pitfall 3: Heavy Computation in Update
```rust
// ❌ Runs every frame (60 FPS)!
fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let result = expensive_computation();  // Very bad!
}
```
**Fix**: Cache results, compute only when needed.

## Advanced Topics

1. **Custom widgets**: Creating reusable UI components
2. **Plotting**: Using egui_plot for charts and graphs
3. **File dialogs**: Native file pickers with rfd crate
4. **Async operations**: Running background tasks
5. **WebAssembly**: Compile to run in browser
6. **Custom styling**: Theming with Visuals

## Additional Challenges

1. **Todo list app**: Add, remove, complete tasks
2. **Calculator**: Build a functional calculator UI
3. **Image viewer**: Display and zoom images
4. **Settings panel**: Save/load configuration to file
5. **Chat interface**: Message list with input box

## Next Steps

- **Project 25**: Web server with Axum
- **Project 38**: CLI todo application
- **Project 41**: Web scraper with UI

## Expected Output

When you run the application, you'll see a window with:
- A text editor area
- Increment/decrement buttons
- A counter display
- Menu bar with File and Edit menus
- Reset button

The UI responds to mouse and keyboard input, and state persists as you interact with it.

## Dependencies

```toml
[dependencies]
eframe = "0.27"  # egui framework
egui = "0.27"    # GUI library
```

## Platform-Specific Notes

**Linux:**
```bash
# May need to install dependencies
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

**macOS:**
- Works out of the box
- Native window decorations

**Windows:**
- Works out of the box
- Compile with `cargo build --release` for faster startup
