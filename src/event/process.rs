use std::path::PathBuf;

use crate::core::func;
use crate::core::global::DEBUG;

#[derive(Debug)]
struct ProcessBlock {
    uid: String,
    pid: u32,
    ppid: u32,
    c: u32,
    stime: String,
    tty: String,
    time: String,
    cmd: String,
    origin: String,
}

impl ProcessBlock {
    pub fn from(input: &str) -> ProcessBlock {
        let entries: Vec<String> = input
            .split(" ")
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let mut cmd = String::new();
        for cmd_part in &entries[7..] {
            cmd += cmd_part;
            cmd += " ";
        }
        cmd = cmd.trim().to_string();
        return ProcessBlock {
            uid: entries[0].clone(),
            pid: entries[1].parse().unwrap(),
            ppid: entries[2].parse().unwrap(),
            c: entries[3].parse().unwrap(),
            stime: entries[4].clone(),
            tty: entries[5].clone(),
            time: entries[6].clone(),
            cmd,
            origin: String::from(input),
        };
    }
}

fn get_all_process() -> Vec<ProcessBlock> {
    let command = format!("ps -ef | sed -n '2,$p'");
    let output = func::execute_command(&command);
    let entries: Vec<&str> = output.trim().split("\n").collect();
    let mut all_process = Vec::new();
    for entry in entries {
        all_process.push(ProcessBlock::from(entry));
    }
    return all_process;
}

fn get_ps_head() -> String {
    let command = String::from("ps -ef | sed -n '1p'");
    return func::execute_command(&command);
}

pub fn show_user_all_process(user: &String, uid: &String) {
    let all_process = get_all_process();
    let user_process: Vec<&ProcessBlock> = all_process
        .iter()
        .filter(|x| &x.uid == user || &x.uid == uid)
        .collect();
    println!("{}", get_ps_head());
    for process in user_process {
        println!("{}", process.origin);
    }
}

pub fn show_user_spec_process(user: &String,
                              uid: &String,
                              process_name: &String) {
    let all_process = get_all_process();
    let user_process: Vec<&ProcessBlock> = all_process
        .iter()
        .filter(|x| &x.uid == user || &x.uid == uid && x.cmd.contains(process_name))
        .collect();
    println!("{}", get_ps_head());
    for process in user_process {
        println!("{}", process.origin);
    }
}

pub fn dump_proc(user: &String,
                 uid: &String,
                 work_path: &PathBuf,
                 input_path: &PathBuf) {
    let mut target = func::get_execute_target(work_path, input_path);
    let all_process = get_all_process();
    let user_process: Vec<&ProcessBlock> = all_process
        .iter()
        .filter(|x| &x.uid == user || &x.uid == uid)
        .collect();
    if *DEBUG {
        dbg!(&user_process);
    }
    for process in user_process {
        let uid = &process.uid;
        target.push(uid);
        let command = String::from(format!("cat /proc/{}/smaps > {}/smaps", uid, target.display()));
        func::execute_command(&command);
        let command = String::from(format!("cat /proc/{}/cmdline > {}/cmdline", uid, target.display()));
        func::execute_command(&command);
        target.pop();
    }
}

pub fn get_proc_mem_detail(user: &String,
                           uid: &String) {
    return;
}