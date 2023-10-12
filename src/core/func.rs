use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Stdio;

use execute::{Execute, shell};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use crate::core::global::{DEBUG, RAND_STR_LEN, RECYCLE, RM_STACK, SPLITTER};
use crate::event::process;

pub fn get_home() -> String {
    return env::var("HOME").unwrap();
}

pub fn get_user() -> String {
    return env::var("USER").unwrap();
}

pub fn get_uid() -> String {
    let command = format!("id");
    let output = execute_command(&command);
    let uid = output
        .split(" ")
        .collect::<Vec<&str>>()[0]
        .split("=")
        .collect::<Vec<&str>>()[1]
        .split("(")
        .collect::<Vec<&str>>()[0].to_string();
    return uid;
}

pub fn execute_command(input: &String) -> String {
    let mut command = shell(input);
    command.stdout(Stdio::piped());
    let output = String::from_utf8(command.execute_output().unwrap().stdout).unwrap();
    return output;
}

pub fn get_execute_target(work_path: &PathBuf, input_path: &PathBuf) -> PathBuf {
    let target_canon;
    if input_path.is_absolute() {
        target_canon = input_path.clone();
    } else {
        let mut cur_path = work_path.clone();
        cur_path.push(&input_path);
        if cur_path.exists() {
            target_canon = cur_path.canonicalize().unwrap();
        } else {
            target_canon = cur_path;
        }
    }
    return target_canon;
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
        dbg!(&rm_stack);
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
        dbg!(&rm_stack);
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
        dbg!(&paths);
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

pub fn test() {
    process::read_mem_detail_from_proc(226912);
}