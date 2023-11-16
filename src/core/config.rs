use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::error::HinaError;
use crate::core::global::TARGET_MAP;
use crate::event::fs::{MakeNestedDir, Rename};
use crate::event::process::Process;
use crate::event::recycle::{RecycleBin, Remove};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Target {
    Remove(Remove),
    RecycleBin(RecycleBin),
    MakeNestedDir(MakeNestedDir),
    Process(Process),
    Rename(Rename),
    None,
}

#[derive(Debug, Clone)]
pub struct Config {
    target: Target,
    args: Vec<String>,
    flags: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RMRecord {
    file: String,
    src: String,
    delete_time: String,
}

impl Config {
    pub fn add_flag(input: &String, map: &mut HashMap<String, String>) {
        if input.contains("=") {
            let entries: Vec<&str> = input.split("=").collect();
            map.insert(entries[0][1..].to_string(), entries[1].to_string());
        } else {
            map.insert(input[1..].to_string(), "".to_string());
        }
    }

    pub fn build(input: &[String]) -> Result<Config, HinaError> {
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
        let config = Config { target, args, flags };
        return Ok(config);
    }

    pub fn get_target(&self) -> &Target {
        return &self.target;
    }

    pub fn get_args(&self) -> &Vec<String> {
        return &self.args;
    }

    pub fn get_flags(&self) -> &HashMap<String, String> {
        return &self.flags;
    }

    fn parse_flag_and_arg(input: &mut Vec<String>) -> (HashMap<String, String>, Vec<String>) {
        let mut flags: HashMap<String, String> = HashMap::new();
        let mut args = Vec::new();
        let _: Vec<_> = input.iter().map(
            |x| {
                if x.starts_with("-") { Config::add_flag(x, &mut flags) } else { args.push(x.clone()) }
            }
        ).collect();
        return (flags, args);
    }
}

impl RMRecord {
    pub fn from(file: String,
                src: String,
                delete_time: String) -> RMRecord {
        return RMRecord {
            file,
            src,
            delete_time,
        };
    }

    pub fn get_file(&self) -> &String {
        return &self.file;
    }

    pub fn get_src(&self) -> &String {
        return &self.src;
    }

    pub fn get_del_time(&self) -> &String {
        return &self.delete_time;
    }
}