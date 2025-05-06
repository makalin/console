use console::plugin::Plugin;
use console::telemetry::TelemetryData;
use console::storage::Storage;
use egui::Context;
use std::sync::Arc;
use std::fs;

// Mock plugin for testing
struct MockPlugin {
    initialized: bool,
    update_count: i32,
}

impl Plugin for MockPlugin {
    fn init(&mut self) {
        self.initialized = true;
    }

    fn update(&mut self, _data: &TelemetryData) {
        self.update_count += 1;
    }

    fn render(&self, _ui: &mut egui::Ui) {
        // Mock render implementation
    }
}

#[test]
fn test_plugin_lifecycle() {
    let mut plugin = MockPlugin {
        initialized: false,
        update_count: 0,
    };

    // Test initialization
    plugin.init();
    assert!(plugin.initialized, "Plugin should be initialized");

    // Test update
    let telemetry = TelemetryData::default();
    plugin.update(&telemetry);
    assert_eq!(plugin.update_count, 1, "Update count should be incremented");
}

#[test]
fn test_telemetry_data() {
    let mut telemetry = TelemetryData::default();
    
    // Test default values
    assert_eq!(telemetry.speed, 0.0, "Default speed should be 0.0");
    assert_eq!(telemetry.rpm, 0.0, "Default RPM should be 0.0");
    
    // Test updating values
    telemetry.speed = 60.0;
    telemetry.rpm = 3000.0;
    assert_eq!(telemetry.speed, 60.0, "Speed should be updated to 60.0");
    assert_eq!(telemetry.rpm, 3000.0, "RPM should be updated to 3000.0");
}

#[test]
fn test_storage_operations() {
    let test_file = "test_storage.json";
    let storage = Storage::new(test_file);
    
    // Create test data
    let mut test_data = TelemetryData::default();
    test_data.speed = 75.0;
    test_data.rpm = 4000.0;
    
    // Test saving
    storage.save(&test_data).expect("Failed to save telemetry data");
    
    // Test loading
    let loaded_data = storage.load().expect("Failed to load telemetry data");
    assert_eq!(loaded_data.speed, 75.0, "Loaded speed should match saved speed");
    assert_eq!(loaded_data.rpm, 4000.0, "Loaded RPM should match saved RPM");
    
    // Cleanup test file
    fs::remove_file(test_file).expect("Failed to remove test file");
}

#[test]
fn test_plugin_loading() {
    // Add plugin loading tests here
    // This will test the dynamic library loading functionality
} 