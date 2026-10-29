//! SigmaOS Accessibility Daemon entry point.

use std::process;
use sigmaos_accessibility::accessibility::AccessibilityService;

fn main() {
    let service = AccessibilityService::new();
    if let Err(e) = service.run() {
        eprintln!("sigmaos-accessibility: fatal error: {}", e);
        process::exit(1);
    }
}
