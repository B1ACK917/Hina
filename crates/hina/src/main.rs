use std::env;

use colored::Colorize;

use hina::config::Config;
use hina::executor::Executor;
use hina_core::debug_var;
use hina_core::error::HinaError;

fn main() -> Result<(), HinaError> {
    // Collect args
    let arg: Vec<String> = env::args().collect();

    // Build config
    let config = Config::build(&arg)?;

    // Build executor with config
    let executor = Executor::build(config)?;
    debug_var!(executor);
    executor.run()?;
    Ok(())
}