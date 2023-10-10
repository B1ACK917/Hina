use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use chrono::{DateTime, Local};

use crate::core::global::{DEBUG, RAND_STR_LEN, RECYCLE, RM_STACK, SPLITTER};

pub fn get_home() -> String {
    return env::var("HOME").unwrap();
}

pub fn get_user() -> String {
    return env::var("USER").unwrap();
}

pub fn init_data_dir(data_path: &PathBuf) {
    let mut data_path_ = data_path.clone();
    if !data_path.exists() {
        if *DEBUG {
            println!("Running first time, init {}", data_path.display());
        }
        fs::create_dir(&data_path_).unwrap();

        data_path_.push(RECYCLE);
        fs::create_dir(&data_path_).unwrap();
        data_path_.pop();

        data_path_.push(RM_STACK);
        File::create(data_path_).unwrap();
    } else {
        if *DEBUG {
            println!("{} exists, skip initiation", data_path.display());
        }
    }
}

pub fn load_rm_stack(data_path: &PathBuf) -> Vec<String> {
    let mut data_path_ = data_path.clone();
    data_path_.push(RM_STACK);
    let mut file = File::open(data_path_).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let rm_stack: Vec<String> = contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if *DEBUG {
        dbg!(rm_stack.clone());
    }

    return rm_stack;
}

pub fn save_rm_stack(data_path: &PathBuf,
                     rm_stack: &Vec<String>) {
    let mut data_path_ = data_path.clone();
    data_path_.push(RM_STACK);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(data_path_).unwrap();
    for content in rm_stack {
        file.write(content.as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }

    if *DEBUG {
        dbg!(rm_stack.clone());
    }
}

pub fn show_rm_stack(rm_stack: &Vec<String>) -> Vec<(PathBuf, PathBuf)> {
    let mut paths: Vec<(PathBuf, PathBuf)> = Vec::new();
    for (i, rm_log) in rm_stack.iter().enumerate() {
        let record: Vec<String> = rm_log
            .split(SPLITTER)
            .map(|s| s.to_string())
            .collect();
        let file = PathBuf::from(&record[0]);
        let mut path = PathBuf::from(&record[1]);
        let filename = file.file_name().unwrap().to_str().unwrap();
        path.push(&filename[..filename.len() - (RAND_STR_LEN as usize)]);
        paths.push((file.clone(), path.clone()));
        println!("{}: {}\tDelete Time: {}",
                 i,
                 &record[0][..record[0].len() - (RAND_STR_LEN as usize)],
                 record[2]);
    }
    if *DEBUG {
        dbg!(paths.clone());
    }
    return paths;
}

pub fn gen_rand_str(len: u8) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len as usize)
        .map(char::from)
        .collect();
    return rand_string;
}

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
    file_name += &gen_rand_str(RAND_STR_LEN);
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
}

pub fn show_user_all_process(user: &String) {
    let all_process = Command::new("ps")
        .arg("-ef") // Should replace to -aux
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = Command::new("grep")
        .arg(user)
        .stdin(all_process.stdout.unwrap())
        .output()
        .unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
}

pub fn show_user_spec_process(user: &String,
                              process_name: &String) {
    let all_process = Command::new("ps")
        .arg("-ef")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = Command::new("grep")
        .arg(user)
        .stdin(all_process.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = Command::new("grep")
        .arg(process_name)
        .stdin(output.stdout.unwrap())
        .output()
        .unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
}