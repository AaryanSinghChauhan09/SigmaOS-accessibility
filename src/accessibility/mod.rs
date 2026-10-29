// SigmaOS Accessibility & Screen Reader Subsystem
use std::fmt;

pub mod framework;
pub mod keyboard;
pub mod magnifier;
pub mod screenreader;

pub use framework::*;
pub use keyboard::*;
pub use magnifier::*;
pub use screenreader::{
    BrailleDisplay, ScreenReader, ScreenReaderError, SimpleBrailleDisplay, SimpleScreenReader,
    SimpleVoice, Voice, VoiceGender, VoiceID,
};

pub struct AccessibilityEngine;

impl AccessibilityEngine {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AccessibilityEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur in the accessibility service.
#[derive(Debug)]
pub enum AccessibilityServiceError {
    InitFailed(String),
    BusFailed(String),
    RegistryUnavailable,
}

impl fmt::Display for AccessibilityServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessibilityServiceError::InitFailed(msg) => write!(f, "Initialization failed: {}", msg),
            AccessibilityServiceError::BusFailed(msg) => write!(f, "D-Bus connection failed: {}", msg),
            AccessibilityServiceError::RegistryUnavailable => write!(f, "AT-SPI2 registry is unavailable"),
        }
    }
}

/// A focusable UI node exposed to the accessibility tree.
#[derive(Debug, Clone)]
pub struct AccessibleNode {
    pub id: u64,
    pub role: NodeRole,
    pub name: String,
    pub description: String,
    pub children: Vec<u64>,
}

/// The semantic role of an accessible node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeRole {
    Application,
    Window,
    Button,
    TextInput,
    Label,
    List,
    ListItem,
    Menu,
    MenuItem,
    Unknown,
}

/// Core accessibility service.
pub struct AccessibilityService {
    nodes: Vec<AccessibleNode>,
}

impl AccessibilityService {
    pub fn new() -> Self {
        AccessibilityService { nodes: Vec::new() }
    }

    pub fn run(&self) -> Result<(), AccessibilityServiceError> {
        Ok(())
    }

    pub fn register_node(&mut self, node: AccessibleNode) {
        self.nodes.push(node);
    }

    pub fn find_node(&self, id: u64) -> Option<&AccessibleNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for AccessibilityService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod service_tests {
    use super::*;

    #[test]
    fn test_register_and_find_accessible_node() {
        let mut svc = AccessibilityService::new();
        let node = AccessibleNode {
            id: 1,
            role: NodeRole::Button,
            name: "Submit".to_string(),
            description: "Submit action".to_string(),
            children: vec![],
        };
        svc.register_node(node);
        assert_eq!(svc.node_count(), 1);
        assert!(svc.find_node(1).is_some());
        assert_eq!(svc.find_node(1).unwrap().name, "Submit");
        assert!(svc.find_node(99).is_none());
    }
}
