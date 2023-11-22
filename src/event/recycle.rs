use std::io::stdin;
use std::path::PathBuf;

use chrono::{DateTime, Local};

use crate::{DEBUG, debug_fn};
use crate::core::config::{Flag, RMRecord};
use crate::core::error::HinaError;
use crate::core::error::HinaError::{FileExistError, OutOfIndexError};
use crate::core::func;
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
           _target: &PathBuf,
           _arg_num: usize,
    ) -> Result<(), HinaError> {
        debug_fn!(_work_path,_data_path,_recycle_path,_user,_uid,_flags,_rm_stack,_target,_arg_num);
        if _arg_num < 1 {
            return Ok(());
        }
        let mut recycle_bin = _recycle_path.clone();
        let file_name = func::gen_rand_str(RAND_STR_LEN);
        recycle_bin.push(file_name.clone());

        let command = String::from(format!("mv \"{}\" \"{}\"", _target.display(), recycle_bin.display()));
        func::execute_command(&command)?;
        let now: DateTime<Local> = Local::now();
        _rm_stack.push(RMRecord::from(
            recycle_bin.display().to_string(),
            _target.display().to_string(),
            now.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
        ));
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
           _target: &PathBuf,
           _arg_num: usize,
    ) -> Result<(), HinaError> {
        debug_fn!(_work_path,_data_path,_recycle_path,_user,_uid,_flags,_rm_stack,_target,_arg_num);
        let _list = _flags.parse_bool(vec!["ls", "list"]);
        let _restore = _flags.parse_bool(vec!["rs", "restore"]);
        let _empty = _flags.parse_bool(vec!["ept", "empty"]);
        let _help = _flags.parse_bool(vec!["help"]);
        if _help {
            RecycleBin::print_help()?;
        }
        if _list {
            RecycleBin::show(_rm_stack)?
        }
        if _restore {
            RecycleBin::show(_rm_stack)?;
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let index: isize = input.trim().parse().unwrap_or(-1);
            RecycleBin::restore(_rm_stack, index)?
        }
        if _empty {
            RecycleBin::empty(_recycle_path, _rm_stack)?
        }
        Ok(())
    }
}

impl RecycleBin {
    fn print_help() -> Result<(), HinaError> {
        Ok(())
    }

    fn show(rm_stack: &Vec<RMRecord>) -> Result<(), HinaError> {
        debug_fn!(rm_stack);
        for (i, record) in rm_stack.iter().enumerate() {
            println!("{}: File: {}, delete-time: {}", i, record.get_src(), record.get_del_time());
        }
        Ok(())
    }

    fn restore(rm_stack: &mut Vec<RMRecord>, index: isize) -> Result<(), HinaError> {
        debug_fn!(rm_stack,index);
        if index < 0 || index as usize >= rm_stack.len() {
            return Err(OutOfIndexError(format!("Index {} is out of recycle bin", index)));
        }
        let record = &rm_stack[index as usize];
        let src = PathBuf::from(record.get_src());
        if src.exists() {
            Err(FileExistError(format!("{} already exists, cannot restore", record.get_src())))
        } else {
            let command = String::from(format!("mv \"{}\" \"{}\"", record.get_file(), record.get_src()));
            func::execute_command(&command)?;
            println!("{} restored", record.get_src());
            rm_stack.remove(index as usize);
            Ok(())
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