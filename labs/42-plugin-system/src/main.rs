// Project 39: Plugin System
//
// This program implements a plugin system using trait objects
// and dynamic dispatch. It demonstrates how to create extensible
// applications where functionality can be added at runtime.

use std::collections::HashMap;

fn main() {
    println!("=== Plugin System ===\n");

    // ============================================================================
    // WHAT IS A PLUGIN SYSTEM?
    // ============================================================================
    // A plugin system allows extending an application's functionality
    // without modifying its core code. Plugins implement a common interface
    // (trait) and can be loaded and executed dynamically.
    //
    // Key concepts:
    // - Trait objects (dyn Trait) for runtime polymorphism
    // - Box<dyn Trait> for heap-allocated trait objects
    // - Dynamic dispatch via vtables
    // - Plugin registry for managing plugins

    let mut registry = PluginRegistry::new();

    println!("=== Registering Plugins ===\n");

    // Register different plugins
    registry.register("logger", Box::new(LoggerPlugin::new("app.log")));
    registry.register("cache", Box::new(CachePlugin::new(100)));
    registry.register("analytics", Box::new(AnalyticsPlugin::new("UA-123")));
    registry.register("notification", Box::new(NotificationPlugin));

    println!();
    println!("=== Plugin Information ===\n");

    registry.list_plugins();

    println!();
    println!("=== Initializing All Plugins ===\n");

    registry.initialize_all();

    println!();
    println!("=== Processing Data with Plugins ===\n");

    let data = "User login event";
    println!("Processing: \"{}\"", data);
    println!();

    registry.process_all(data);

    println!();
    println!("=== Executing Specific Plugin ===\n");

    if let Some(plugin) = registry.get_plugin("analytics") {
        plugin.process(data);
    }

    println!();
    println!("=== Shutting Down Plugins ===\n");

    registry.shutdown_all();

    println!();

    // ============================================================================
    // DEMONSTRATING DATA FORMATTERS
    // ============================================================================

    println!("=== Data Formatter Plugins ===\n");

    let formatters: Vec<Box<dyn DataFormatter>> = vec![
        Box::new(JsonFormatter),
        Box::new(XmlFormatter),
        Box::new(CsvFormatter),
    ];

    let data = vec![
        ("name", "Alice"),
        ("age", "30"),
        ("city", "Seattle"),
    ];

    for formatter in formatters.iter() {
        println!("{}", formatter.format(&data));
    }

    println!();
}

// ============================================================================
// PLUGIN TRAIT
// ============================================================================

/// The main plugin trait that all plugins must implement
/// This is "object-safe" - it can be used as a trait object
trait Plugin {
    /// Get plugin name
    fn name(&self) -> &str;

    /// Get plugin version
    fn version(&self) -> &str {
        "1.0.0"
    }

    /// Initialize the plugin
    fn initialize(&mut self) {
        println!("  [{}] Initialized", self.name());
    }

    /// Process some data
    fn process(&self, data: &str);

    /// Shutdown the plugin
    fn shutdown(&self) {
        println!("  [{}] Shut down", self.name());
    }
}

// ============================================================================
// PLUGIN IMPLEMENTATIONS
// ============================================================================

/// Logger plugin - logs events to file
struct LoggerPlugin {
    filename: String,
    enabled: bool,
}

impl LoggerPlugin {
    fn new(filename: &str) -> Self {
        LoggerPlugin {
            filename: filename.to_string(),
            enabled: false,
        }
    }
}

impl Plugin for LoggerPlugin {
    fn name(&self) -> &str {
        "Logger"
    }

    fn version(&self) -> &str {
        "1.2.0"
    }

    fn initialize(&mut self) {
        self.enabled = true;
        println!("  [{}] Initialized - logging to {}", self.name(), self.filename);
    }

    fn process(&self, data: &str) {
        if self.enabled {
            println!("  [{}] Logging to {}: {}", self.name(), self.filename, data);
        }
    }
}

/// Cache plugin - caches data in memory
struct CachePlugin {
    max_size: usize,
    cache: HashMap<String, String>,
}

impl CachePlugin {
    fn new(max_size: usize) -> Self {
        CachePlugin {
            max_size,
            cache: HashMap::new(),
        }
    }
}

impl Plugin for CachePlugin {
    fn name(&self) -> &str {
        "Cache"
    }

    fn initialize(&mut self) {
        self.cache.clear();
        println!("  [{}] Initialized - max size: {} items", self.name(), self.max_size);
    }

    fn process(&self, data: &str) {
        println!("  [{}] Caching data: {} (current size: {})",
                 self.name(), data, self.cache.len());
    }
}

/// Analytics plugin - sends data to analytics service
struct AnalyticsPlugin {
    tracking_id: String,
}

impl AnalyticsPlugin {
    fn new(tracking_id: &str) -> Self {
        AnalyticsPlugin {
            tracking_id: tracking_id.to_string(),
        }
    }
}

impl Plugin for AnalyticsPlugin {
    fn name(&self) -> &str {
        "Analytics"
    }

    fn initialize(&mut self) {
        println!("  [{}] Initialized - tracking ID: {}", self.name(), self.tracking_id);
    }

    fn process(&self, data: &str) {
        println!("  [{}] Sending to analytics: {}", self.name(), data);
    }
}

/// Notification plugin - sends notifications
struct NotificationPlugin;

impl Plugin for NotificationPlugin {
    fn name(&self) -> &str {
        "Notification"
    }

    fn process(&self, data: &str) {
        println!("  [{}] Sending notification: {}", self.name(), data);
    }
}

// ============================================================================
// PLUGIN REGISTRY
// ============================================================================

/// Registry that manages all plugins
struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    fn new() -> Self {
        PluginRegistry {
            plugins: HashMap::new(),
        }
    }

    /// Register a plugin
    fn register(&mut self, id: &str, plugin: Box<dyn Plugin>) {
        println!("Registered plugin: {} v{}", plugin.name(), plugin.version());
        self.plugins.insert(id.to_string(), plugin);
    }

    /// Get a specific plugin
    fn get_plugin(&self, id: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins.get(id)
    }

    /// List all plugins
    fn list_plugins(&self) {
        println!("Loaded plugins:");
        for (id, plugin) in &self.plugins {
            println!("  {} - {} v{}", id, plugin.name(), plugin.version());
        }
    }

    /// Initialize all plugins
    fn initialize_all(&mut self) {
        for plugin in self.plugins.values_mut() {
            plugin.initialize();
        }
    }

    /// Process data with all plugins
    fn process_all(&self, data: &str) {
        for plugin in self.plugins.values() {
            plugin.process(data);
        }
    }

    /// Shutdown all plugins
    fn shutdown_all(&self) {
        for plugin in self.plugins.values() {
            plugin.shutdown();
        }
    }
}

// ============================================================================
// DATA FORMATTER TRAIT (ANOTHER EXAMPLE)
// ============================================================================

/// A different plugin trait for formatting data
trait DataFormatter {
    fn format(&self, data: &[(&str, &str)]) -> String;
    fn format_type(&self) -> &str;
}

struct JsonFormatter;

impl DataFormatter for JsonFormatter {
    fn format(&self, data: &[(&str, &str)]) -> String {
        let fields: Vec<String> = data
            .iter()
            .map(|(k, v)| format!("  \"{}\": \"{}\"", k, v))
            .collect();
        format!("JSON:\n{{\n{}\n}}", fields.join(",\n"))
    }

    fn format_type(&self) -> &str {
        "JSON"
    }
}

struct XmlFormatter;

impl DataFormatter for XmlFormatter {
    fn format(&self, data: &[(&str, &str)]) -> String {
        let fields: Vec<String> = data
            .iter()
            .map(|(k, v)| format!("  <{}>{}</{}>", k, v, k))
            .collect();
        format!("XML:\n<data>\n{}\n</data>", fields.join("\n"))
    }

    fn format_type(&self) -> &str {
        "XML"
    }
}

struct CsvFormatter;

impl DataFormatter for CsvFormatter {
    fn format(&self, data: &[(&str, &str)]) -> String {
        let headers: Vec<&str> = data.iter().map(|(k, _)| *k).collect();
        let values: Vec<&str> = data.iter().map(|(_, v)| *v).collect();
        format!("CSV:\n{}\n{}", headers.join(","), values.join(","))
    }

    fn format_type(&self) -> &str {
        "CSV"
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. TRAIT OBJECTS
//    dyn Plugin is a "trait object" - a reference to any type that implements Plugin.
//    It consists of TWO pointers (fat pointer):
//    - Data pointer: Points to the actual object
//    - Vtable pointer: Points to a table of function pointers
//
// 2. VTABLE (VIRTUAL METHOD TABLE)
//    When you call plugin.process(data), Rust:
//    a) Looks up the vtable pointer
//    b) Finds the process method in the vtable
//    c) Calls the function pointer
//    This is "dynamic dispatch" - the method is chosen at runtime.
//
// 3. BOX<DYN TRAIT>
//    Box allocates the trait object on the heap.
//    This is necessary because:
//    - Different plugins have different sizes
//    - We need a fixed-size type to store in Vec or HashMap
//    - The trait object itself is just two pointers (16 bytes on 64-bit)
//
// 4. OBJECT SAFETY
//    A trait is "object-safe" if it can be used as a trait object.
//    Rules:
//    - Methods can't have type parameters
//    - Methods can't return Self
//    - Trait can't have associated constants
//    This ensures the vtable can be constructed.
//
// 5. MEMORY LAYOUT
//    Vec<Box<dyn Plugin>>:
//    - Vec on stack: 24 bytes (ptr, len, cap)
//    - Each Box on heap: plugin size + overhead
//    - Each trait object: 16 bytes (fat pointer)
//    Total: O(n) where n = number of plugins
//
// 6. PERFORMANCE
//    Dynamic dispatch overhead:
//    - Vtable lookup: ~1-3 nanoseconds
//    - Cannot be inlined (compiler doesn't know the type)
//    - Still very fast - negligible for most use cases
//
//    Static dispatch (generics):
//    - No vtable lookup
//    - Can be inlined
//    - But increases code size (monomorphization)
//
// 7. WHEN TO USE EACH
//    Use trait objects (dyn Trait) when:
//    - Types are not known at compile time
//    - Need heterogeneous collections
//    - Implementing plugin systems, GUI frameworks
//
//    Use generics (<T: Trait>) when:
//    - Types are known at compile time
//    - Need maximum performance
//    - Can accept larger binary size

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Trait objects enable RUNTIME POLYMORPHISM
// 2. dyn Trait creates a fat pointer (data + vtable)
// 3. Box<dyn Trait> stores trait objects on heap
// 4. Dynamic dispatch has small overhead (~1-3ns per call)
// 5. Object safety rules ensure vtable can be constructed
// 6. Plugin systems benefit from trait objects
// 7. HashMap<String, Box<dyn Trait>> stores heterogeneous types
// 8. Trait objects cannot be cloned without special handling

// ============================================================================
// STATIC VS DYNAMIC DISPATCH
// ============================================================================
// STATIC DISPATCH (Generics):
// fn process<T: Plugin>(plugin: &T) {
//     plugin.process("data");
// }
// - Compiler generates separate code for each type T
// - No runtime overhead (inlined, optimized)
// - Larger binary size
// - Types must be known at compile time
//
// DYNAMIC DISPATCH (Trait Objects):
// fn process(plugin: &dyn Plugin) {
//     plugin.process("data");
// }
// - Single function for all types
// - Small runtime cost (vtable lookup)
// - Smaller binary size
// - Enables runtime extensibility
//
// HYBRID APPROACH:
// fn process<T: Plugin>(plugin: &T) {
//     process_dyn(plugin);
// }
// fn process_dyn(plugin: &dyn Plugin) {
//     plugin.process("data");
// }
// - Generic wrapper over dynamic dispatch
// - Best of both worlds

// ============================================================================
// WHY THIS MATTERS
// ============================================================================
// Plugin systems are used in:
// - **Applications**: VSCode, Chrome, Vim (extensibility)
// - **Frameworks**: Web frameworks, game engines
// - **Tools**: Compilers (LLVM passes), build systems
// - **Libraries**: Middleware systems, event handlers
//
// Understanding trait objects enables you to:
// - Build extensible systems
// - Write framework code
// - Implement dependency injection
// - Create flexible APIs

// ============================================================================
// IMPROVEMENTS FOR PRODUCTION
// ============================================================================
// 1. Error handling (Result<(), PluginError>)
// 2. Plugin configuration (load from files)
// 3. Plugin dependencies (dependency graph)
// 4. Version compatibility checking
// 5. Plugin isolation (separate processes/sandboxing)
// 6. Hot reloading (libloading crate for dynamic libraries)
// 7. Async support (async-trait crate)
// 8. Plugin marketplace/discovery
// 9. Resource limits (CPU, memory quotas)
// 10. Comprehensive testing framework

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Trying to clone trait objects (they're not Clone by default)
// ❌ Returning Self from trait methods (breaks object safety)
// ❌ Using associated types (breaks object safety)
// ❌ Forgetting Box when storing in collections
// ❌ Trying to downcast without Any trait
// ❌ Not understanding vtable overhead
// ❌ Using trait objects when generics would be better
// ❌ Assuming trait objects have zero cost (they have small cost)
