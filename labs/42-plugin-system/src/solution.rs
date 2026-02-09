// Lab 42: Plugin System
//
// This module implements a plugin system using trait objects and dynamic dispatch.
// All output is returned as data (Strings, Vecs) rather than printed, making the
// entire system testable without capturing stdout.
//
// Key Concepts:
// - Trait objects (dyn Trait) for runtime polymorphism
// - Box<dyn Trait> for heap-allocated trait objects
// - Dynamic dispatch via vtables
// - Plugin registry pattern for managing heterogeneous plugins
// - Object safety rules and why they matter
// - Multiple trait hierarchies (Plugin + DataFormatter)

use std::collections::HashMap;

// ============================================================================
// PLUGIN TRAIT
// ============================================================================

/// The main plugin trait that all plugins must implement.
///
/// This trait is "object-safe" because:
/// - No methods return Self
/// - No methods have generic type parameters
/// - All methods take &self or &mut self
///
/// # Memory Model (trait object)
/// A `dyn Plugin` is a "fat pointer" consisting of:
/// - Data pointer (8 bytes): points to the concrete struct on the heap
/// - Vtable pointer (8 bytes): points to a table of function pointers
/// Total: 16 bytes per trait object reference
pub trait Plugin {
    /// Get the plugin's name.
    fn name(&self) -> &str;

    /// Get the plugin's version. Default implementation returns "1.0.0".
    fn version(&self) -> &str {
        "1.0.0"
    }

    /// Initialize the plugin. Returns a status message.
    /// Default implementation returns a generic initialization message.
    fn initialize(&mut self) -> String {
        format!("[{}] Initialized", self.name())
    }

    /// Process some data and return the result as a String.
    fn process(&self, data: &str) -> String;

    /// Shutdown the plugin. Returns a status message.
    fn shutdown(&self) -> String {
        format!("[{}] Shut down", self.name())
    }
}

// ============================================================================
// PLUGIN IMPLEMENTATIONS
// ============================================================================

/// Logger plugin - simulates logging events to a file.
///
/// Tracks whether it has been initialized via the `enabled` flag.
pub struct LoggerPlugin {
    filename: String,
    enabled: bool,
}

impl LoggerPlugin {
    pub fn new(filename: &str) -> Self {
        LoggerPlugin {
            filename: filename.to_string(),
            enabled: false,
        }
    }

    /// Check if the logger is enabled (has been initialized).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get the configured filename.
    pub fn filename(&self) -> &str {
        &self.filename
    }
}

impl Plugin for LoggerPlugin {
    fn name(&self) -> &str {
        "Logger"
    }

    fn version(&self) -> &str {
        "1.2.0"
    }

    fn initialize(&mut self) -> String {
        self.enabled = true;
        format!("[{}] Initialized - logging to {}", self.name(), self.filename)
    }

    fn process(&self, data: &str) -> String {
        if self.enabled {
            format!("[{}] Logged to {}: {}", self.name(), self.filename, data)
        } else {
            format!("[{}] Not initialized, cannot log: {}", self.name(), data)
        }
    }
}

/// Cache plugin - simulates an in-memory cache with a max size.
pub struct CachePlugin {
    max_size: usize,
    cache: HashMap<String, String>,
}

impl CachePlugin {
    pub fn new(max_size: usize) -> Self {
        CachePlugin {
            max_size,
            cache: HashMap::new(),
        }
    }

    /// Get the maximum cache size.
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Get the current number of cached items.
    pub fn current_size(&self) -> usize {
        self.cache.len()
    }

    /// Store a key-value pair in the cache.
    /// Returns true if stored, false if cache is full.
    pub fn store(&mut self, key: String, value: String) -> bool {
        if self.cache.len() >= self.max_size {
            return false;
        }
        self.cache.insert(key, value);
        true
    }

    /// Retrieve a value from the cache by key.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.cache.get(key)
    }
}

impl Plugin for CachePlugin {
    fn name(&self) -> &str {
        "Cache"
    }

    fn initialize(&mut self) -> String {
        self.cache.clear();
        format!("[{}] Initialized - max size: {} items", self.name(), self.max_size)
    }

    fn process(&self, data: &str) -> String {
        format!(
            "[{}] Caching data: {} (current size: {})",
            self.name(),
            data,
            self.cache.len()
        )
    }
}

/// Analytics plugin - simulates sending data to an analytics service.
pub struct AnalyticsPlugin {
    tracking_id: String,
}

impl AnalyticsPlugin {
    pub fn new(tracking_id: &str) -> Self {
        AnalyticsPlugin {
            tracking_id: tracking_id.to_string(),
        }
    }

    /// Get the configured tracking ID.
    pub fn tracking_id(&self) -> &str {
        &self.tracking_id
    }
}

impl Plugin for AnalyticsPlugin {
    fn name(&self) -> &str {
        "Analytics"
    }

    fn initialize(&mut self) -> String {
        format!("[{}] Initialized - tracking ID: {}", self.name(), self.tracking_id)
    }

    fn process(&self, data: &str) -> String {
        format!("[{}] Sending to analytics ({}): {}", self.name(), self.tracking_id, data)
    }
}

/// Notification plugin - simulates sending notifications.
///
/// A minimal plugin with no configuration state, demonstrating
/// that plugins can be as simple as a unit struct.
pub struct NotificationPlugin;

impl Plugin for NotificationPlugin {
    fn name(&self) -> &str {
        "Notification"
    }

    fn process(&self, data: &str) -> String {
        format!("[{}] Notification: {}", self.name(), data)
    }
}

// ============================================================================
// PLUGIN REGISTRY
// ============================================================================

/// Registry that manages a collection of plugins by string ID.
///
/// Uses HashMap<String, Box<dyn Plugin>> to store heterogeneous plugin types.
/// Each plugin is heap-allocated (Box) and accessed through dynamic dispatch.
///
/// # Memory Model
/// - HashMap on stack: ~48 bytes (pointer, len, cap, hash builder)
/// - Each entry on heap: String key + Box<dyn Plugin> (16-byte fat pointer)
/// - Each Box points to the concrete plugin struct on the heap
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        PluginRegistry {
            plugins: HashMap::new(),
        }
    }

    /// Register a plugin with the given ID.
    /// If a plugin with this ID already exists, it is replaced.
    pub fn register(&mut self, id: &str, plugin: Box<dyn Plugin>) {
        self.plugins.insert(id.to_string(), plugin);
    }

    /// Unregister (remove) a plugin by ID. Returns true if a plugin was removed.
    pub fn unregister(&mut self, id: &str) -> bool {
        self.plugins.remove(id).is_some()
    }

    /// Get a reference to a specific plugin by ID.
    pub fn get_plugin(&self, id: &str) -> Option<&dyn Plugin> {
        self.plugins.get(id).map(|boxed| boxed.as_ref())
    }

    /// Get a mutable reference to a specific plugin by ID.
    pub fn get_plugin_mut(&mut self, id: &str) -> Option<&mut Box<dyn Plugin>> {
        self.plugins.get_mut(id)
    }

    /// Get the number of registered plugins.
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Check if a plugin with the given ID is registered.
    pub fn has_plugin(&self, id: &str) -> bool {
        self.plugins.contains_key(id)
    }

    /// Get a list of all plugin IDs (sorted for deterministic output).
    pub fn plugin_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.plugins.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Get plugin info (id, name, version) for all registered plugins (sorted by ID).
    pub fn list_plugins(&self) -> Vec<(String, String, String)> {
        let mut entries: Vec<(String, String, String)> = self
            .plugins
            .iter()
            .map(|(id, plugin)| {
                (
                    id.clone(),
                    plugin.name().to_string(),
                    plugin.version().to_string(),
                )
            })
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        entries
    }

    /// Initialize all plugins. Returns a Vec of status messages.
    pub fn initialize_all(&mut self) -> Vec<String> {
        let mut messages = Vec::new();
        // Collect keys first to avoid borrow issues
        let keys: Vec<String> = self.plugins.keys().cloned().collect();
        for key in keys {
            if let Some(plugin) = self.plugins.get_mut(&key) {
                messages.push(plugin.initialize());
            }
        }
        messages
    }

    /// Process data with all plugins. Returns a Vec of results.
    pub fn process_all(&self, data: &str) -> Vec<String> {
        self.plugins
            .values()
            .map(|plugin| plugin.process(data))
            .collect()
    }

    /// Shutdown all plugins. Returns a Vec of status messages.
    pub fn shutdown_all(&self) -> Vec<String> {
        self.plugins
            .values()
            .map(|plugin| plugin.shutdown())
            .collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// DATA FORMATTER TRAIT
// ============================================================================

/// A trait for formatting key-value data into different output formats.
///
/// Demonstrates a second trait hierarchy independent of Plugin,
/// showing that trait objects can be used for any polymorphic pattern.
pub trait DataFormatter {
    /// Format key-value pairs into a string representation.
    fn format(&self, data: &[(&str, &str)]) -> String;

    /// Get the format type name (e.g., "JSON", "XML", "CSV").
    fn format_type(&self) -> &str;
}

// ============================================================================
// FORMATTER IMPLEMENTATIONS
// ============================================================================

/// Formats data as a JSON-like object.
pub struct JsonFormatter;

impl DataFormatter for JsonFormatter {
    fn format(&self, data: &[(&str, &str)]) -> String {
        let fields: Vec<String> = data
            .iter()
            .map(|(k, v)| format!("  \"{}\": \"{}\"", k, v))
            .collect();
        format!("{{\n{}\n}}", fields.join(",\n"))
    }

    fn format_type(&self) -> &str {
        "JSON"
    }
}

/// Formats data as XML elements.
pub struct XmlFormatter;

impl DataFormatter for XmlFormatter {
    fn format(&self, data: &[(&str, &str)]) -> String {
        let fields: Vec<String> = data
            .iter()
            .map(|(k, v)| format!("  <{}>{}</{}>", k, v, k))
            .collect();
        format!("<data>\n{}\n</data>", fields.join("\n"))
    }

    fn format_type(&self) -> &str {
        "XML"
    }
}

/// Formats data as CSV (comma-separated values).
pub struct CsvFormatter;

impl DataFormatter for CsvFormatter {
    fn format(&self, data: &[(&str, &str)]) -> String {
        let headers: Vec<&str> = data.iter().map(|(k, _)| *k).collect();
        let values: Vec<&str> = data.iter().map(|(_, v)| *v).collect();
        format!("{}\n{}", headers.join(","), values.join(","))
    }

    fn format_type(&self) -> &str {
        "CSV"
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. TRAIT OBJECTS (dyn Trait)
//    A fat pointer consisting of a data pointer + vtable pointer (16 bytes).
//    The vtable contains function pointers for each trait method.
//    Dynamic dispatch: method resolution happens at runtime via vtable lookup.
//
// 2. BOX<DYN TRAIT>
//    Box allocates the concrete type on the heap.
//    The Box itself stores the fat pointer (data ptr + vtable ptr).
//    Different plugins have different sizes, but Box<dyn Plugin> is always 16 bytes.
//
// 3. OBJECT SAFETY
//    A trait is object-safe if it can be used as `dyn Trait`.
//    Rules: no Self return types, no generic methods, no associated constants.
//    These rules ensure the vtable can be constructed at compile time.
//
// 4. HASHMAP<STRING, BOX<DYN PLUGIN>>
//    Heterogeneous collection: stores different concrete types under one type.
//    Key lookup is O(1) average via hashing.
//    Each value is a fat pointer to a heap-allocated plugin.
//
// 5. DEFAULT METHODS
//    Trait methods with default implementations (version, initialize, shutdown)
//    are included in the vtable. Overriding them replaces the vtable entry.
//    Default methods reduce boilerplate while allowing customization.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_plugin_basics() {
        let plugin = NotificationPlugin;
        assert_eq!(plugin.name(), "Notification");
        assert_eq!(plugin.version(), "1.0.0"); // default
    }

    #[test]
    fn test_logger_plugin_disabled_by_default() {
        let logger = LoggerPlugin::new("test.log");
        assert!(!logger.is_enabled());
    }

    #[test]
    fn test_registry_default() {
        let registry = PluginRegistry::default();
        assert!(registry.is_empty());
    }

    #[test]
    fn test_json_formatter_type() {
        let fmt = JsonFormatter;
        assert_eq!(fmt.format_type(), "JSON");
    }
}
