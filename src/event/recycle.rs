use std::path::PathBuf;

use chrono::{DateTime, Local};

use crate::core::func;
use crate::core::global::{RAND_STR_LEN, RECYCLE, SPLITTER};

pub fn remove(target: &PathBuf,
              data_path: &PathBuf,
              work_path: &PathBuf,
              rm_stack: &mut Vec<String>) {
    let mut recycle = data_path.clone();
    recycle.push(RECYCLE);

    if !target.exists() {
        return;
    }

    let target_canon = func::get_execute_target(&work_path, &target);
    let mut file_name = String::from(target_canon.file_name().unwrap().to_str().unwrap());
    let file_path = String::from(target_canon.parent().unwrap().to_str().unwrap());
    file_name += &func::gen_rand_str(RAND_STR_LEN);
    recycle.push(file_name.clone());

    let command = String::from(format!("mv {} {}", target.display(), recycle.display()));
    func::execute_command(&command);
    let now: DateTime<Local> = Local::now();
    let rm_log = format!("{}{}{}{}{}",
                         recycle.display(),
                         SPLITTER,
                         file_path,
                         SPLITTER,
                         now.format("%Y-%m-%d %H:%M:%S%.3f"));
    rm_stack.push(rm_log);
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