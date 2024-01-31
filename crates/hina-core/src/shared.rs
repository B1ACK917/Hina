use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::debug_fn_inline;
use crate::error::HinaError;

#[derive(Serialize, Deserialize, Debug)]
pub struct RMRecord {
    file: String,
    src: String,
    delete_time: String,
}

impl RMRecord {
    pub fn new(file: String, src: String) -> RMRecord {
        let now: DateTime<Local> = Local::now();
        RMRecord {
            file,
            src,
            delete_time: now.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
        }
    }

    pub fn get_file(&self) -> &String {
        debug_fn_inline!();
        return &self.file;
    }

    pub fn get_src(&self) -> &String {
        debug_fn_inline!();
        return &self.src;
    }

    pub fn get_del_time(&self) -> &String {
        debug_fn_inline!();
        return &self.delete_time;
    }
}

pub trait HinaModuleRun: {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &Flag,
           _rm_stack: &mut Vec<RMRecord>,
           _arg: Option<&String>) -> Result<(), HinaError> {
        let err = "Function run not implemented".to_string();
        Err(HinaError::NotImplementedError(err))
    }
}

impl fmt::Debug for dyn HinaModuleRun {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // debug_fn_inline!();
        // write!(f, "{{{:?}}}", stringify!(self))
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Flag {
    pub flags: HashMap<String, String>,
}

impl Flag {
    pub fn parse_bool(&self, symbols: Vec<&str>) -> bool {
        debug_fn_inline!(symbols);
        for symbol in symbols {
            if self.flags.contains_key(symbol) {
                return true;
            }
        }
        return false;
    }

    pub fn parse_string(&self, symbols: Vec<&str>) -> String {
        debug_fn_inline!(symbols);
        for symbol in symbols {
            if self.flags.contains_key(symbol) {
                return self.flags[symbol].clone();
            }
        }
        return String::new();
    }

    pub fn parse_uint(&self, symbols: Vec<&str>) -> usize {
        debug_fn_inline!(symbols);
        for symbol in symbols {
            if self.flags.contains_key(symbol) {
                return self.flags[symbol].clone().parse().unwrap_or(0);
            }
        }
        return 0;
    }
}