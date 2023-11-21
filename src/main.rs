use std::env;

use crate::core::config::Config;
use crate::core::error::HinaError;
use crate::core::executor::Executor;
use crate::core::global::DEBUG;

mod core;
mod event;

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