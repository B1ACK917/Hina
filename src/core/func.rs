use std::cmp::max;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use colored::Colorize;
use execute::{Execute, shell};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use crate::{debug_fn, debug_info, debugln};
use crate::core::config::RMRecord;
use crate::core::error::HinaError;
use crate::core::error::HinaError::{DirCreateError, FileCreateError, FileOpenError, FileWriteError};
use crate::core::global::{DEBUG, RECYCLE, RM_STACK};

fn read_var(var_name: &str) -> Result<String, HinaError> {
    // Read variable from system variables
    debug_fn!(var_name);
    // debugln!("Calling read_var(), trying to locate VAR \"{}\"",var_name);
    match env::var(var_name) {
        Ok(val) => { Ok(val) }
        Err(_) => {
            let err = format!("Cannot locate variable {}", var_name);
            Err(HinaError::VarError(err))
        }
    }
}

pub fn get_home() -> Result<String, HinaError> {
    debug_fn!();
    Ok(read_var("HOME")?)
}

pub fn get_user() -> Result<String, HinaError> {
    debug_fn!();
    Ok(read_var("USER")?)
}

pub fn get_current_path() -> Result<PathBuf, HinaError> {
    // Get Hina working path
    debug_fn!();
    match env::current_dir() {
        Ok(cur) => { Ok(cur) }
        Err(err) => {
            Err(HinaError::WorkPathError(err.to_string()))
        }
    }
}

pub fn get_uid() -> Result<String, HinaError> {
    // Use unix shell "id" to get user id
    debug_fn!();
    let command = format!("id");
    let output = execute_command(&command)?;
    let uid = output
        .split(" ")
        .collect::<Vec<&str>>()[0]
        .split("=")
        .collect::<Vec<&str>>()[1]
        .split("(")
        .collect::<Vec<&str>>()[0].to_string();
    Ok(uid)
}

pub fn execute_command(input: &String) -> Result<String, HinaError> {
    // Shell utils, for running a unix shell
    debug_fn!(input);
    let mut command = shell(input);
    command.stdout(Stdio::piped());
    let command_out_utf_8 = match command.execute_output() {
        Ok(output) => { output.stdout }
        Err(err) => { return Err(HinaError::CommandExecError(err.to_string())); }
    };
    match String::from_utf8(command_out_utf_8) {
        Ok(output) => { Ok(output) }
        Err(err) => { Err(HinaError::CommandParseError(err.to_string())) }
    }
}

pub fn execute_command_in_terminal(cmd: &str, args: Vec<&str>) -> Result<(), HinaError> {
    // Execute a command with interactive
    debug_fn!(cmd,args);
    let mut cmd = Command::new(cmd);
    cmd.args(args);

    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    let mut child = cmd.spawn().expect("Failed to execute command");
    let status = child.wait().expect("Failed to wait for command");

    if status.success() {
        Ok(())
    } else {
        Err(HinaError::CommandExecError(String::new()))
    }
}

pub fn parse_path_or(input_path: Option<&String>, default: &str) -> Result<PathBuf, HinaError> {
    debug_fn!(input_path,default);
    match input_path {
        None => {
            Ok(PathBuf::from(default))
        }
        Some(path) => {
            Ok(PathBuf::from(path))
        }
    }
}

pub fn get_execute_target(work_path: &PathBuf, input_path: &PathBuf) -> Result<PathBuf, HinaError> {
    // Parse real execute target from input and return the abs path
    debug_fn!(work_path,input_path);
    let mut target;
    if input_path.is_absolute() {
        target = input_path.clone();
    } else {
        target = work_path.clone();
        target.push(&input_path);
    }
    if target.exists() {
        if !target.is_symlink() {
            match target.canonicalize() {
                Ok(path) => { Ok(path) }
                Err(_) => {
                    let err = format!("Unable to open {}", target.display());
                    Err(HinaError::BadFileError(err))
                }
            }
        } else {
            Ok(target)
        }
    } else {
        let err = format!("Unable to locate {}", target.display());
        Err(HinaError::FileNotExistError(err))
    }
}

pub fn init_data_dir(data_path: &PathBuf) -> Result<(), HinaError> {
    // Hina utils, for initializing the Hina data dir containing recycle bin and etc.
    debug_fn!(data_path);
    let mut mut_data_path = data_path.clone();
    if !mut_data_path.exists() {
        debugln!("Running first time, init {}", mut_data_path.display());

        // Create Hina data dir
        match fs::create_dir(&mut_data_path) {
            Ok(_) => {}
            Err(err) => { return Err(DirCreateError(err.to_string())); }
        }

        // Create Hina recycle bin
        mut_data_path.push(RECYCLE);
        match fs::create_dir(&mut_data_path) {
            Ok(_) => {}
            Err(err) => { return Err(DirCreateError(err.to_string())); }
        }

        // RM_STACK for recording the removed file source
        mut_data_path.pop();
        mut_data_path.push(RM_STACK);
        match File::create(&mut_data_path) {
            Ok(_) => {}
            Err(err) => { return Err(FileCreateError(err.to_string())); }
        }
    } else {
        debugln!("{} exists, skip initiation", data_path.display());
    }
    Ok(())
}

pub fn load_rm_stack(data_path: &PathBuf) -> Result<Vec<RMRecord>, HinaError> {
    // Load the RM_STACK for recycle bin
    debug_fn!(data_path);
    let mut rm_stack_path = data_path.clone();
    rm_stack_path.push(RM_STACK);
    let file = match File::open(rm_stack_path) {
        Ok(file) => { file }
        Err(err) => { return Err(FileOpenError(err.to_string())); }
    };
    let reader = BufReader::new(file);
    let rm_stack = match serde_json::from_reader(reader) {
        Ok(rm_stack) => { rm_stack }
        Err(_) => { Vec::new() }
    };
    Ok(rm_stack)
}

pub fn save_rm_stack(data_path: &PathBuf,
                     rm_stack: &Vec<RMRecord>) -> Result<(), HinaError> {
    // Write RM_STACK back to disk
    debug_fn!(rm_stack);
    let mut rm_stack_path = data_path.clone();
    rm_stack_path.push(RM_STACK);
    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(rm_stack_path) {
        Ok(file) => { file }
        Err(err) => { return Err(FileOpenError(err.to_string())); }
    };
    let writer = BufWriter::new(file);
    match serde_json::to_writer(writer, rm_stack) {
        Ok(_) => {}
        Err(err) => { return Err(FileWriteError(err.to_string())); }
    }
    Ok(())
}

pub fn split_and_remove_blank(content: &String, pattern: &str) -> Result<Vec<String>, HinaError> {
    debug_fn!(content,pattern);
    Ok(content
        .trim()
        .split(pattern)
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

pub fn gen_rand_str(len: usize) -> String {
    debug_fn!(len);
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
}

pub fn gen_str_width_ctrl(str: &String, width: usize) -> String {
    debug_fn!(str,width);
    let len = str.len();
    let mut result = String::new();
    result += str;
    if len < width {
        result += &String::from(" ").repeat(width - len);
    }
    return result;
}

pub fn print_info(head: &Vec<String>,
                  data: &Vec<Vec<String>>,
                  n_len: usize) {
    debug_fn!();
    let mut max_len: Vec<usize> = vec![0; n_len];
    for line in data {
        for i in 0..n_len {
            max_len[i] = max(max_len[i], line[i].len());
        }
    }
    for i in 0..n_len {
        print!("{}  ", gen_str_width_ctrl(&head[i], max_len[i]));
    }
    println!();
    for d in data {
        for i in 0..n_len {
            print!("{}  ", gen_str_width_ctrl(&d[i], max_len[i]));
        }
        println!();
    }
}