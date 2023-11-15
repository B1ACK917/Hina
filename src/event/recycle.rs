use std::path::PathBuf;

use chrono::{DateTime, Local};

use crate::core::error::HinaError;
use crate::core::executor::ExecFlag;
use crate::core::func;
use crate::core::global::{RAND_STR_LEN, RECYCLE, SPLITTER};
use crate::event::base::HinaModuleRun;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Remove;

impl HinaModuleRun for Remove {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &ExecFlag,
           _rm_stack: &mut Vec<String>,
           _target: &PathBuf,
    ) -> Result<(), HinaError> {
        let mut recycle_bin = _recycle_path.clone();
        let mut file_name = String::from(_target.file_name().unwrap().to_str().unwrap());
        let file_path = String::from(_target.parent().unwrap().to_str().unwrap());
        file_name += &func::gen_rand_str(RAND_STR_LEN);
        recycle_bin.push(file_name.clone());

        let command = String::from(format!("mv \"{}\" \"{}\"", _target.display(), recycle_bin.display()));
        func::execute_command(&command)?;
        let now: DateTime<Local> = Local::now();
        let rm_log = format!("{}{}{}{}{}",
                             recycle_bin.display(),
                             SPLITTER,
                             file_path,
                             SPLITTER,
                             now.format("%Y-%m-%d %H:%M:%S%.3f"));
        _rm_stack.push(rm_log);
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RecycleBin;

impl HinaModuleRun for RecycleBin {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &ExecFlag,
           _rm_stack: &mut Vec<String>,
           _target: &PathBuf
    ) -> Result<(), HinaError> {
        todo!()
    }
}

pub fn restore(rm_paths: Vec<(PathBuf, PathBuf)>,
               index: i8,
               rm_stack: &mut Vec<String>) {
    if index < 0 || index as usize >= rm_paths.len() {
        return;
    }
    let (rm_tar, rm_src) = &rm_paths[index as usize];
    if rm_src.exists() {
        println!("{} already exists, cannot restore", rm_src.display());
    } else {
        let command = String::from(format!("mv {} {}", rm_tar.display(), rm_src.display()));
        func::execute_command(&command);
        println!("{} restored", rm_src.display());
        rm_stack.remove(index as usize);
    }
}

pub fn empty_trash_bin(data_path: &PathBuf,
                       rm_stack: &mut Vec<String>) {
    let mut recycle = data_path.clone();
    recycle.push(RECYCLE);
    let command = String::from(format!("rm -rf {}/*", recycle.display()));
    func::execute_command(&command);
    rm_stack.clear();
    println!("Recycle bin emptied");
}