use std::path::PathBuf;
use std::process::Command;

use chrono::{DateTime, Local};

use crate::core::func;
use crate::core::global::{RAND_STR_LEN, RECYCLE, SPLITTER};

pub fn remove(target: &String,
              data_path: &PathBuf,
              work_path: &PathBuf,
              rm_stack: &mut Vec<String>) {
    let mut recycle = data_path.clone();
    recycle.push(RECYCLE);
    let rm_target = PathBuf::from(target);
    let mut file_name: String;
    let file_path: String;

    if !rm_target.exists() {
        return;
    }

    if rm_target.is_absolute() {
        file_name = String::from(rm_target.file_name().unwrap().to_str().unwrap());
        file_path = String::from(rm_target.parent().unwrap().to_str().unwrap());
    } else {
        file_name = target.clone();
        let mut cur_path = work_path.clone();
        cur_path.push(&target);
        file_path = String::from(cur_path.parent().unwrap().to_str().unwrap());
    }
    file_name += &func::gen_rand_str(RAND_STR_LEN);
    recycle.push(file_name.clone());

    Command::new("mv")
        .arg(target)
        .arg(recycle.clone())
        .output()
        .unwrap();
    let now: DateTime<Local> = Local::now();
    let rm_log = format!("{}{}{}{}{}",
                         recycle.display().to_string(),
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
        Command::new("mv")
            .arg(rm_tar)
            .arg(rm_src)
            .output()
            .unwrap();
        println!("{} restored", rm_src.display());
        rm_stack.remove(index as usize);
    }
}

pub fn empty_trash_bin(data_path: &PathBuf,
                       rm_stack: &mut Vec<String>) {
    let mut recycle = data_path.clone();
    recycle.push(RECYCLE);
    Command::new("rm")
        .arg("-r")
        .arg("-f")
        .arg(&recycle)
        .output()
        .unwrap();
    Command::new("mkdir")
        .arg(&recycle)
        .output()
        .unwrap();
    rm_stack.clear();
    println!("Recycle bin emptied");
}