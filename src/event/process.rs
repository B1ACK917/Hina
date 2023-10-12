use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::core::func;
use crate::core::global::{DEBUG, MEM_EXTRACT_RE};

#[derive(Debug)]
struct ProcessBlock {
    uid: String,
    pid: u32,
    _ppid: u32,
    _c: u32,
    _stime: String,
    _tty: String,
    _time: String,
    cmd: String,
    origin: String,
}

struct MapMeta {
    size: u32,
    rss: u32,
    pss: u32,
    swap: u32,
}

pub struct ProcessMap {
    start: String,
    end: String,
    mode: String,
    offset: String,
    device: String,
    inode: String,
    name: String,
    maps: Vec<MapMeta>,
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
            _ppid: entries[2].parse().unwrap(),
            _c: entries[3].parse().unwrap(),
            _stime: entries[4].clone(),
            _tty: entries[5].clone(),
            _time: entries[6].clone(),
            cmd,
            origin: String::from(input),
        };
    }
}

impl ProcessMap {
    pub fn from(smaps: &String) -> ProcessMap {
        let lines: Vec<&str> = smaps.split("\n").collect();
        for line in lines {
            let caps = MEM_EXTRACT_RE.captures(line);
            if caps.is_some() {
                let map = caps.unwrap();
                let name = map["name"].to_string();
                let amount = map["amount"].to_string();
            }
        }
        return ProcessMap {
            start: "".to_string(),
            end: "".to_string(),
            mode: "".to_string(),
            offset: "".to_string(),
            device: "".to_string(),
            inode: "".to_string(),
            name: "".to_string(),
            maps: vec![],
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
        .filter(|x| (&x.uid == user || &x.uid == uid) && x.cmd.contains(process_name))
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
    if target.exists() {
        let is_some = target.read_dir().unwrap().next().is_some();
        if is_some {
            println!("Directory not empty");
            return;
        }
    } else {
        fs::create_dir_all(&target).unwrap();
    }
    let all_process = get_all_process();
    let user_process: Vec<&ProcessBlock> = all_process
        .iter()
        .filter(|x| &x.uid == user || &x.uid == uid)
        .collect();
    if *DEBUG {
        dbg!(&user_process);
    }
    for process in user_process {
        let pid = &process.pid;
        target.push(pid.to_string());
        fs::create_dir(&target).unwrap();
        let command = String::from(format!("cat /proc/{}/smaps > {}/smaps", pid, &target.display()));
        func::execute_command(&command);
        let command = String::from(format!("cat /proc/{}/cmdline > {}/cmdline", pid, &target.display()));
        func::execute_command(&command);
        target.pop();
    }
}

pub fn read_mem_detail_from_proc(proc_id: u32) -> Option<ProcessMap> {
    let proc_file = PathBuf::from(format!("proc/{}/smaps", proc_id));
    if proc_file.exists() {
        let mut file = File::open(proc_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let process_map = ProcessMap::from(&contents);
        return Some(process_map);
    } else {
        return None;
    }
}

pub fn get_proc_mem_detail(user: &String,
                           uid: &String) {
    let all_process = get_all_process();
    let user_process: Vec<&ProcessBlock> = all_process
        .iter()
        .filter(|x| &x.uid == user || &x.uid == uid)
        .collect();
    for process in user_process {
        read_mem_detail_from_proc(process.pid);
    }
}