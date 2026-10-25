//! SigmaOS Accessibility Daemon entry point.

use std::process;

fn main() {
    env_logger::init();
    let service = sigmaccess::AccessibilityService::new();
    if let Err(e) = service.run() {
        eprintln!("sigmaccess: fatal error: {}", e);
        process::exit(1);
    }
}
