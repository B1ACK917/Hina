use std::io::stdin;
use std::path::PathBuf;

use colored::Colorize;

use hina_core::debug_fn;
use hina_core::error::HinaError;
use hina_core::func::{execute_command, execute_command_in_terminal, split_and_remove_blank};
use hina_core::shared::{Flag, HinaModuleRun, RMRecord};

#[derive(Debug, Clone)]
pub struct RecycleBin;

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
            return Err(HinaError::OutOfIndexError(format!("Index {} is out of recycle bin", index)));
        }
        let record = &rm_stack[index];
        let src = PathBuf::from(record.get_src());
        if src.exists() {
            Err(HinaError::FileExistError(format!("{} already exists, cannot restore", record.get_src())))
        } else {
            let command = String::from(format!("mv \"{}\" \"{}\"", record.get_file(), record.get_src()));
            execute_command(&command)?;
            println!("{} restored", record.get_src());
            Ok(index)
        }
    }

    fn empty(recycle_path: &PathBuf, rm_stack: &mut Vec<RMRecord>) -> Result<(), HinaError> {
        debug_fn!(recycle_path,rm_stack);
        let command = String::from(format!("rm -rf {}/*", recycle_path.display()));
        execute_command(&command)?;
        rm_stack.clear();
        println!("Recycle bin emptied");
        Ok(())
    }
}