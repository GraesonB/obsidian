use anyhow::{Context, Result};
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::{fs::File, path::Path};

// Two ways of doing trait bounds:
// 1. fn func_name<T: trait>(argument: T)
// 2. fn func_name(argument: impl trait)
pub fn create_logger<T: AsRef<Path>>(path: T) -> Result<()> {
    let name = path.as_ref().display().to_string();
    let error_message = format!("Failed to create log file named {}", name);
    let file = File::create(path).context(error_message)?;
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(LevelFilter::max(), Config::default(), file),
    ])?;
    Ok(())
}

