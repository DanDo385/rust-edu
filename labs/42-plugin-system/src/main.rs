//! # Plugin System - Demo
//!
//! Demonstrates registering trait-object plugins, invoking them, and calling
//! standalone data formatter plugins to produce different output formats.

use plugin_system::solution::{
    AnalyticsPlugin, CachePlugin, CsvFormatter, DataFormatter, JsonFormatter, LoggerPlugin,
    NotificationPlugin, PluginRegistry, XmlFormatter,
};

fn main() {
    println!("=== Plugin System Demo ===\n");

    let mut registry = PluginRegistry::new();
    register_sample_plugins(&mut registry);

    println!("=== Registered Plugins ===");
    for (id, name, version) in registry.list_plugins() {
        println!("  [{}] {} v{}", id, name, version);
    }

    println!("\n=== Initializing All Plugins ===");
    for status in registry.initialize_all() {
        println!("  {}", status);
    }

    println!("\n=== Processing Event ===");
    let event = "User login";
    for output in registry.process_all(event) {
        println!("  {}", output);
    }

    println!("\n=== Shutting Down ===");
    for status in registry.shutdown_all() {
        println!("  {}", status);
    }

    println!("\n=== Data Formatters ===");
    let record = [("name", "Alice"), ("city", "Seattle"), ("role", "Engineer")];
    print_formatter(&JsonFormatter, &record);
    print_formatter(&XmlFormatter, &record);
    print_formatter(&CsvFormatter, &record);
}

fn register_sample_plugins(registry: &mut PluginRegistry) {
    registry.register("logger", Box::new(LoggerPlugin::new("app.log")));
    registry.register("cache", Box::new(CachePlugin::new(100)));
    registry.register("analytics", Box::new(AnalyticsPlugin::new("UA-123")));
    registry.register("notification", Box::new(NotificationPlugin));
}

fn print_formatter(formatter: &dyn DataFormatter, data: &[(&str, &str)]) {
    println!("--- {} ---", formatter.format_type());
    println!("{}", formatter.format(data));
}
