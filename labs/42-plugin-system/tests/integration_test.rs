// Lab 42: Plugin System - Integration Tests
//
// These tests verify the Plugin trait, concrete plugin implementations,
// the PluginRegistry, and the DataFormatter trait. All output is captured
// as return values (no stdout dependency).

use plugin_system::solution::{
    AnalyticsPlugin, CachePlugin, CsvFormatter, DataFormatter, JsonFormatter, LoggerPlugin,
    NotificationPlugin, Plugin, PluginRegistry, XmlFormatter,
};

// ============================================================================
// LOGGER PLUGIN
// ============================================================================

#[test]
fn test_logger_plugin_name_and_version() {
    let logger = LoggerPlugin::new("app.log");
    assert_eq!(logger.name(), "Logger");
    assert_eq!(logger.version(), "1.2.0");
}

#[test]
fn test_logger_plugin_filename() {
    let logger = LoggerPlugin::new("custom.log");
    assert_eq!(logger.filename(), "custom.log");
}

#[test]
fn test_logger_plugin_disabled_before_init() {
    let logger = LoggerPlugin::new("app.log");
    assert!(!logger.is_enabled());
}

#[test]
fn test_logger_plugin_enabled_after_init() {
    let mut logger = LoggerPlugin::new("app.log");
    let msg = logger.initialize();
    assert!(logger.is_enabled());
    assert!(msg.contains("Initialized"));
    assert!(msg.contains("app.log"));
}

#[test]
fn test_logger_plugin_process_when_enabled() {
    let mut logger = LoggerPlugin::new("app.log");
    logger.initialize();
    let result = logger.process("user login");
    assert!(result.contains("Logged"));
    assert!(result.contains("app.log"));
    assert!(result.contains("user login"));
}

#[test]
fn test_logger_plugin_process_when_disabled() {
    let logger = LoggerPlugin::new("app.log");
    let result = logger.process("user login");
    assert!(result.contains("Not initialized"));
    assert!(result.contains("user login"));
}

#[test]
fn test_logger_plugin_shutdown() {
    let logger = LoggerPlugin::new("app.log");
    let msg = logger.shutdown();
    assert!(msg.contains("Logger"));
    assert!(msg.contains("Shut down"));
}

// ============================================================================
// CACHE PLUGIN
// ============================================================================

#[test]
fn test_cache_plugin_name_and_version() {
    let cache = CachePlugin::new(100);
    assert_eq!(cache.name(), "Cache");
    assert_eq!(cache.version(), "1.0.0"); // default version
}

#[test]
fn test_cache_plugin_max_size() {
    let cache = CachePlugin::new(50);
    assert_eq!(cache.max_size(), 50);
}

#[test]
fn test_cache_plugin_store_and_get() {
    let mut cache = CachePlugin::new(10);
    assert!(cache.store("key1".to_string(), "value1".to_string()));
    assert_eq!(cache.get("key1"), Some(&"value1".to_string()));
    assert_eq!(cache.current_size(), 1);
}

#[test]
fn test_cache_plugin_store_respects_max_size() {
    let mut cache = CachePlugin::new(2);
    assert!(cache.store("a".to_string(), "1".to_string()));
    assert!(cache.store("b".to_string(), "2".to_string()));
    assert!(!cache.store("c".to_string(), "3".to_string())); // full
    assert_eq!(cache.current_size(), 2);
}

#[test]
fn test_cache_plugin_get_missing_key() {
    let cache = CachePlugin::new(10);
    assert_eq!(cache.get("nonexistent"), None);
}

#[test]
fn test_cache_plugin_initialize_clears_cache() {
    let mut cache = CachePlugin::new(10);
    cache.store("key".to_string(), "value".to_string());
    assert_eq!(cache.current_size(), 1);

    let msg = cache.initialize();
    assert_eq!(cache.current_size(), 0);
    assert!(msg.contains("Initialized"));
    assert!(msg.contains("10"));
}

#[test]
fn test_cache_plugin_process() {
    let cache = CachePlugin::new(10);
    let result = cache.process("user data");
    assert!(result.contains("Cache"));
    assert!(result.contains("user data"));
    assert!(result.contains("current size: 0"));
}

// ============================================================================
// ANALYTICS PLUGIN
// ============================================================================

#[test]
fn test_analytics_plugin_name() {
    let analytics = AnalyticsPlugin::new("UA-123");
    assert_eq!(analytics.name(), "Analytics");
}

#[test]
fn test_analytics_plugin_tracking_id() {
    let analytics = AnalyticsPlugin::new("UA-456-789");
    assert_eq!(analytics.tracking_id(), "UA-456-789");
}

#[test]
fn test_analytics_plugin_initialize() {
    let mut analytics = AnalyticsPlugin::new("UA-123");
    let msg = analytics.initialize();
    assert!(msg.contains("Analytics"));
    assert!(msg.contains("UA-123"));
    assert!(msg.contains("Initialized"));
}

#[test]
fn test_analytics_plugin_process() {
    let analytics = AnalyticsPlugin::new("UA-123");
    let result = analytics.process("page view");
    assert!(result.contains("Analytics"));
    assert!(result.contains("UA-123"));
    assert!(result.contains("page view"));
}

// ============================================================================
// NOTIFICATION PLUGIN
// ============================================================================

#[test]
fn test_notification_plugin_name_and_version() {
    let notif = NotificationPlugin;
    assert_eq!(notif.name(), "Notification");
    assert_eq!(notif.version(), "1.0.0"); // default
}

#[test]
fn test_notification_plugin_initialize_default() {
    let mut notif = NotificationPlugin;
    let msg = notif.initialize();
    assert!(msg.contains("Notification"));
    assert!(msg.contains("Initialized"));
}

#[test]
fn test_notification_plugin_process() {
    let notif = NotificationPlugin;
    let result = notif.process("new message");
    assert!(result.contains("Notification"));
    assert!(result.contains("new message"));
}

#[test]
fn test_notification_plugin_shutdown_default() {
    let notif = NotificationPlugin;
    let msg = notif.shutdown();
    assert!(msg.contains("Notification"));
    assert!(msg.contains("Shut down"));
}

// ============================================================================
// PLUGIN REGISTRY - CREATION
// ============================================================================

#[test]
fn test_registry_new_is_empty() {
    let registry = PluginRegistry::new();
    assert!(registry.is_empty());
    assert_eq!(registry.plugin_count(), 0);
}

#[test]
fn test_registry_default_is_empty() {
    let registry = PluginRegistry::default();
    assert!(registry.is_empty());
}

// ============================================================================
// PLUGIN REGISTRY - REGISTRATION
// ============================================================================

#[test]
fn test_register_single_plugin() {
    let mut registry = PluginRegistry::new();
    registry.register("logger", Box::new(LoggerPlugin::new("app.log")));
    assert_eq!(registry.plugin_count(), 1);
    assert!(registry.has_plugin("logger"));
}

#[test]
fn test_register_multiple_plugins() {
    let mut registry = PluginRegistry::new();
    registry.register("logger", Box::new(LoggerPlugin::new("app.log")));
    registry.register("cache", Box::new(CachePlugin::new(100)));
    registry.register("analytics", Box::new(AnalyticsPlugin::new("UA-123")));
    registry.register("notification", Box::new(NotificationPlugin));

    assert_eq!(registry.plugin_count(), 4);
}

#[test]
fn test_register_replaces_existing() {
    let mut registry = PluginRegistry::new();
    registry.register("logger", Box::new(LoggerPlugin::new("old.log")));
    registry.register("logger", Box::new(LoggerPlugin::new("new.log")));

    assert_eq!(registry.plugin_count(), 1);
}

// ============================================================================
// PLUGIN REGISTRY - UNREGISTER
// ============================================================================

#[test]
fn test_unregister_existing_plugin() {
    let mut registry = PluginRegistry::new();
    registry.register("logger", Box::new(LoggerPlugin::new("app.log")));

    assert!(registry.unregister("logger"));
    assert_eq!(registry.plugin_count(), 0);
    assert!(!registry.has_plugin("logger"));
}

#[test]
fn test_unregister_nonexistent_plugin() {
    let mut registry = PluginRegistry::new();
    assert!(!registry.unregister("ghost"));
}

// ============================================================================
// PLUGIN REGISTRY - LOOKUP
// ============================================================================

#[test]
fn test_get_plugin_exists() {
    let mut registry = PluginRegistry::new();
    registry.register("analytics", Box::new(AnalyticsPlugin::new("UA-123")));

    let plugin = registry.get_plugin("analytics").unwrap();
    assert_eq!(plugin.name(), "Analytics");
}

#[test]
fn test_get_plugin_not_found() {
    let registry = PluginRegistry::new();
    assert!(registry.get_plugin("missing").is_none());
}

#[test]
fn test_get_plugin_mut_initialize() {
    let mut registry = PluginRegistry::new();
    registry.register("logger", Box::new(LoggerPlugin::new("app.log")));

    let plugin = registry.get_plugin_mut("logger").unwrap();
    let msg = plugin.initialize();
    assert!(msg.contains("Initialized"));
}

#[test]
fn test_has_plugin() {
    let mut registry = PluginRegistry::new();
    registry.register("cache", Box::new(CachePlugin::new(10)));

    assert!(registry.has_plugin("cache"));
    assert!(!registry.has_plugin("logger"));
}

// ============================================================================
// PLUGIN REGISTRY - LISTING
// ============================================================================

#[test]
fn test_plugin_ids_sorted() {
    let mut registry = PluginRegistry::new();
    registry.register("zebra", Box::new(NotificationPlugin));
    registry.register("alpha", Box::new(NotificationPlugin));
    registry.register("middle", Box::new(NotificationPlugin));

    let ids = registry.plugin_ids();
    assert_eq!(ids, vec!["alpha", "middle", "zebra"]);
}

#[test]
fn test_list_plugins_info() {
    let mut registry = PluginRegistry::new();
    registry.register("logger", Box::new(LoggerPlugin::new("app.log")));
    registry.register("analytics", Box::new(AnalyticsPlugin::new("UA-1")));

    let listing = registry.list_plugins();
    assert_eq!(listing.len(), 2);

    // Sorted by ID
    assert_eq!(listing[0].0, "analytics");
    assert_eq!(listing[0].1, "Analytics");

    assert_eq!(listing[1].0, "logger");
    assert_eq!(listing[1].1, "Logger");
    assert_eq!(listing[1].2, "1.2.0");
}

// ============================================================================
// PLUGIN REGISTRY - BULK OPERATIONS
// ============================================================================

#[test]
fn test_initialize_all() {
    let mut registry = PluginRegistry::new();
    registry.register("logger", Box::new(LoggerPlugin::new("app.log")));
    registry.register("notification", Box::new(NotificationPlugin));

    let messages = registry.initialize_all();
    assert_eq!(messages.len(), 2);
    assert!(messages.iter().all(|m| m.contains("Initialized")));
}

#[test]
fn test_process_all() {
    let mut registry = PluginRegistry::new();
    registry.register("analytics", Box::new(AnalyticsPlugin::new("UA-1")));
    registry.register("notification", Box::new(NotificationPlugin));

    let results = registry.process_all("test event");
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.contains("test event")));
}

#[test]
fn test_shutdown_all() {
    let mut registry = PluginRegistry::new();
    registry.register("logger", Box::new(LoggerPlugin::new("app.log")));
    registry.register("cache", Box::new(CachePlugin::new(10)));

    let messages = registry.shutdown_all();
    assert_eq!(messages.len(), 2);
    assert!(messages.iter().all(|m| m.contains("Shut down")));
}

#[test]
fn test_process_all_empty_registry() {
    let registry = PluginRegistry::new();
    let results = registry.process_all("data");
    assert!(results.is_empty());
}

// ============================================================================
// DATA FORMATTER - JSON
// ============================================================================

#[test]
fn test_json_formatter_type() {
    let fmt = JsonFormatter;
    assert_eq!(fmt.format_type(), "JSON");
}

#[test]
fn test_json_formatter_output() {
    let fmt = JsonFormatter;
    let data = vec![("name", "Alice"), ("age", "30")];
    let output = fmt.format(&data);

    assert!(output.contains("\"name\": \"Alice\""));
    assert!(output.contains("\"age\": \"30\""));
    assert!(output.starts_with('{'));
    assert!(output.ends_with('}'));
}

#[test]
fn test_json_formatter_empty_data() {
    let fmt = JsonFormatter;
    let data: Vec<(&str, &str)> = vec![];
    let output = fmt.format(&data);
    assert!(output.contains('{'));
    assert!(output.contains('}'));
}

#[test]
fn test_json_formatter_single_field() {
    let fmt = JsonFormatter;
    let data = vec![("key", "value")];
    let output = fmt.format(&data);
    assert!(output.contains("\"key\": \"value\""));
}

// ============================================================================
// DATA FORMATTER - XML
// ============================================================================

#[test]
fn test_xml_formatter_type() {
    let fmt = XmlFormatter;
    assert_eq!(fmt.format_type(), "XML");
}

#[test]
fn test_xml_formatter_output() {
    let fmt = XmlFormatter;
    let data = vec![("name", "Bob"), ("city", "NYC")];
    let output = fmt.format(&data);

    assert!(output.contains("<name>Bob</name>"));
    assert!(output.contains("<city>NYC</city>"));
    assert!(output.contains("<data>"));
    assert!(output.contains("</data>"));
}

#[test]
fn test_xml_formatter_empty_data() {
    let fmt = XmlFormatter;
    let data: Vec<(&str, &str)> = vec![];
    let output = fmt.format(&data);
    assert!(output.contains("<data>"));
    assert!(output.contains("</data>"));
}

// ============================================================================
// DATA FORMATTER - CSV
// ============================================================================

#[test]
fn test_csv_formatter_type() {
    let fmt = CsvFormatter;
    assert_eq!(fmt.format_type(), "CSV");
}

#[test]
fn test_csv_formatter_output() {
    let fmt = CsvFormatter;
    let data = vec![("name", "Alice"), ("age", "30"), ("city", "Seattle")];
    let output = fmt.format(&data);

    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "name,age,city");
    assert_eq!(lines[1], "Alice,30,Seattle");
}

#[test]
fn test_csv_formatter_single_column() {
    let fmt = CsvFormatter;
    let data = vec![("name", "Alice")];
    let output = fmt.format(&data);

    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "name");
    assert_eq!(lines[1], "Alice");
}

// ============================================================================
// DATA FORMATTER - POLYMORPHISM
// ============================================================================

#[test]
fn test_formatters_as_trait_objects() {
    let formatters: Vec<Box<dyn DataFormatter>> = vec![
        Box::new(JsonFormatter),
        Box::new(XmlFormatter),
        Box::new(CsvFormatter),
    ];

    let data = vec![("key", "value")];

    // Each formatter should produce different output
    let outputs: Vec<String> = formatters.iter().map(|f| f.format(&data)).collect();
    assert_eq!(outputs.len(), 3);

    // All outputs should contain the data
    for output in &outputs {
        assert!(output.contains("key") || output.contains("value"));
    }

    // Verify format types via trait object
    assert_eq!(formatters[0].format_type(), "JSON");
    assert_eq!(formatters[1].format_type(), "XML");
    assert_eq!(formatters[2].format_type(), "CSV");
}

// ============================================================================
// COMPLEX SCENARIOS
// ============================================================================

#[test]
fn test_full_plugin_lifecycle() {
    let mut registry = PluginRegistry::new();

    // Register
    registry.register("logger", Box::new(LoggerPlugin::new("app.log")));
    registry.register("cache", Box::new(CachePlugin::new(50)));
    registry.register("analytics", Box::new(AnalyticsPlugin::new("UA-999")));
    registry.register("notification", Box::new(NotificationPlugin));
    assert_eq!(registry.plugin_count(), 4);

    // Initialize
    let init_messages = registry.initialize_all();
    assert_eq!(init_messages.len(), 4);

    // Process
    let results = registry.process_all("user signup event");
    assert_eq!(results.len(), 4);
    for result in &results {
        assert!(result.contains("user signup event"));
    }

    // Shutdown
    let shutdown_messages = registry.shutdown_all();
    assert_eq!(shutdown_messages.len(), 4);

    // Unregister one
    registry.unregister("cache");
    assert_eq!(registry.plugin_count(), 3);
    assert!(!registry.has_plugin("cache"));
}

#[test]
fn test_registry_register_and_query_workflow() {
    let mut registry = PluginRegistry::new();

    registry.register("a", Box::new(NotificationPlugin));
    registry.register("b", Box::new(NotificationPlugin));
    registry.register("c", Box::new(NotificationPlugin));

    // IDs should be sorted
    assert_eq!(registry.plugin_ids(), vec!["a", "b", "c"]);

    // Remove middle
    registry.unregister("b");
    assert_eq!(registry.plugin_ids(), vec!["a", "c"]);
    assert_eq!(registry.plugin_count(), 2);

    // Add new
    registry.register("d", Box::new(NotificationPlugin));
    assert_eq!(registry.plugin_ids(), vec!["a", "c", "d"]);
}
