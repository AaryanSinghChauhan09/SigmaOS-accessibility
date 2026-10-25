//! Screen reader module.
//!
//! Synthesizes speech and Braille output for accessible UI nodes.
//! Single responsibility: text-to-speech and Braille translation pipeline.
//! Inspired by GNOME Orca.

use crate::accessibility::{AccessibleNode, NodeRole};

/// Output backends supported by the screen reader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputBackend {
    /// Speech-Dispatcher TTS backend
    SpeechDispatcher,
    /// espeak-ng direct backend
    EspeakNg,
    /// Braille display via BrlAPI
    BrlApi,
    /// No-op backend (testing / silent mode)
    Null,
}

/// Configuration for the screen reader engine.
#[derive(Debug, Clone)]
pub struct ScreenReaderConfig {
    pub backend: OutputBackend,
    pub speech_rate: f32,
    pub speech_pitch: f32,
    pub verbosity: Verbosity,
}

/// How much context the screen reader announces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verbosity {
    /// Announce only the node name.
    Brief,
    /// Announce name, role, and description.
    Full,
}

impl Default for ScreenReaderConfig {
    fn default() -> Self {
        ScreenReaderConfig {
            backend: OutputBackend::SpeechDispatcher,
            speech_rate: 1.0,
            speech_pitch: 1.0,
            verbosity: Verbosity::Full,
        }
    }
}

/// Screen reader engine.
///
/// Converts accessibility events into synthesized speech or Braille output
/// according to the configured verbosity and output backend.
pub struct ScreenReader {
    config: ScreenReaderConfig,
}

impl ScreenReader {
    /// Create a new screen reader with the given configuration.
    pub fn new(config: ScreenReaderConfig) -> Self {
        ScreenReader { config }
    }

    /// Announce an accessible node when it receives focus.
    pub fn announce_focus(&self, node: &AccessibleNode) {
        let text = self.build_announcement(node);
        self.speak(&text);
    }

    /// Build the announcement string for a node based on verbosity settings.
    fn build_announcement(&self, node: &AccessibleNode) -> String {
        match self.config.verbosity {
            Verbosity::Brief => node.name.clone(),
            Verbosity::Full => {
                let role_str = role_to_str(&node.role);
                format!("{}: {}. {}", node.name, role_str, node.description)
            }
        }
    }

    /// Send text to the configured output backend.
    fn speak(&self, text: &str) {
        match self.config.backend {
            OutputBackend::Null => {}
            OutputBackend::SpeechDispatcher => {
                // TODO: invoke speech-dispatcher API
                let _ = text;
            }
            OutputBackend::EspeakNg => {
                // TODO: invoke espeak-ng binary / library
                let _ = text;
            }
            OutputBackend::BrlApi => {
                // TODO: write to BrlAPI display
                let _ = text;
            }
        }
    }
}

/// Map a `NodeRole` to a human-readable string.
fn role_to_str(role: &NodeRole) -> &'static str {
    match role {
        NodeRole::Application => "application",
        NodeRole::Window => "window",
        NodeRole::Button => "button",
        NodeRole::TextInput => "text field",
        NodeRole::Label => "label",
        NodeRole::List => "list",
        NodeRole::ListItem => "list item",
        NodeRole::Menu => "menu",
        NodeRole::MenuItem => "menu item",
        NodeRole::Unknown => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accessibility::{AccessibleNode, NodeRole};

    fn test_node() -> AccessibleNode {
        AccessibleNode {
            id: 1,
            role: NodeRole::Button,
            name: "Close".to_string(),
            description: "Close the window".to_string(),
            children: vec![],
        }
    }

    #[test]
    fn brief_announcement() {
        let cfg = ScreenReaderConfig {
            backend: OutputBackend::Null,
            verbosity: Verbosity::Brief,
            ..Default::default()
        };
        let sr = ScreenReader::new(cfg);
        // Should not panic; output goes to Null backend
        sr.announce_focus(&test_node());
    }

    #[test]
    fn full_announcement() {
        let cfg = ScreenReaderConfig {
            backend: OutputBackend::Null,
            verbosity: Verbosity::Full,
            ..Default::default()
        };
        let sr = ScreenReader::new(cfg);
        sr.announce_focus(&test_node());
    }
}
