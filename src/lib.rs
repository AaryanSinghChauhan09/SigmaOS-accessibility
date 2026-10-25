//! SigmaOS Accessibility Library
//!
//! Provides accessibility services including screen reader support,
//! AT-SPI2 bridge integration, and assistive technology interfaces.
//! Inspired by GNOME Accerciser and Orca.

pub mod accessibility;
pub mod screen_reader;

pub use accessibility::AccessibilityService;
pub use screen_reader::ScreenReader;
