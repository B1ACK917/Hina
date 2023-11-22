use std::path::PathBuf;

use crate::{DEBUG, debug_fn};
use crate::core::config::{Flag, RMRecord};
use crate::core::error::HinaError;
use crate::core::global::HELP_DICT;
use crate::event::base::HinaModuleRun;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PlaceHold;

impl HinaModuleRun for PlaceHold {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &Flag,
           _rm_stack: &mut Vec<RMRecord>,
           _target: &PathBuf,
           _arg_num: usize,
    ) -> Result<(), HinaError> {
        debug_fn!(_work_path,_data_path,_recycle_path,_user,_uid,_flags,_rm_stack,_target,_arg_num);
        let help = _flags.parse_bool(vec!["h", "help"]);
        let version = _flags.parse_bool(vec!["v", "version"]);
        if help {
            PlaceHold::print_help()?;
        }
        if version {
            PlaceHold::print_version()?;
        }
        Ok(())
    }
}

impl PlaceHold {
    fn print_help() -> Result<(), HinaError> {
        debug_fn!();
        println!("Usage: hina [-v | --version] [-h | --help] <module> [<params>]");
        println!();
        println!("These are common Hina commands used in various situations:");
        println!();
        for (help_situation, operations) in &*HELP_DICT {
            println!("{}", help_situation);
            for (module, desc) in operations {
                println!("\t{}\t{} (See also: hina {} --help)", module, desc, module);
            }
            println!();
        }
        Ok(())
    }

    fn print_version() -> Result<(), HinaError> {
        debug_fn!();
        const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
        println!("Hina version {}", VERSION.unwrap_or("unknown"));
        Ok(())
    }
}