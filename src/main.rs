mod algorithms;
mod app;
mod crossterm;
mod ui;

use crate::crossterm::run;
use argh::FromArgs;
use std::{error::Error, time::Duration};

/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    run(tick_rate)?;
    Ok(())
}