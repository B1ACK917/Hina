use std::env;
use std::process;

use crate::core::config::Config;
use crate::core::executor::Executor;
use crate::core::global::DEBUG;

mod core;
mod event;

fn main() {
    // Collect args
    let arg: Vec<String> = env::args().collect();

    // Build config
    let config = Config::build(&arg).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(-1);
    });

    // Build executor with config
    let executor = Executor::build(config);
    if *DEBUG {
        dbg!(&executor);
    }
    executor.run();
}