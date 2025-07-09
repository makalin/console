use console::plugin::{Plugin, PluginMetadata, PluginCategory, PluginSetting, SettingType};
use console::telemetry::TelemetryData;
use console::storage::Storage;
use console::{calculate_average, calculate_std_deviation, mph_to_kmh, kmh_to_mph, format_speed, format_rpm, is_valid_speed, is_valid_rpm};
use std::fs;

// Mock plugin for testing
struct MockPlugin {
    initialized: bool,
    update_count: i32,
    enabled: bool,
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
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    fn get_metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "Mock Plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "A mock plugin for testing".to_string(),
            category: PluginCategory::Speedometer,
            dependencies: vec![],
            settings: vec![],
        }
    }
}

#[test]
fn test_plugin_lifecycle() {
    let mut plugin = MockPlugin {
        initialized: false,
        update_count: 0,
        enabled: true,
    };

    // Test initialization
    plugin.init();
    assert!(plugin.initialized, "Plugin should be initialized");

    // Test update
    let telemetry = TelemetryData::new();
    plugin.update(&telemetry);
    assert_eq!(plugin.update_count, 1, "Update count should be incremented");
    
    // Test metadata
    let metadata = plugin.get_metadata();
    assert_eq!(metadata.name, "Mock Plugin");
    assert_eq!(metadata.category, PluginCategory::Speedometer);
    
    // Test enable/disable
    assert!(plugin.is_enabled());
    plugin.set_enabled(false);
    assert!(!plugin.is_enabled());
}

#[test]
fn test_telemetry_data() {
    let mut telemetry = TelemetryData::new();
    
    // Test default values
    assert_eq!(telemetry.speed, 0.0, "Default speed should be 0.0");
    assert_eq!(telemetry.rpm, 0.0, "Default RPM should be 0.0");
    assert_eq!(telemetry.engine_temp, 0.0, "Default engine temp should be 0.0");
    assert_eq!(telemetry.fuel_level, 0.0, "Default fuel level should be 0.0");
    
    // Test updating values
    telemetry.speed = 60.0;
    telemetry.rpm = 3000.0;
    telemetry.engine_temp = 180.0;
    telemetry.fuel_level = 75.0;
    
    assert_eq!(telemetry.speed, 60.0, "Speed should be updated to 60.0");
    assert_eq!(telemetry.rpm, 3000.0, "RPM should be updated to 3000.0");
    assert_eq!(telemetry.engine_temp, 180.0, "Engine temp should be updated to 180.0");
    assert_eq!(telemetry.fuel_level, 75.0, "Fuel level should be updated to 75.0");
    
    // Test utility methods
    telemetry.rpm = 0.0; // Reset to 0 for this test
    assert!(!telemetry.is_engine_running(), "Engine should not be running at 0 RPM");
    telemetry.rpm = 500.0;
    assert!(telemetry.is_engine_running(), "Engine should be running at 500 RPM");
    
    telemetry.speed = 0.0; // Reset to 0 for this test
    assert!(!telemetry.is_moving(), "Vehicle should not be moving at 0 speed");
    telemetry.speed = 5.0;
    assert!(telemetry.is_moving(), "Vehicle should be moving at 5 mph");
    
    assert_eq!(telemetry.engine_status(), "Running");
    assert_eq!(telemetry.gear_string(), "0");
    
    // Test gear string
    telemetry.gear = 3;
    assert_eq!(telemetry.gear_string(), "3");
    telemetry.gear = -1;
    assert_eq!(telemetry.gear_string(), "R");
    telemetry.gear = 0;
    assert_eq!(telemetry.gear_string(), "N");
}

#[test]
fn test_telemetry_validation() {
    let mut telemetry = TelemetryData::new();
    
    // Test valid data - set reasonable defaults
    telemetry.battery_voltage = 12.5;
    assert!(telemetry.is_valid(), "Default telemetry should be valid");
    
    // Test invalid speed
    telemetry.speed = 250.0;
    assert!(!telemetry.is_valid(), "Speed > 200 should be invalid");
    
    // Test invalid RPM
    telemetry.speed = 60.0;
    telemetry.rpm = 15000.0;
    assert!(!telemetry.is_valid(), "RPM > 10000 should be invalid");
    
    // Test invalid engine temp
    telemetry.rpm = 3000.0;
    telemetry.engine_temp = 350.0;
    assert!(!telemetry.is_valid(), "Engine temp > 300 should be invalid");
    
    // Test invalid fuel level
    telemetry.engine_temp = 180.0;
    telemetry.fuel_level = 150.0;
    assert!(!telemetry.is_valid(), "Fuel level > 100 should be invalid");
}

#[test]
fn test_telemetry_alerts() {
    let mut telemetry = TelemetryData::new();
    
    // Test no alerts for normal values
    // Set reasonable default values to avoid alerts
    telemetry.engine_temp = 180.0;
    telemetry.fuel_level = 50.0;
    telemetry.battery_voltage = 12.5;
    telemetry.oil_pressure = 30.0;
    telemetry.tire_pressure_fl = 35.0;
    telemetry.tire_pressure_fr = 35.0;
    telemetry.tire_pressure_rl = 35.0;
    telemetry.tire_pressure_rr = 35.0;
    assert!(telemetry.get_alerts().is_empty(), "Should have no alerts for normal values");
    
    // Test high engine temperature alert
    telemetry.engine_temp = 250.0;
    let alerts = telemetry.get_alerts();
    assert!(alerts.iter().any(|a| a.contains("temperature")), "Should have temperature alert");
    
    // Test low fuel alert
    telemetry.engine_temp = 180.0;
    telemetry.fuel_level = 5.0;
    let alerts = telemetry.get_alerts();
    assert!(alerts.iter().any(|a| a.contains("fuel")), "Should have fuel alert");
    
    // Test low battery alert
    telemetry.fuel_level = 75.0;
    telemetry.battery_voltage = 10.0;
    let alerts = telemetry.get_alerts();
    assert!(alerts.iter().any(|a| a.contains("battery")), "Should have battery alert");
}

#[test]
fn test_storage_operations() {
    let test_file = "test_storage.json";
    let storage = Storage::new(test_file);
    
    // Create test data
    let mut test_data = TelemetryData::new();
    test_data.speed = 75.0;
    test_data.rpm = 4000.0;
    test_data.engine_temp = 185.0;
    test_data.fuel_level = 60.0;
    
    // Test saving
    storage.save(&test_data).expect("Failed to save telemetry data");
    
    // Test loading
    let loaded_data = storage.load().expect("Failed to load telemetry data");
    assert_eq!(loaded_data.speed, 75.0, "Loaded speed should match saved speed");
    assert_eq!(loaded_data.rpm, 4000.0, "Loaded RPM should match saved RPM");
    assert_eq!(loaded_data.engine_temp, 185.0, "Loaded engine temp should match saved temp");
    assert_eq!(loaded_data.fuel_level, 60.0, "Loaded fuel level should match saved level");
    
    // Test storage stats
    let stats = storage.get_stats().expect("Failed to get storage stats");
    assert!(stats.main_file_size > 0, "File size should be greater than 0");
    
    // Test file size formatting
    let size_str = storage.get_file_size_human().expect("Failed to get file size");
    assert!(!size_str.is_empty(), "File size string should not be empty");
    
    // Test health check
    assert!(storage.is_healthy().expect("Health check should pass"), "Storage should be healthy");
    
    // Cleanup test file
    fs::remove_file(test_file).expect("Failed to remove test file");
}

#[test]
fn test_storage_backup() {
    let test_file = "test_backup.json";
    let storage = Storage::new(test_file);
    
    // Create test data
    let test_data = TelemetryData::new();
    storage.save(&test_data).expect("Failed to save test data");
    
    // Test backup creation
    let backup_path = storage.create_backup().expect("Failed to create backup");
    assert!(std::path::Path::new(&backup_path).exists(), "Backup file should exist");
    
    // Test backup listing
    let backups = storage.list_backups().expect("Failed to list backups");
    assert!(!backups.is_empty(), "Should have at least one backup");
    
    // Test backup restoration
    if let Some(backup_name) = backups.first() {
        storage.restore_backup(backup_name).expect("Failed to restore backup");
        let restored_data = storage.load().expect("Failed to load restored data");
        assert_eq!(restored_data.speed, test_data.speed, "Restored data should match original");
    }
    
    // Cleanup
    fs::remove_file(test_file).expect("Failed to remove test file");
    if let Ok(backups) = storage.list_backups() {
        for backup in backups {
            let backup_path = format!("{}/{}", storage.backup_dir, backup);
            let _ = fs::remove_file(backup_path);
        }
    }
    let _ = fs::remove_dir(&storage.backup_dir);
}

#[test]
fn test_utility_functions() {
    // Test average calculation
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(calculate_average(&numbers), 3.0, "Average should be 3.0");
    
    // Test standard deviation
    let std_dev = calculate_std_deviation(&numbers);
    assert!((std_dev - 1.5811).abs() < 0.01, "Standard deviation should be approximately 1.5811");
    
    // Test speed conversion
    assert_eq!(mph_to_kmh(60.0), 96.5604, "60 mph should equal 96.5604 kmh");
    assert!((kmh_to_mph(100.0) - 62.1371).abs() < 0.001, "100 kmh should equal approximately 62.1371 mph");
    
    // Test speed formatting
    assert_eq!(format_speed(60.0, false), "60.0 mph", "Speed should format as mph");
    assert_eq!(format_speed(60.0, true), "96.6 km/h", "Speed should format as kmh");
    
    // Test RPM formatting
    assert_eq!(format_rpm(3000.0), "3000 RPM", "RPM should format correctly");
    
    // Test validation functions
    assert!(is_valid_speed(60.0), "60 mph should be valid");
    assert!(!is_valid_speed(250.0), "250 mph should be invalid");
    assert!(is_valid_rpm(3000.0), "3000 RPM should be valid");
    assert!(!is_valid_rpm(15000.0), "15000 RPM should be invalid");
}

#[test]
fn test_telemetry_utils() {
    use console::telemetry::utils;
    use std::collections::VecDeque;
    
    // Test moving average
    let mut data_points = VecDeque::new();
    data_points.push_back(1.0);
    data_points.push_back(2.0);
    data_points.push_back(3.0);
    
    let avg = utils::moving_average(&data_points, 3);
    assert_eq!(avg, 2.0, "Moving average should be 2.0");
    
    // Test interpolation
    let start = TelemetryData::new();
    let mut end = TelemetryData::new();
    end.speed = 100.0;
    end.rpm = 5000.0;
    
    let interpolated = utils::interpolate_telemetry(&start, &end, 0.5);
    assert_eq!(interpolated.speed, 50.0, "Interpolated speed should be 50.0");
    assert_eq!(interpolated.rpm, 2500.0, "Interpolated RPM should be 2500.0");
    
    // Test distance calculation
    let distance = utils::calculate_distance(40.0, -74.0, 41.0, -75.0);
    assert!(distance > 0.0, "Distance should be positive");
}

#[test]
fn test_storage_utils() {
    use console::storage::utils;
    
    // Test data compression
    let mut data_points = vec![
        TelemetryData::new(),
        TelemetryData::new(),
        TelemetryData::new(),
    ];
    data_points[1].speed = 50.0;
    data_points[2].speed = 100.0;
    
    let compressed = utils::compress_telemetry_data(&data_points);
    assert!(compressed.len() <= data_points.len(), "Compressed data should not be longer");
    
    // Test data integrity
    let test_data = TelemetryData::new();
    let hash = utils::calculate_data_hash(&test_data);
    assert!(utils::validate_data_integrity(&test_data, hash), "Data integrity should be valid");
}

#[test]
fn test_plugin_utils() {
    use console::plugin::utils;
    
    // Test config validation
    let mut metadata = PluginMetadata::default();
    metadata.settings.push(PluginSetting {
        name: "test_setting".to_string(),
        value_type: SettingType::String,
        default_value: "default".to_string(),
        description: "Test setting".to_string(),
        required: true,
    });
    
    let mut config = std::collections::HashMap::new();
    config.insert("test_setting".to_string(), "test_value".to_string());
    
    assert!(utils::validate_config(&metadata, &config).is_ok(), "Valid config should pass validation");
    
    // Test setting value parsing
    let result = utils::parse_setting_value(&SettingType::Integer, "42");
    assert!(result.is_ok(), "Valid integer should parse successfully");
    
    let result = utils::parse_setting_value(&SettingType::Float, "3.14");
    assert!(result.is_ok(), "Valid float should parse successfully");
    
    let result = utils::parse_setting_value(&SettingType::Boolean, "true");
    assert!(result.is_ok(), "Valid boolean should parse successfully");
}

#[test]
fn test_plugin_loading() {
    // This test would require actual plugin binaries
    // For now, we'll just test that the plugin system can be initialized
    let mut plugin_manager = console::plugin::PluginManager::new();
    
    // Test adding a mock plugin
    let mock_plugin = MockPlugin {
        initialized: false,
        update_count: 0,
        enabled: true,
    };
    
    plugin_manager.add_plugin(Box::new(mock_plugin));
    
    // Test plugin management
    let metadata = plugin_manager.get_all_metadata();
    assert_eq!(metadata.len(), 1, "Should have one plugin");
    assert_eq!(metadata[0].name, "Mock Plugin", "Plugin name should match");
    
    // Test plugin enable/disable
    assert!(plugin_manager.enable_plugin("Mock Plugin"), "Should enable plugin");
    assert!(plugin_manager.get_enabled_plugins().contains(&"Mock Plugin".to_string()));
    
    assert!(plugin_manager.disable_plugin("Mock Plugin"), "Should disable plugin");
    assert!(!plugin_manager.get_enabled_plugins().contains(&"Mock Plugin".to_string()));
} 