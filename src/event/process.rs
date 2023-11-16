use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::string::ToString;

use crate::core::config::RMRecord;
use crate::core::error::HinaError;
use crate::core::func;
use crate::core::func::{parse_flag_bool, parse_flag_string, parse_flag_u, print_info};
use crate::core::global::{DEBUG, MEM_EXTRACT_RE};
use crate::event::base::HinaModuleRun;

#[derive(Debug, Clone)]
struct ProcessInfo {
    _uid: String,
    _pid: usize,
    _ppid: usize,
    _c: usize,
    _stime: String,
    _tty: String,
    _time: String,
    _cmd: String,
    _origin: String,
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
    _maps: HashMap<String, usize>,
    _cmd: String,
}

#[derive(Debug, Clone)]
pub struct ProcessMap {
    _data: Vec<ProcessMapMeta>,
    _total: HashMap<String, u64>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Process;

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
            _uid: entries[0].clone(),
            _pid: entries[1].parse().unwrap(),
            _ppid: entries[2].parse().unwrap(),
            _c: entries[3].parse().unwrap(),
            _stime: entries[4].clone(),
            _tty: entries[5].clone(),
            _time: entries[6].clone(),
            _cmd: cmd,
            _origin: String::from(input),
        };
    }
}

impl ProcessMapMeta {
    pub fn from(smap_block: &Vec<&str>, cmdline: &String) -> ProcessMapMeta {
        let keys = func::split_and_remove_blank(&smap_block[0].to_string(), " ");
        let mut maps: HashMap<String, usize> = HashMap::new();
        for line in smap_block {
            let caps = MEM_EXTRACT_RE.captures(line);
            if caps.is_some() {
                let map = caps.unwrap();
                let name = map["name"].to_lowercase().to_string();
                let amount: usize = map["amount"].to_string().parse().unwrap_or(0);
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
            _maps: maps,
            _cmd: cmdline.replace('\0', " ").trim().to_string(),
        };
    }
}

impl ProcessMap {
    pub fn from(input: Vec<ProcessMapMeta>) -> ProcessMap {
        let keys = input[0]._maps.keys();
        let mut total: HashMap<String, u64> = HashMap::new();
        for key in keys {
            let mut cal: u64 = 0;
            for datum in &input {
                cal += datum._maps[key] as u64;
            }
            total.insert(key.clone(), cal);
        }
        return ProcessMap {
            _data: input,
            _total: total,
        };
    }

    pub fn get_total_as_kb(&self, key: &str) -> String {
        return format!("{} KB", self._total[&key.to_string()]);
    }

    pub fn get_total_as_str(&self, key: &str) -> String {
        return format!("{}", self._total[&key.to_string()]);
    }

    pub fn get_total_as_human_readable(&self, key: &str) -> String {
        let mut num = self._total[&key.to_string()] as f64;
        if num > 1024f64 {
            num /= 1024.0;
        } else {
            return format!("{:.3} KB", num);
        }
        if num > 1024f64 {
            num /= 1024.0;
        } else {
            return format!("{:.3} MB", num);
        }
        return format!("{:.3} GB", num);
    }
}

impl HinaModuleRun for Process {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &HashMap<String, String>,
           _rm_stack: &mut Vec<RMRecord>,
           _target: &PathBuf,
           _arg_num: usize,
    ) -> Result<(), HinaError> {
        let spec_pattern = parse_flag_string(_flags, "i");
        let ans_id = parse_flag_u(_flags, "a");
        let dump = parse_flag_bool(_flags, "dump");
        let xray = parse_flag_bool(_flags, "x");
        let sort_by = parse_flag_string(_flags, "s");
        let human_readable = parse_flag_bool(_flags, "h");
        if ans_id != 0 {
            Process::show_process_ancestor(ans_id)?;
            return Ok(());
        }
        if dump {
            let mut target = if _arg_num > 0 {
                _target.clone()
            } else {
                let mut proc = _work_path.clone();
                proc.push("proc");
                proc
            };
            Process::dump_proc(_user, _uid, &mut target)?;
            return Ok(());
        }
        if xray {
            Process::get_proc_mem_detail(_user, _uid, &sort_by, human_readable)?;
            return Ok(());
        }
        if spec_pattern.is_empty() {
            Process::show_user_all_process(_user, _uid)?;
        } else {
            Process::show_user_spec_process(_user, _uid, &spec_pattern)?;
        }
        Ok(())
    }
}

impl Process {
    fn get_all_process() -> Result<Vec<ProcessInfo>, HinaError> {
        let command = format!("ps -ef | sed -n '2,$p'");
        let output = func::execute_command(&command)?;
        let entries: Vec<&str> = output.trim().split("\n").collect();
        let mut all_process = Vec::new();
        for entry in entries {
            all_process.push(ProcessInfo::from(entry));
        }
        Ok(all_process)
    }

    fn get_ps_head() -> Result<String, HinaError> {
        let command = String::from("ps -ef | sed -n '1p'");
        Ok(func::execute_command(&command)?)
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

    pub fn read_mem_detail_from_proc(proc_id: usize) -> Option<ProcessMap> {
        let smap_file = PathBuf::from(format!("/proc/{}/smaps", proc_id));
        // let cmd_file = PathBuf::from(format!("/proc/{}/cmdline", proc_id));
        return if smap_file.exists() {
            let smap = File::open(smap_file);
            if smap.is_err() {
                return None;
            }
            let mut smap_contents = String::new();
            smap.unwrap().read_to_string(&mut smap_contents).unwrap();
            let process_map = Process::build_proc_map_list(&smap_contents, None);
            Some(process_map)
        } else {
            None
        };
    }
    pub fn show_user_all_process(user: &String, uid: &String) -> Result<(), HinaError> {
        let all_process = Process::get_all_process()?;
        let user_process: Vec<&ProcessInfo> = all_process
            .iter()
            .filter(|x| &x._uid == user || &x._uid == uid)
            .collect();
        println!("{}", Process::get_ps_head()?);
        for process in user_process {
            println!("{}", process._origin);
        }
        Ok(())
    }

    pub fn show_user_spec_process(user: &String,
                                  uid: &String,
                                  process_name: &String) -> Result<(), HinaError> {
        let all_process = Process::get_all_process()?;
        let user_process: Vec<&ProcessInfo> = all_process
            .iter()
            .filter(|x| (&x._uid == user || &x._uid == uid) && x._cmd.contains(process_name))
            .collect();
        println!("{}", Process::get_ps_head()?);
        for process in user_process {
            println!("{}", process._origin);
        }
        Ok(())
    }

    pub fn show_process_ancestor(process_id: usize) -> Result<(), HinaError> {
        let all_process = Process::get_all_process()?;
        let mut process_route = HashMap::new();
        for process in all_process {
            process_route.insert(process._pid, (process._ppid, process._origin));
        }
        if process_route.contains_key(&process_id) {
            let mut pid = process_id;
            let mut ppid;
            let mut ancestors = vec![&process_route[&pid].1];
            loop {
                ppid = process_route[&pid].0;
                if ppid == 0 {
                    break;
                }
                pid = ppid;
                ancestors.push(&process_route[&pid].1);
            }
            println!("{}", Process::get_ps_head()?);
            for ancestor in ancestors {
                println!("{}", ancestor);
            }
        }
        Ok(())
    }

    pub fn dump_proc(user: &String,
                     uid: &String,
                     target: &mut PathBuf) -> Result<(), HinaError> {
        if target.exists() {
            let is_some = target.read_dir().unwrap().next().is_some();
            if is_some {
                let err = format!("Directory {} not empty", target.display());
                return Err(HinaError::DirNotEmptyError(err));
            }
        } else {
            fs::create_dir_all(&target).unwrap();
        }
        let all_process = Process::get_all_process()?;
        let user_process: Vec<&ProcessInfo> = all_process
            .iter()
            .filter(|x| &x._uid == user || &x._uid == uid)
            .collect();
        if *DEBUG {
            dbg!(&user_process);
        }
        for process in user_process {
            let pid = &process._pid;
            target.push(pid.to_string());
            fs::create_dir(&target).unwrap();
            let command = String::from(format!("cat /proc/{}/smaps > {}/smaps", pid, &target.display()));
            func::execute_command(&command)?;
            let command = String::from(format!("cat /proc/{}/cmdline > {}/cmdline", pid, &target.display()));
            func::execute_command(&command)?;
            target.pop();
        }
        Ok(())
    }

    pub fn get_proc_mem_detail(user: &String,
                               uid: &String,
                               sort_by: &String,
                               human_readable: bool) -> Result<(), HinaError> {
        let all_process = Process::get_all_process()?;
        let user_process: Vec<&ProcessInfo> = all_process
            .iter()
            .filter(|x| &x._uid == user || &x._uid == uid)
            .collect();
        let mut output_list = Vec::new();
        let head = vec!["UID".to_string(),
                        "PID".to_string(),
                        "SIZE".to_string(),
                        "SWAP".to_string(),
                        "PSS".to_string(),
                        "RSS".to_string(),
                        "CMD".to_string()];
        let sort_by_map: HashMap<&str, i32> = HashMap::from([
            ("pid", 1),
            ("size", 7),
            ("swap", 8),
            ("pss", 9),
            ("rss", 10),
        ]);
        let sort_by_ind = if sort_by_map.contains_key(sort_by.as_str()) {
            sort_by_map[sort_by.as_str()] as usize
        } else { 1 };

        for proc_info in user_process {
            let proc_map_opt = Process::read_mem_detail_from_proc(proc_info._pid);
            if proc_map_opt.is_some() {
                let proc_map = proc_map_opt.unwrap();
                let output_info: Vec<String>;
                if human_readable {
                    output_info = vec![proc_info._uid.to_string(),
                                       proc_info._pid.to_string(),
                                       proc_map.get_total_as_human_readable("size"),
                                       proc_map.get_total_as_human_readable("swap"),
                                       proc_map.get_total_as_human_readable("pss"),
                                       proc_map.get_total_as_human_readable("rss"),
                                       proc_info._cmd.to_string(),
                                       proc_map.get_total_as_str("size"),
                                       proc_map.get_total_as_str("swap"),
                                       proc_map.get_total_as_str("pss"),
                                       proc_map.get_total_as_str("rss"), ];
                } else {
                    output_info = vec![proc_info._uid.to_string(),
                                       proc_info._pid.to_string(),
                                       proc_map.get_total_as_kb("size"),
                                       proc_map.get_total_as_kb("swap"),
                                       proc_map.get_total_as_kb("pss"),
                                       proc_map.get_total_as_kb("rss"),
                                       proc_info._cmd.to_string(),
                                       proc_map.get_total_as_str("size"),
                                       proc_map.get_total_as_str("swap"),
                                       proc_map.get_total_as_str("pss"),
                                       proc_map.get_total_as_str("rss"), ];
                }
                output_list.push(output_info);
            }
        }
        output_list.sort_by(|x1, x2| {
            let x3: u64 = x1[sort_by_ind].parse().unwrap();
            let x4: u64 = x2[sort_by_ind].parse().unwrap();
            return x3.partial_cmp(&x4).unwrap();
        });
        print_info(&head, &output_list, 7);
        Ok(())
    }
}