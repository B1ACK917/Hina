use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::error::HinaError;
use crate::core::global::TARGET_MAP;
use crate::DEBUG;
use crate::debug_fn;
use crate::event::fs::{LinkConvert, MakeNestedDir, Rename};
use crate::event::process::Process;
use crate::event::recycle::{RecycleBin, Remove};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Target {
    Remove(Remove),
    RecycleBin(RecycleBin),
    MakeNestedDir(MakeNestedDir),
    Process(Process),
    Rename(Rename),
    LinkConvert(LinkConvert),
    None,
}

#[derive(Debug, Clone)]
pub struct Flag {
    flags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    target: Target,
    args: Vec<String>,
    flags: Flag,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RMRecord {
    file: String,
    src: String,
    delete_time: String,
}

impl Flag {
    pub fn parse_bool(&self, symbol: &str) -> bool {
        debug_fn!(symbol);
        if self.flags.contains_key(symbol) {
            true
        } else {
            false
        }
    }

    pub fn parse_string(&self, symbol: &str) -> String {
        debug_fn!(symbol);
        if self.flags.contains_key(symbol) {
            self.flags[symbol].clone()
        } else {
            String::new()
        }
    }

    pub fn parse_uint(&self, symbol: &str) -> usize {
        debug_fn!(symbol);
        if self.flags.contains_key(symbol) {
            self.flags[symbol].clone().parse().unwrap_or(0)
        } else {
            0
        }
    }
}

impl Config {
    pub fn add_flag(input: &String, index: usize, map: &mut HashMap<String, String>) {
        debug_fn!(input,map,index);
        if input.contains("=") {
            let entries: Vec<&str> = input.split("=").collect();
            map.insert(entries[0][index..].to_string(), entries[1].to_string());
        } else {
            map.insert(input[index..].to_string(), "".to_string());
        }
    }

    pub fn build(input: &[String]) -> Result<Config, HinaError> {
        debug_fn!(input);
        let target;
        if input.len() < 2 {
            target = Target::None;
        } else {
            if TARGET_MAP.contains_key(input[1].as_str()) {
                target = TARGET_MAP[input[1].as_str()].0.clone();
            } else {
                let err = format!("Illegal action \'{}\'", input[1]);
                return Err(HinaError::ConfigParseError(err));
            }
        }

        let mut args_or_flags = Vec::new();
        if input.len() > 2 {
            for entry in &input[2..] {
                args_or_flags.push(entry.clone());
            }
        }

        let (flags, args) = Config::parse_flag_and_arg(&mut args_or_flags);
        let config = Config {
            target,
            args,
            flags: Flag { flags },
        };
        return Ok(config);
    }

    pub fn get_target(&self) -> &Target {
        debug_fn!();
        return &self.target;
    }

    pub fn get_args(&self) -> &Vec<String> {
        debug_fn!();
        return &self.args;
    }

    pub fn get_flags(&self) -> &Flag {
        debug_fn!();
        return &self.flags;
    }

    fn parse_flag_and_arg(input: &mut Vec<String>) -> (HashMap<String, String>, Vec<String>) {
        debug_fn!();
        let mut flags: HashMap<String, String> = HashMap::new();
        let mut args = Vec::new();
        let _: Vec<_> = input.iter().map(
            |x| {
                if x.starts_with("--") {
                    Config::add_flag(x, 2, &mut flags)
                } else if x.starts_with("-") {
                    Config::add_flag(x, 1, &mut flags)
                } else {
                    args.push(x.clone())
                }
            }
        ).collect();
        return (flags, args);
    }
}

impl RMRecord {
    pub fn from(file: String,
                src: String,
                delete_time: String) -> RMRecord {
        debug_fn!();
        return RMRecord {
            file,
            src,
            delete_time,
        };
    }

    pub fn get_file(&self) -> &String {
        debug_fn!();
        return &self.file;
    }

    pub fn get_src(&self) -> &String {
        debug_fn!();
        return &self.src;
    }

    pub fn get_del_time(&self) -> &String {
        debug_fn!();
        return &self.delete_time;
    }
}