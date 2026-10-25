//! Accessibility service module.
//!
//! Implements the AT-SPI2 accessibility registry bridge for SigmaOS,
//! exposing application UI trees to assistive technologies.
//! Single responsibility: AT-SPI2 registration and event dispatch.

use std::fmt;

/// Errors that can occur in the accessibility service.
#[derive(Debug)]
pub enum AccessibilityError {
    InitFailed(String),
    BusFailed(String),
    RegistryUnavailable,
}

impl fmt::Display for AccessibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessibilityError::InitFailed(msg) => write!(f, "Initialization failed: {}", msg),
            AccessibilityError::BusFailed(msg) => write!(f, "D-Bus connection failed: {}", msg),
            AccessibilityError::RegistryUnavailable => write!(f, "AT-SPI2 registry is unavailable"),
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
///
/// Registers with the AT-SPI2 bus, maintains the accessible node tree,
/// and dispatches focus / text-change events to listening assistive technologies.
pub struct AccessibilityService {
    nodes: Vec<AccessibleNode>,
}

impl AccessibilityService {
    /// Construct a new, unstarted accessibility service.
    pub fn new() -> Self {
        AccessibilityService { nodes: Vec::new() }
    }

    /// Start the service event loop.
    ///
    /// Returns `Ok(())` when the loop exits cleanly, or an error on failure.
    pub fn run(&self) -> Result<(), AccessibilityError> {
        self.init_bus()?;
        self.event_loop()
    }

    /// Register with the AT-SPI2 session bus.
    fn init_bus(&self) -> Result<(), AccessibilityError> {
        // TODO: connect to D-Bus AT-SPI2 registry interface
        Ok(())
    }

    /// Process incoming AT-SPI2 events until shutdown is requested.
    fn event_loop(&self) -> Result<(), AccessibilityError> {
        // TODO: implement event processing loop
        Ok(())
    }

    /// Register an accessible node into the tree.
    pub fn register_node(&mut self, node: AccessibleNode) {
        self.nodes.push(node);
    }

    /// Look up a node by its unique ID.
    pub fn find_node(&self, id: u64) -> Option<&AccessibleNode> {
        self.nodes.iter().find(|n| n.id == id)
    }
}

impl Default for AccessibilityService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_find_node() {
        let mut svc = AccessibilityService::new();
        let node = AccessibleNode {
            id: 1,
            role: NodeRole::Button,
            name: "OK".to_string(),
            description: "Confirm action".to_string(),
            children: vec![],
        };
        svc.register_node(node);
        assert!(svc.find_node(1).is_some());
        assert!(svc.find_node(99).is_none());
    }
}
