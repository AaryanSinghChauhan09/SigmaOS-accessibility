use clap::Parser;
use accessibility::accessibility;

#[derive(Parser)]
struct Cli;

fn main() -> anyhow::Result<()> {
    let config = accessibility::Config::default();
    let mut component = accessibility::new(config);
    component.initialize()?;
    println!("SigmaOS-accessibility ready");
    Ok(())
}
