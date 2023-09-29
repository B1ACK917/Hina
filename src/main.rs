mod core;

use std::env;
use std::process;
use crate::core::config::Config;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let arg_num = arg.len();
    dbg!(&arg_num);

    let config = Config::build(&arg).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    dbg!(config);
}