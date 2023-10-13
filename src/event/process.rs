use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::string::ToString;

use crate::core::func;
use crate::core::func::print_info;
use crate::core::global::{DEBUG, MEM_EXTRACT_RE};

#[derive(Debug, Clone)]
struct ProcessInfo {
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

#[derive(Debug, Clone)]
pub struct ProcessMapMeta {
    _start: String,
    _end: String,
    _mode: String,
    _offset: String,
    _device: String,
    _inode: String,
    _name: String,
    maps: HashMap<String, u32>,
    _cmd: String,
}

#[derive(Debug, Clone)]
pub struct ProcessMap {
    _data: Vec<ProcessMapMeta>,
    total: HashMap<String, u64>,
}


impl ProcessInfo {
    pub fn from(input: &str) -> ProcessInfo {
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
        return ProcessInfo {
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

impl ProcessMapMeta {
    pub fn from(smap_block: &Vec<&str>, cmdline: &String) -> ProcessMapMeta {
        let keys = func::split_and_remove_blank(&smap_block[0].to_string(), " ");
        let mut maps: HashMap<String, u32> = HashMap::new();
        for line in smap_block {
            let caps = MEM_EXTRACT_RE.captures(line);
            if caps.is_some() {
                let map = caps.unwrap();
                let name = map["name"].to_lowercase().to_string();
                let amount: u32 = map["amount"].to_string().parse().unwrap_or(0);
                maps.insert(name, amount);
            }
        }

        let range: Vec<&str> = keys[0].split("-").collect();
        let name = if keys.len() > 5 { keys[5].to_string() } else { "".to_string() };
        return ProcessMapMeta {
            _start: range[0].to_string(),
            _end: range[1].to_string(),
            _mode: keys[1].to_string(),
            _offset: keys[2].to_string(),
            _device: keys[3].to_string(),
            _inode: keys[4].to_string(),
            _name: name,
            maps,
            _cmd: cmdline.replace('\0', " ").trim().to_string(),
        };
    }
}

impl ProcessMap {
    pub fn from(input: Vec<ProcessMapMeta>) -> ProcessMap {
        let keys = input[0].maps.keys();
        let mut total: HashMap<String, u64> = HashMap::new();
        for key in keys {
            let mut cal: u64 = 0;
            for datum in &input {
                cal += datum.maps[key] as u64;
            }
            total.insert(key.clone(), cal);
        }
        return ProcessMap {
            _data: input,
            total,
        };
    }

    pub fn get_total(&self, key: &str) -> u64 {
        return self.total[&key.to_string()];
    }
}

fn get_all_process() -> Vec<ProcessInfo> {
    let command = format!("ps -ef | sed -n '2,$p'");
    let output = func::execute_command(&command);
    let entries: Vec<&str> = output.trim().split("\n").collect();
    let mut all_process = Vec::new();
    for entry in entries {
        all_process.push(ProcessInfo::from(entry));
    }
    return all_process;
}

fn get_ps_head() -> String {
    let command = String::from("ps -ef | sed -n '1p'");
    return func::execute_command(&command);
}

pub fn build_proc_map_list(smap_input: &String, cmd_input: Option<&String>) -> ProcessMap {
    let lines: Vec<&str> = smap_input.split("\n").collect();
    let mut map_list: Vec<ProcessMapMeta> = Vec::new();
    let mut smap_block: Vec<&str> = Vec::new();
    smap_block.push(lines[0]);

    for line in &lines[1..] {
        if line.contains("-") {
            map_list.push(ProcessMapMeta::from(&smap_block, cmd_input.unwrap_or(&"".to_string())));
            smap_block.clear();
            smap_block.push(line);
        } else {
            smap_block.push(line);
        }
    }

    return ProcessMap::from(map_list);
}

pub fn read_mem_detail_from_proc(proc_id: u32) -> Option<ProcessMap> {
    let smap_file = PathBuf::from(format!("/proc/{}/smaps", proc_id));
    // let cmd_file = PathBuf::from(format!("/proc/{}/cmdline", proc_id));
    return if smap_file.exists() {
        let smap = File::open(smap_file);
        if smap.is_err() {
            return None;
        }
        // let cmdline = File::open(cmd_file);
        // if cmdline.is_err() {
        //     return None;
        // }
        let mut smap_contents = String::new();
        smap.unwrap().read_to_string(&mut smap_contents).unwrap();
        // let mut cmd_contents = String::new();
        // cmdline.unwrap().read_to_string(&mut cmd_contents).unwrap();
        let process_map = build_proc_map_list(&smap_contents, None);
        Some(process_map)
    } else {
        None
    };
}

pub fn show_user_all_process(user: &String, uid: &String) {
    let all_process = get_all_process();
    let user_process: Vec<&ProcessInfo> = all_process
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
    let user_process: Vec<&ProcessInfo> = all_process
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
    let user_process: Vec<&ProcessInfo> = all_process
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

pub fn get_proc_mem_detail(user: &String,
                           uid: &String) {
    let all_process = get_all_process();
    let user_process: Vec<&ProcessInfo> = all_process
        .iter()
        .filter(|x| &x.uid == user || &x.uid == uid)
        .collect();
    let mut output_list = Vec::new();
    let head = vec!["UID".to_string(),
                    "PID".to_string(),
                    "SIZE".to_string(),
                    "SWAP".to_string(),
                    "PSS".to_string(),
                    "RSS".to_string(),
                    "CMD".to_string()];

    for proc_info in user_process {
        let proc_map_opt = read_mem_detail_from_proc(proc_info.pid);
        if proc_map_opt.is_some() {
            let proc_map = proc_map_opt.unwrap();
            let output_info = vec![proc_info.uid.to_string(),
                                   proc_info.pid.to_string(),
                                   proc_map.get_total("size").to_string(),
                                   proc_map.get_total("swap").to_string(),
                                   proc_map.get_total("pss").to_string(),
                                   proc_map.get_total("rss").to_string(),
                                   proc_info.cmd.to_string()];
            output_list.push(output_info);
        }
    }
    print_info(&head, &output_list);
}