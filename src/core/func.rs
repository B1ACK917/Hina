use std::cmp::max;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Stdio;

use execute::{Execute, shell};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use crate::core::error::HinaError;
use crate::core::global::{DEBUG, RAND_STR_LEN, RECYCLE, RM_STACK, SPLITTER};

fn read_var(var_name: &str) -> Result<String, HinaError> {
    return match env::var(var_name) {
        Ok(val) => { Ok(val) }
        Err(_) => {
            let err = format!("Cannot locate variable {}", var_name);
            Err(HinaError::VarError(err))
        }
    };
}

pub fn get_home() -> Result<String, HinaError> {
    Ok(read_var("HOME")?)
}

pub fn get_user() -> Result<String, HinaError> {
    Ok(read_var("USER")?)
}

pub fn get_current_path() -> Result<PathBuf, HinaError> {
    return match env::current_dir() {
        Ok(cur) => { Ok(cur) }
        Err(err) => {
            Err(HinaError::WorkPathError(err.to_string()))
        }
    };
}

pub fn get_uid() -> Result<String, HinaError> {
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

pub fn split_and_remove_blank(content: &String, pattern: &str) -> Vec<String> {
    return content
        .split(pattern)
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();
}

pub fn execute_command(input: &String) -> Result<String, HinaError> {
    let mut command = shell(input);
    command.stdout(Stdio::piped());
    let command_out_utf_8 = match command.execute_output() {
        Ok(output) => { output.stdout }
        Err(err) => { return Err(HinaError::CommandExecError(err.to_string())); }
    };
    return match String::from_utf8(command_out_utf_8) {
        Ok(output) => { Ok(output) }
        Err(err) => { Err(HinaError::CommandParseError(err.to_string())) }
    };
}

pub fn get_execute_target(work_path: &PathBuf, input_path: &PathBuf) -> Result<PathBuf, HinaError> {
    let mut target;
    if input_path.is_absolute() {
        target = input_path.clone();
    } else {
        target = work_path.clone();
        target.push(&input_path);
    }
    if target.exists() {
        if !target.is_symlink() {
            return match target.canonicalize() {
                Ok(path) => { Ok(path) }
                Err(_) => {
                    let err = format!("Unable to open {}", target.display());
                    Err(HinaError::BadFileError(err))
                }
            };
        } else {
            Ok(target)
        }
    } else {
        let err = format!("Unable to locate {}", target.display());
        Err(HinaError::FileNotExistError(err))
    }
}

pub fn parse_args_or(args: &Vec<String>, default: String) -> Vec<String> {
    return if args.len() > 0 {
        args.clone()
    } else {
        vec![default]
    };
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
    let rm_stack: Vec<String> = split_and_remove_blank(&contents, "\n");

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
                 path.display(),
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

pub fn gen_str_width_ctrl(str: &String, width: usize) -> String {
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

// pub fn test() {
//     let head = vec!["UID".to_string(),
//                     "PID".to_string(),
//                     "SIZE".to_string(),
//                     "SWAP".to_string(),
//                     "PSS".to_string(),
//                     "RSS".to_string(),
//                     "CMD".to_string()];
//     let proc_map_opt = process::read_mem_detail_from_proc(226912);
//     dbg!(&proc_map_opt);
//     let mut output_list: Vec<Vec<String>> = Vec::new();
//     if proc_map_opt.is_some() {
//         let proc_map = proc_map_opt.unwrap();
//         let output_info = vec![proc_map.get_total_as_kb("size").to_string(),
//                                proc_map.get_total_as_kb("size").to_string(),
//                                proc_map.get_total_as_kb("size").to_string(),
//                                proc_map.get_total_as_kb("swap").to_string(),
//                                proc_map.get_total_as_kb("pss").to_string(),
//                                proc_map.get_total_as_kb("rss").to_string(),
//                                proc_map.get_total_as_kb("size").to_string(), ];
//         output_list.push(output_info);
//     }
//     print_info(&head, &output_list, 7);
// }