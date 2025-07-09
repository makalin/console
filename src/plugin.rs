use egui::Ui;
use crate::telemetry::TelemetryData;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub trait Plugin {
    fn init(&mut self);
    fn update(&mut self, data: &TelemetryData);
    fn render(&self, ui: &mut Ui);
    
    /// Get plugin metadata
    fn get_metadata(&self) -> PluginMetadata {
        PluginMetadata::default()
    }
    
    /// Get plugin configuration
    fn get_config(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    
    /// Set plugin configuration
    fn set_config(&mut self, _config: HashMap<String, String>) {}
    
    /// Check if plugin is enabled
    fn is_enabled(&self) -> bool {
        true
    }
    
    /// Enable or disable plugin
    fn set_enabled(&mut self, _enabled: bool) {}
    
    /// Get plugin status
    fn get_status(&self) -> PluginStatus {
        PluginStatus::Ready
    }
    
    /// Cleanup resources when plugin is unloaded
    fn cleanup(&mut self) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub category: PluginCategory,
    pub dependencies: Vec<String>,
    pub settings: Vec<PluginSetting>,
}

impl Default for PluginMetadata {
    fn default() -> Self {
        Self {
            name: "Unknown Plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Unknown".to_string(),
            description: "No description available".to_string(),
            category: PluginCategory::Other,
            dependencies: Vec::new(),
            settings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCategory {
    Speedometer,
    Engine,
    Fuel,
    Temperature,
    Pressure,
    Navigation,
    Entertainment,
    Diagnostics,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSetting {
    pub name: String,
    pub value_type: SettingType,
    pub default_value: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettingType {
    String,
    Integer,
    Float,
    Boolean,
    Color,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginStatus {
    Loading,
    Ready,
    Error(String),
    Disabled,
}

/// Plugin manager for handling multiple plugins
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    plugin_configs: HashMap<String, HashMap<String, String>>,
    enabled_plugins: Vec<String>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            plugin_configs: HashMap::new(),
            enabled_plugins: Vec::new(),
        }
    }
    
    /// Add a plugin to the manager
    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        let metadata = plugin.get_metadata();
        let plugin_name = metadata.name.clone();
        
        // Load saved configuration if available
        if let Some(_config) = self.plugin_configs.get(&plugin_name) {
            // We can't modify the plugin here due to trait object limitations
            // In a real implementation, you'd need to handle this differently
        }
        
        self.plugins.push(plugin);
    }
    
    /// Remove a plugin by name
    pub fn remove_plugin(&mut self, name: &str) -> Option<Box<dyn Plugin>> {
        if let Some(index) = self.plugins.iter().position(|p| p.get_metadata().name == name) {
            let mut plugin = self.plugins.remove(index);
            plugin.cleanup();
            Some(plugin)
        } else {
            None
        }
    }
    
    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.iter().find(|p| p.get_metadata().name == name).map(|p| p.as_ref())
    }
    
    /// Get a mutable plugin by name
    pub fn get_plugin_mut(&mut self, name: &str) -> Option<&mut Box<dyn Plugin>> {
        self.plugins.iter_mut().find(|p| p.get_metadata().name == name)
    }
    
    /// Update all plugins with telemetry data
    pub fn update_plugins(&mut self, data: &TelemetryData) {
        for plugin in &mut self.plugins {
            if plugin.is_enabled() {
                plugin.update(data);
            }
        }
    }
    
    /// Render all plugins
    pub fn render_plugins(&self, ui: &mut Ui) {
        for plugin in &self.plugins {
            if plugin.is_enabled() {
                plugin.render(ui);
            }
        }
    }
    
    /// Get all plugin metadata
    pub fn get_all_metadata(&self) -> Vec<PluginMetadata> {
        self.plugins.iter().map(|p| p.get_metadata()).collect()
    }
    
    /// Enable a plugin
    pub fn enable_plugin(&mut self, name: &str) -> bool {
        if let Some(plugin) = self.get_plugin_mut(name) {
            plugin.set_enabled(true);
            if !self.enabled_plugins.contains(&name.to_string()) {
                self.enabled_plugins.push(name.to_string());
            }
            true
        } else {
            false
        }
    }
    
    /// Disable a plugin
    pub fn disable_plugin(&mut self, name: &str) -> bool {
        if let Some(plugin) = self.get_plugin_mut(name) {
            plugin.set_enabled(false);
            self.enabled_plugins.retain(|n| n != name);
            true
        } else {
            false
        }
    }
    
    /// Get list of enabled plugins
    pub fn get_enabled_plugins(&self) -> &[String] {
        &self.enabled_plugins
    }
    
    /// Save plugin configurations
    pub fn save_configs(&self) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;
        use serde_json;
        
        let configs: HashMap<String, HashMap<String, String>> = self.plugins
            .iter()
            .map(|p| (p.get_metadata().name.clone(), p.get_config()))
            .collect();
        
        let json = serde_json::to_string_pretty(&configs)?;
        fs::write("plugin_configs.json", json)?;
        Ok(())
    }
    
    /// Load plugin configurations
    pub fn load_configs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;
        use serde_json;
        
        if let Ok(json) = fs::read_to_string("plugin_configs.json") {
            let configs: HashMap<String, HashMap<String, String>> = serde_json::from_str(&json)?;
            self.plugin_configs = configs;
            
            // Apply configurations to plugins
            for plugin in &mut self.plugins {
                let name = plugin.get_metadata().name.clone();
                if let Some(config) = self.plugin_configs.get(&name) {
                    plugin.set_config(config.clone());
                }
            }
        }
        Ok(())
    }
}

/// Utility functions for plugin development
pub mod utils {
    use super::*;
    use egui::{Color32, RichText};
    
    /// Create a standard plugin panel
    pub fn create_plugin_panel(ui: &mut Ui, title: &str, content: impl FnOnce(&mut Ui)) {
        ui.group(|ui| {
            ui.heading(RichText::new(title).color(Color32::from_rgb(100, 150, 255)));
            ui.separator();
            content(ui);
        });
    }
    
    /// Create a value display with label
    pub fn display_value(ui: &mut Ui, label: &str, value: &str, color: Option<Color32>) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("{}: ", label)).strong());
            let text = RichText::new(value);
            let text = if let Some(c) = color {
                text.color(c)
            } else {
                text
            };
            ui.label(text);
        });
    }
    
    /// Create a gauge display
    pub fn display_gauge(ui: &mut Ui, label: &str, value: f64, max_value: f64, color: Color32) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("{}: ", label)).strong());
            let progress = (value / max_value).clamp(0.0, 1.0) as f32;
            ui.add(egui::ProgressBar::new(progress)
                .text(format!("{:.1}", value))
                .fill(color));
        });
    }
    
    /// Create a status indicator
    pub fn display_status(ui: &mut Ui, status: &PluginStatus) {
        let (text, color) = match status {
            PluginStatus::Loading => ("Loading...", Color32::YELLOW),
            PluginStatus::Ready => ("Ready", Color32::GREEN),
            PluginStatus::Error(msg) => (msg.as_str(), Color32::RED),
            PluginStatus::Disabled => ("Disabled", Color32::GRAY),
        };
        
        ui.label(RichText::new(text).color(color));
    }
    
    /// Validate plugin configuration
    pub fn validate_config(metadata: &PluginMetadata, config: &HashMap<String, String>) -> Result<(), String> {
        for setting in &metadata.settings {
            if setting.required {
                if !config.contains_key(&setting.name) {
                    return Err(format!("Required setting '{}' is missing", setting.name));
                }
            }
        }
        Ok(())
    }
    
    /// Parse setting value based on type
    pub fn parse_setting_value(setting_type: &SettingType, value: &str) -> Result<Box<dyn std::any::Any>, String> {
        match setting_type {
            SettingType::String => Ok(Box::new(value.to_string())),
            SettingType::Integer => {
                value.parse::<i32>()
                    .map(|v| Box::new(v) as Box<dyn std::any::Any>)
                    .map_err(|e| format!("Invalid integer: {}", e))
            }
            SettingType::Float => {
                value.parse::<f64>()
                    .map(|v| Box::new(v) as Box<dyn std::any::Any>)
                    .map_err(|e| format!("Invalid float: {}", e))
            }
            SettingType::Boolean => {
                value.parse::<bool>()
                    .map(|v| Box::new(v) as Box<dyn std::any::Any>)
                    .map_err(|e| format!("Invalid boolean: {}", e))
            }
            SettingType::Color => {
                // Simple color parsing - in real implementation you'd want more robust parsing
                if value.starts_with('#') && value.len() == 7 {
                    Ok(Box::new(value.to_string()) as Box<dyn std::any::Any>)
                } else {
                    Err("Invalid color format. Use #RRGGBB".to_string())
                }
            }
            SettingType::File => {
                if std::path::Path::new(value).exists() {
                    Ok(Box::new(value.to_string()) as Box<dyn std::any::Any>)
                } else {
                    Err(format!("File not found: {}", value))
                }
            }
        }
    }
} 