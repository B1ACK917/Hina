use std::io::stdin;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use colored::Colorize;

use crate::{DEBUG, debug_fn, debug_info};
use crate::core::config::{Flag, RMRecord};
use crate::core::error::HinaError;
use crate::core::error::HinaError::{FileExistError, OutOfIndexError};
use crate::core::func;
use crate::core::func::{execute_command_in_terminal, get_execute_target, split_and_remove_blank};
use crate::core::global::RAND_STR_LEN;
use crate::event::base::HinaModuleRun;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Remove;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RecycleBin;

impl HinaModuleRun for Remove {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &Flag,
           _rm_stack: &mut Vec<RMRecord>,
           _arg: Option<&String>,
    ) -> Result<(), HinaError> {
        debug_fn!(_work_path,_data_path,_recycle_path,_user,_uid,_flags,_rm_stack,_arg);
        let _help = _flags.parse_bool(vec!["help"]);
        if _help {
            Remove::print_help()?;
            return Ok(());
        }
        match _arg {
            None => {}
            Some(arg) => {
                let remove_target = get_execute_target(_work_path, &PathBuf::from(arg))?;
                let mut recycle_bin = _recycle_path.clone();
                let file_name = func::gen_rand_str(RAND_STR_LEN);
                recycle_bin.push(file_name.clone());

                let command = String::from(format!("mv \"{}\" \"{}\"", remove_target.display(), recycle_bin.display()));
                func::execute_command(&command)?;
                let now: DateTime<Local> = Local::now();
                _rm_stack.push(RMRecord::from(
                    recycle_bin.display().to_string(),
                    remove_target.display().to_string(),
                    now.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl Remove {
    fn print_help() -> Result<(), HinaError> {
        debug_fn!();
        execute_command_in_terminal("man", vec!["hina-rm"])?;
        Ok(())
    }
}

impl HinaModuleRun for RecycleBin {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &Flag,
           _rm_stack: &mut Vec<RMRecord>,
           _arg: Option<&String>,
    ) -> Result<(), HinaError> {
        debug_fn!(_work_path,_data_path,_recycle_path,_user,_uid,_flags,_rm_stack,_arg);
        let _help = _flags.parse_bool(vec!["help"]);
        if _help {
            RecycleBin::print_help()?;
            return Ok(());
        }
        let _list = _flags.parse_bool(vec!["ls", "list"]);
        let _restore = _flags.parse_bool(vec!["rs", "restore"]);
        let _empty = _flags.parse_bool(vec!["ept", "empty"]);
        if _list {
            RecycleBin::show(_rm_stack)?;
            return Ok(());
        }
        if _restore {
            RecycleBin::restore(_rm_stack)?;
            return Ok(());
        }
        if _empty {
            RecycleBin::empty(_recycle_path, _rm_stack)?;
            return Ok(());
        }
        Ok(())
    }
}

impl RecycleBin {
    fn print_help() -> Result<(), HinaError> {
        debug_fn!();
        execute_command_in_terminal("man", vec!["hina-rb"])?;
        Ok(())
    }

    fn show(rm_stack: &Vec<RMRecord>) -> Result<(), HinaError> {
        debug_fn!(rm_stack);
        for (i, record) in rm_stack.iter().enumerate() {
            println!("{}: File: {}, delete-time: {}", i, record.get_src(), record.get_del_time());
        }
        Ok(())
    }

    fn restore(_rm_stack: &mut Vec<RMRecord>) -> Result<(), HinaError> {
        debug_fn!(_rm_stack);
        RecycleBin::show(_rm_stack)?;

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let indices_str = split_and_remove_blank(&input, ",")?;
        let indices: Vec<_> = indices_str
            .into_iter()
            .filter_map(|s| s.parse().ok())
            .collect();
        let removed: Vec<_> = indices
            .into_iter()
            .filter_map(|i| match RecycleBin::restore_index(_rm_stack, i) {
                Ok(i) => { Some(i) }
                Err(err) => {
                    println!("{:?}", err);
                    None
                }
            })
            .collect();
        for i in removed.into_iter().rev() {
            if i < _rm_stack.len() {
                _rm_stack.remove(i);
            }
        }
        return Ok(());
    }

    fn restore_index(rm_stack: &Vec<RMRecord>, index: usize) -> Result<usize, HinaError> {
        debug_fn!(rm_stack,index);
        if index >= rm_stack.len() {
            return Err(OutOfIndexError(format!("Index {} is out of recycle bin", index)));
        }
        let record = &rm_stack[index];
        let src = PathBuf::from(record.get_src());
        if src.exists() {
            Err(FileExistError(format!("{} already exists, cannot restore", record.get_src())))
        } else {
            let command = String::from(format!("mv \"{}\" \"{}\"", record.get_file(), record.get_src()));
            func::execute_command(&command)?;
            println!("{} restored", record.get_src());
            Ok(index)
        }
    }

    fn empty(recycle_path: &PathBuf, rm_stack: &mut Vec<RMRecord>) -> Result<(), HinaError> {
        debug_fn!(recycle_path,rm_stack);
        let command = String::from(format!("rm -rf {}/*", recycle_path.display()));
        func::execute_command(&command)?;
        rm_stack.clear();
        println!("Recycle bin emptied");
        Ok(())
    }
}