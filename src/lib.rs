#[derive(Debug, Clone)]
pub struct AccessibilityProfile {
    pub screen_reader_active: bool,
    pub zoom_level: f32,
    pub high_contrast: bool,
}

impl Default for AccessibilityProfile {
    fn default() -> Self {
        Self { screen_reader_active: false, zoom_level: 1.0, high_contrast: false }
    }
}

pub struct AccessibilityDaemon;
impl AccessibilityDaemon {
    pub fn is_assistive_active(p: &AccessibilityProfile) -> bool {
        p.screen_reader_active || p.zoom_level > 1.0 || p.high_contrast
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_accessibility_profile() {
        let mut p = AccessibilityProfile::default();
        assert!(!AccessibilityDaemon::is_assistive_active(&p));
        p.zoom_level = 2.0;
        assert!(AccessibilityDaemon::is_assistive_active(&p));
    }
}
