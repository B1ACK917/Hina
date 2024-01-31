use std::collections::HashMap;

use colored::Colorize;

use hina_core::debug_fn;
use hina_core::error::HinaError;
use hina_core::shared::{Flag, HinaModuleRun};
use hina_lc::LinkConvert;
use hina_mkndir::MakeNestedDir;
use hina_ph::PlaceHold;
use hina_ps::Process;
use hina_rb::RecycleBin;
use hina_rm::Remove;
use hina_rn::Rename;

#[derive(Debug)]
pub struct Config {
    module: Box<dyn HinaModuleRun>,
    args: Vec<String>,
    flags: Flag,
}


impl Config {
    fn add_flag(input: &String, index: usize, map: &mut HashMap<String, String>) {
        debug_fn!(input,map,index);
        if input.contains("=") {
            let entries: Vec<&str> = input.split("=").collect();
            map.insert(entries[0][index..].to_string(), entries[1].to_string());
        } else {
            map.insert(input[index..].to_string(), "".to_string());
        }
    }

    fn module(input: &str) -> Option<Box<dyn HinaModuleRun>> {
        if input == "rm" {
            return Some(Box::new(Remove));
        } else if input == "rb" {
            return Some(Box::new(RecycleBin));
        } else if input == "rn" {
            return Some(Box::new(Rename));
        } else if input == "mkndir" {
            return Some(Box::new(MakeNestedDir));
        } else if input == "lc" {
            return Some(Box::new(LinkConvert));
        } else if input == "ps" {
            return Some(Box::new(Process));
        }
        None
    }

    pub fn build(input: &[String]) -> Result<Config, HinaError> {
        debug_fn!(input);
        let mut target: Box<dyn HinaModuleRun> = Box::new(PlaceHold);
        let index;
        let need_parse;
        if input.len() < 2 {
            need_parse = false;
            index = 0;
        } else if input.len() == 2 {
            if input[1].starts_with("-") {
                need_parse = true;
                index = 1;
            } else if let Some(value) = Config::module(input[1].as_str()) {
                target = value;
                need_parse = false;
                index = 0;
            } else {
                let err = format!("Illegal action \'{}\'", input[1]);
                return Err(HinaError::ConfigParseError(err));
            }
        } else {
            if let Some(value) = Config::module(input[1].as_str()) {
                target = value;
                need_parse = true;
                index = 2;
            } else {
                let err = format!("Illegal action \'{}\'", input[1]);
                return Err(HinaError::ConfigParseError(err));
            }
        }

        let mut args_or_flags = Vec::new();
        if need_parse {
            for entry in &input[index..] {
                args_or_flags.push(entry.clone());
            }
        }

        let (flags, args) = Config::parse_flag_and_arg(&mut args_or_flags);
        let config = Config {
            module: target,
            args,
            flags: Flag { flags },
        };
        return Ok(config);
    }

    pub fn get_target(&self) -> &Box<dyn HinaModuleRun> {
        debug_fn!();
        return &self.module;
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
