//! SigmaOS-accessibility — comprehensive accessibility component.
//!
//! Provides accessibility features for assistive technologies including
//! screen readers, magnification, high contrast themes, keyboard navigation,
//! and alternative input methods.

use std::collections::HashMap;

/// Accessibility feature type
#[derive(Debug, Clone, PartialEq)]
pub enum AccessibilityFeature {
    /// Screen reader
    ScreenReader,
    /// Magnification
    Magnification,
    /// High contrast
    HighContrast,
    /// Keyboard navigation
    KeyboardNavigation,
    /// Voice control
    VoiceControl,
    /// Switch control
    SwitchControl,
    /// Eye tracking
    EyeTracking,
    /// Alternative input
    AlternativeInput,
    /// Captions
    Captions,
    /// Audio description
    AudioDescription,
}

/// Accessibility setting
#[derive(Debug, Clone)]
pub struct AccessibilitySetting {
    /// Setting key
    pub key: String,
    /// Setting value
    pub value: String,
    /// Setting description
    pub description: String,
    /// Feature type
    pub feature: AccessibilityFeature,
    /// Enabled flag
    pub enabled: bool,
}

/// Accessibility profile
#[derive(Debug, Clone)]
pub struct AccessibilityProfile {
    /// Profile ID
    pub id: String,
    /// Profile name
    pub name: String,
    /// Screen reader active
    pub screen_reader_active: bool,
    /// Zoom level
    pub zoom_level: f32,
    /// High contrast enabled
    pub high_contrast: bool,
    /// Keyboard navigation enabled
    pub keyboard_navigation: bool,
    /// Voice control enabled
    pub voice_control: bool,
    /// Switch control enabled
    pub switch_control: bool,
    /// Eye tracking enabled
    pub eye_tracking: bool,
    /// Alternative input enabled
    pub alternative_input: bool,
    /// Captions enabled
    pub captions: bool,
    /// Audio description enabled
    pub audio_description: bool,
    /// Settings
    pub settings: HashMap<String, String>,
}

impl Default for AccessibilityProfile {
    fn default() -> Self {
        Self {
            id: "default".into(),
            name: "Default".into(),
            screen_reader_active: false,
            zoom_level: 1.0,
            high_contrast: false,
            keyboard_navigation: false,
            voice_control: false,
            switch_control: false,
            eye_tracking: false,
            alternative_input: false,
            captions: false,
            audio_description: false,
            settings: HashMap::new(),
        }
    }
}

impl AccessibilityProfile {
    /// Create new profile
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            ..Default::default()
        }
    }
    
    /// Enable feature
    pub fn enable_feature(&mut self, feature: AccessibilityFeature) {
        match feature {
            AccessibilityFeature::ScreenReader => self.screen_reader_active = true,
            AccessibilityFeature::Magnification => self.zoom_level = 2.0,
            AccessibilityFeature::HighContrast => self.high_contrast = true,
            AccessibilityFeature::KeyboardNavigation => self.keyboard_navigation = true,
            AccessibilityFeature::VoiceControl => self.voice_control = true,
            AccessibilityFeature::SwitchControl => self.switch_control = true,
            AccessibilityFeature::EyeTracking => self.eye_tracking = true,
            AccessibilityFeature::AlternativeInput => self.alternative_input = true,
            AccessibilityFeature::Captions => self.captions = true,
            AccessibilityFeature::AudioDescription => self.audio_description = true,
        }
    }
    
    /// Disable feature
    pub fn disable_feature(&mut self, feature: AccessibilityFeature) {
        match feature {
            AccessibilityFeature::ScreenReader => self.screen_reader_active = false,
            AccessibilityFeature::Magnification => self.zoom_level = 1.0,
            AccessibilityFeature::HighContrast => self.high_contrast = false,
            AccessibilityFeature::KeyboardNavigation => self.keyboard_navigation = false,
            AccessibilityFeature::VoiceControl => self.voice_control = false,
            AccessibilityFeature::SwitchControl => self.switch_control = false,
            AccessibilityFeature::EyeTracking => self.eye_tracking = false,
            AccessibilityFeature::AlternativeInput => self.alternative_input = false,
            AccessibilityFeature::Captions => self.captions = false,
            AccessibilityFeature::AudioDescription => self.audio_description = false,
        }
    }
    
    /// Check if feature is enabled
    pub fn is_feature_enabled(&self, feature: &AccessibilityFeature) -> bool {
        match feature {
            AccessibilityFeature::ScreenReader => self.screen_reader_active,
            AccessibilityFeature::Magnification => self.zoom_level > 1.0,
            AccessibilityFeature::HighContrast => self.high_contrast,
            AccessibilityFeature::KeyboardNavigation => self.keyboard_navigation,
            AccessibilityFeature::VoiceControl => self.voice_control,
            AccessibilityFeature::SwitchControl => self.switch_control,
            AccessibilityFeature::EyeTracking => self.eye_tracking,
            AccessibilityFeature::AlternativeInput => self.alternative_input,
            AccessibilityFeature::Captions => self.captions,
            AccessibilityFeature::AudioDescription => self.audio_description,
        }
    }
    
    /// Set zoom level
    pub fn set_zoom_level(&mut self, level: f32) {
        self.zoom_level = level.clamp(0.5, 10.0);
    }
    
    /// Get zoom level
    pub fn get_zoom_level(&self) -> f32 {
        self.zoom_level
    }
    
    /// Set setting
    pub fn set_setting(&mut self, key: String, value: String) {
        self.settings.insert(key, value);
    }
    
    /// Get setting
    pub fn get_setting(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }
    
    /// Get all enabled features
    pub fn get_enabled_features(&self) -> Vec<AccessibilityFeature> {
        let mut features = vec![];
        
        if self.screen_reader_active {
            features.push(AccessibilityFeature::ScreenReader);
        }
        if self.zoom_level > 1.0 {
            features.push(AccessibilityFeature::Magnification);
        }
        if self.high_contrast {
            features.push(AccessibilityFeature::HighContrast);
        }
        if self.keyboard_navigation {
            features.push(AccessibilityFeature::KeyboardNavigation);
        }
        if self.voice_control {
            features.push(AccessibilityFeature::VoiceControl);
        }
        if self.switch_control {
            features.push(AccessibilityFeature::SwitchControl);
        }
        if self.eye_tracking {
            features.push(AccessibilityFeature::EyeTracking);
        }
        if self.alternative_input {
            features.push(AccessibilityFeature::AlternativeInput);
        }
        if self.captions {
            features.push(AccessibilityFeature::Captions);
        }
        if self.audio_description {
            features.push(AccessibilityFeature::AudioDescription);
        }
        
        features
    }
    
    /// Check if any assistive technology is active
    pub fn is_assistive_active(&self) -> bool {
        !self.get_enabled_features().is_empty()
    }
}

/// Accessibility daemon
#[derive(Debug, Clone)]
pub struct AccessibilityDaemon {
    /// Profiles
    profiles: HashMap<String, AccessibilityProfile>,
    /// Current profile
    current_profile: Option<String>,
    /// Accessibility settings
    settings: HashMap<String, AccessibilitySetting>,
    /// Statistics
    total_profiles: u32,
    /// Active profiles
    active_profiles: u32,
}

impl Default for AccessibilityDaemon {
    fn default() -> Self {
        Self {
            profiles: HashMap::new(),
            current_profile: None,
            settings: HashMap::new(),
            total_profiles: 0,
            active_profiles: 0,
        }
    }
}

impl AccessibilityDaemon {
    /// Create new accessibility daemon
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize with default settings
    pub fn initialize(&mut self) {
        // Add default accessibility settings
        self.add_setting(AccessibilitySetting {
            key: "screen_reader.enabled".into(),
            value: "false".into(),
            description: "Enable screen reader".into(),
            feature: AccessibilityFeature::ScreenReader,
            enabled: false,
        });
        
        self.add_setting(AccessibilitySetting {
            key: "magnification.zoom_level".into(),
            value: "1.0".into(),
            description: "Screen magnification level".into(),
            feature: AccessibilityFeature::Magnification,
            enabled: false,
        });
        
        self.add_setting(AccessibilitySetting {
            key: "high_contrast.enabled".into(),
            value: "false".into(),
            description: "Enable high contrast theme".into(),
            feature: AccessibilityFeature::HighContrast,
            enabled: false,
        });
        
        self.add_setting(AccessibilitySetting {
            key: "keyboard_navigation.enabled".into(),
            value: "false".into(),
            description: "Enable keyboard navigation".into(),
            feature: AccessibilityFeature::KeyboardNavigation,
            enabled: false,
        });
        
        self.add_setting(AccessibilitySetting {
            key: "voice_control.enabled".into(),
            value: "false".into(),
            description: "Enable voice control".into(),
            feature: AccessibilityFeature::VoiceControl,
            enabled: false,
        });
        
        // Add default profile
        let default_profile = AccessibilityProfile::default();
        self.add_profile(default_profile);
        self.set_current_profile("default");
    }
    
    /// Add accessibility setting
    pub fn add_setting(&mut self, setting: AccessibilitySetting) -> String {
        let key = setting.key.clone();
        self.settings.insert(key.clone(), setting);
        key
    }
    
    /// Get accessibility setting
    pub fn get_setting(&self, key: &str) -> Option<&AccessibilitySetting> {
        self.settings.get(key)
    }
    
    /// Get all settings
    pub fn get_all_settings(&self) -> Vec<&AccessibilitySetting> {
        self.settings.values().collect()
    }
    
    /// Get settings by feature
    pub fn get_settings_by_feature(&self, feature: &AccessibilityFeature) -> Vec<&AccessibilitySetting> {
        self.settings.values()
            .filter(|s| s.feature == *feature)
            .collect()
    }
    
    /// Add profile
    pub fn add_profile(&mut self, profile: AccessibilityProfile) -> String {
        let id = profile.id.clone();
        self.profiles.insert(id.clone(), profile);
        self.total_profiles += 1;
        id
    }
    
    /// Remove profile
    pub fn remove_profile(&mut self, profile_id: &str) -> bool {
        if self.profiles.remove(profile_id).is_some() {
            self.total_profiles = self.total_profiles.saturating_sub(1);
            if self.current_profile.as_ref() == Some(&profile_id.to_string()) {
                self.current_profile = None;
            }
            true
        } else {
            false
        }
    }
    
    /// Get profile
    pub fn get_profile(&self, profile_id: &str) -> Option<&AccessibilityProfile> {
        self.profiles.get(profile_id)
    }
    
    /// Get profile mutably
    pub fn get_profile_mut(&mut self, profile_id: &str) -> Option<&mut AccessibilityProfile> {
        self.profiles.get_mut(profile_id)
    }
    
    /// Get all profiles
    pub fn get_all_profiles(&self) -> Vec<&AccessibilityProfile> {
        self.profiles.values().collect()
    }
    
    /// Set current profile
    pub fn set_current_profile(&mut self, profile_id: &str) -> bool {
        if self.profiles.contains_key(profile_id) {
            self.current_profile = Some(profile_id.to_string());
            self.update_active_profiles();
            true
        } else {
            false
        }
    }
    
    /// Get current profile
    pub fn get_current_profile(&self) -> Option<&AccessibilityProfile> {
        self.current_profile.as_ref().and_then(|id| self.profiles.get(id))
    }
    
    /// Enable feature on current profile
    pub fn enable_feature(&mut self, feature: AccessibilityFeature) -> bool {
        if let Some(profile_id) = &self.current_profile {
            if let Some(profile) = self.profiles.get_mut(profile_id) {
                profile.enable_feature(feature);
                self.update_active_profiles();
                return true;
            }
        }
        false
    }
    
    /// Disable feature on current profile
    pub fn disable_feature(&mut self, feature: AccessibilityFeature) -> bool {
        if let Some(profile_id) = &self.current_profile {
            if let Some(profile) = self.profiles.get_mut(profile_id) {
                profile.disable_feature(feature);
                self.update_active_profiles();
                return true;
            }
        }
        false
    }
    
    /// Check if feature is enabled on current profile
    pub fn is_feature_enabled(&self, feature: &AccessibilityFeature) -> bool {
        self.get_current_profile()
            .map(|p| p.is_feature_enabled(feature))
            .unwrap_or(false)
    }
    
    /// Set zoom level on current profile
    pub fn set_zoom_level(&mut self, level: f32) -> bool {
        if let Some(profile_id) = &self.current_profile {
            if let Some(profile) = self.profiles.get_mut(profile_id) {
                profile.set_zoom_level(level);
                self.update_active_profiles();
                return true;
            }
        }
        false
    }
    
    /// Get zoom level from current profile
    pub fn get_zoom_level(&self) -> f32 {
        self.get_current_profile()
            .map(|p| p.get_zoom_level())
            .unwrap_or(1.0)
    }
    
    /// Update active profiles count
    fn update_active_profiles(&mut self) {
        self.active_profiles = self.profiles.values()
            .filter(|p| p.is_assistive_active())
            .count() as u32;
    }
    
    /// Get accessibility statistics
    pub fn get_statistics(&self) -> AccessibilityStatistics {
        AccessibilityStatistics {
            total_profiles: self.total_profiles,
            active_profiles: self.active_profiles,
            total_settings: self.settings.len() as u32,
            enabled_features: self.settings.values()
                .filter(|s| s.enabled)
                .count() as u32,
            screen_reader_profiles: self.profiles.values()
                .filter(|p| p.screen_reader_active)
                .count() as u32,
            magnification_profiles: self.profiles.values()
                .filter(|p| p.zoom_level > 1.0)
                .count() as u32,
            high_contrast_profiles: self.profiles.values()
                .filter(|p| p.high_contrast)
                .count() as u32,
        }
    }
    
    /// Export accessibility data
    pub fn export_data(&self) -> AccessibilityExport {
        AccessibilityExport {
            profiles: self.profiles.values().cloned().collect(),
            settings: self.settings.values().cloned().collect(),
        }
    }
    
    /// Import accessibility data
    pub fn import_data(&mut self, export: AccessibilityExport) {
        for profile in export.profiles {
            self.profiles.insert(profile.id.clone(), profile);
        }
        for setting in export.settings {
            self.settings.insert(setting.key.clone(), setting);
        }
        self.total_profiles = self.profiles.len() as u32;
        self.update_active_profiles();
        // Set current profile to first profile if available
        if self.current_profile.is_none() {
            if let Some(first_profile) = self.profiles.keys().next() {
                self.current_profile = Some(first_profile.clone());
            }
        }
    }
}

/// Accessibility statistics
#[derive(Debug, Clone)]
pub struct AccessibilityStatistics {
    /// Total profiles
    pub total_profiles: u32,
    /// Active profiles
    pub active_profiles: u32,
    /// Total settings
    pub total_settings: u32,
    /// Enabled features
    pub enabled_features: u32,
    /// Screen reader profiles
    pub screen_reader_profiles: u32,
    /// Magnification profiles
    pub magnification_profiles: u32,
    /// High contrast profiles
    pub high_contrast_profiles: u32,
}

/// Accessibility export data
#[derive(Debug, Clone)]
pub struct AccessibilityExport {
    /// Profiles
    pub profiles: Vec<AccessibilityProfile>,
    /// Settings
    pub settings: Vec<AccessibilitySetting>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_accessibility_profile_creation() {
        let profile = AccessibilityProfile::new("test".into(), "Test Profile".into());
        assert_eq!(profile.id, "test");
        assert_eq!(profile.name, "Test Profile");
        assert!(!profile.is_assistive_active());
    }
    
    #[test]
    fn test_enable_feature() {
        let mut profile = AccessibilityProfile::default();
        profile.enable_feature(AccessibilityFeature::ScreenReader);
        assert!(profile.screen_reader_active);
        assert!(profile.is_feature_enabled(&AccessibilityFeature::ScreenReader));
        assert!(profile.is_assistive_active());
    }
    
    #[test]
    fn test_disable_feature() {
        let mut profile = AccessibilityProfile::default();
        profile.enable_feature(AccessibilityFeature::HighContrast);
        assert!(profile.high_contrast);
        
        profile.disable_feature(AccessibilityFeature::HighContrast);
        assert!(!profile.high_contrast);
        assert!(!profile.is_feature_enabled(&AccessibilityFeature::HighContrast));
    }
    
    #[test]
    fn test_zoom_level() {
        let mut profile = AccessibilityProfile::default();
        assert_eq!(profile.get_zoom_level(), 1.0);
        
        profile.set_zoom_level(2.0);
        assert_eq!(profile.get_zoom_level(), 2.0);
        assert!(profile.is_feature_enabled(&AccessibilityFeature::Magnification));
        
        profile.set_zoom_level(15.0);
        assert_eq!(profile.get_zoom_level(), 10.0); // Clamped to max
    }
    
    #[test]
    fn test_get_enabled_features() {
        let mut profile = AccessibilityProfile::default();
        profile.enable_feature(AccessibilityFeature::ScreenReader);
        profile.enable_feature(AccessibilityFeature::HighContrast);
        profile.set_zoom_level(2.0);
        
        let features = profile.get_enabled_features();
        assert!(features.contains(&AccessibilityFeature::ScreenReader));
        assert!(features.contains(&AccessibilityFeature::HighContrast));
        assert!(features.contains(&AccessibilityFeature::Magnification));
        assert_eq!(features.len(), 3);
    }
    
    #[test]
    fn test_daemon_creation() {
        let mut daemon = AccessibilityDaemon::new();
        daemon.initialize();
        
        assert!(daemon.get_setting("screen_reader.enabled").is_some());
        assert!(daemon.get_profile("default").is_some());
    }
    
    #[test]
    fn test_profile_management() {
        let mut daemon = AccessibilityDaemon::new();
        daemon.initialize();
        
        let profile = AccessibilityProfile::new("test".into(), "Test".into());
        daemon.add_profile(profile);
        
        assert_eq!(daemon.get_all_profiles().len(), 2);
        assert!(daemon.get_profile("test").is_some());
    }
    
    #[test]
    fn test_current_profile() {
        let mut daemon = AccessibilityDaemon::new();
        daemon.initialize();
        
        assert_eq!(daemon.get_current_profile().unwrap().id, "default");
        
        daemon.enable_feature(AccessibilityFeature::ScreenReader);
        assert!(daemon.is_feature_enabled(&AccessibilityFeature::ScreenReader));
        
        daemon.disable_feature(AccessibilityFeature::ScreenReader);
        assert!(!daemon.is_feature_enabled(&AccessibilityFeature::ScreenReader));
    }
    
    #[test]
    fn test_zoom_level_daemon() {
        let mut daemon = AccessibilityDaemon::new();
        daemon.initialize();
        
        assert_eq!(daemon.get_zoom_level(), 1.0);
        
        daemon.set_zoom_level(2.0);
        assert_eq!(daemon.get_zoom_level(), 2.0);
    }
    
    #[test]
    fn test_statistics() {
        let mut daemon = AccessibilityDaemon::new();
        daemon.initialize();
        
        daemon.enable_feature(AccessibilityFeature::ScreenReader);
        daemon.set_zoom_level(2.0);
        
        let stats = daemon.get_statistics();
        assert_eq!(stats.total_profiles, 1);
        assert_eq!(stats.active_profiles, 1);
        assert!(stats.screen_reader_profiles > 0);
        assert!(stats.magnification_profiles > 0);
    }
    
    #[test]
    fn test_export_import() {
        let mut daemon = AccessibilityDaemon::new();
        daemon.initialize();
        daemon.enable_feature(AccessibilityFeature::HighContrast);
        
        let export = daemon.export_data();
        
        let mut daemon2 = AccessibilityDaemon::new();
        daemon2.import_data(export);
        
        assert!(daemon2.is_feature_enabled(&AccessibilityFeature::HighContrast));
    }
    
    #[test]
    fn test_accessibility_features() {
        let features = vec![
            AccessibilityFeature::ScreenReader,
            AccessibilityFeature::Magnification,
            AccessibilityFeature::HighContrast,
            AccessibilityFeature::KeyboardNavigation,
            AccessibilityFeature::VoiceControl,
        ];
        assert_eq!(features.len(), 5);
    }
}
