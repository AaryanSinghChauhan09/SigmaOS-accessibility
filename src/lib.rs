use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Config { pub enabled: bool }
impl Default for Config { fn default() -> Self { Self { enabled: true } } }

pub struct accessibility { config: Config }
impl accessibility {
    pub fn new(config: Config) -> Self { Self { config } }
    pub fn initialize(&mut self) -> Result<()> { Ok(()) }
}
