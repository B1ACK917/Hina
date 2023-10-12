use std::{env, io};
use std::path::PathBuf;
use std::process::exit;

use crate::core::config::{Action, Config};
use crate::core::func;
use crate::core::global::DATA_DIR;
use crate::event::fs;
use crate::event::process;
use crate::event::recycle;

#[derive(Debug, Clone)]
pub struct Executor {
    config: Config,
    work_path: PathBuf,
    data_path: PathBuf,
    user: String,
    uid: String,
}

impl Executor {
    pub fn build(config: Config) -> Executor {
        let work_path_str = env::current_dir().unwrap().display().to_string();
        let home_path_str = func::get_home();

        let work_path = PathBuf::from(work_path_str);
        let home_path = PathBuf::from(home_path_str);
        let mut data_path = home_path;
        data_path.push(DATA_DIR);

        return Executor {
            config,
            work_path,
            data_path,
            user: func::get_user(),
            uid: func::get_uid(),
        };
    }

    pub fn run(&self) {
        func::init_data_dir(&self.data_path);

        let args = self.config.get_args();
        let arg_num = self.config.arg_num();
        let mut rm_stack = func::load_rm_stack(&self.data_path);

        match self.config.get_action() {
            Action::Remove => {
                for arg in args {
                    let target = PathBuf::from(arg);
                    recycle::remove(&target, &self.data_path, &self.work_path, &mut rm_stack);
                }
            }

            Action::Restore => {
                if rm_stack.len() > 0 {
                    let rm_paths = func::show_rm_stack(&rm_stack);
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let index: i8 = input.trim().parse().unwrap_or(-1);
                    recycle::restore(rm_paths, index, &mut rm_stack);
                } else {
                    println!("Recycle bin empty.");
                }
            }

            Action::EmptyTrash => {
                recycle::empty_trash_bin(&self.data_path, &mut rm_stack);
            }

            Action::Process => {
                match arg_num {
                    0 => { process::show_user_all_process(&self.user, &self.uid); }
                    1 => { process::show_user_spec_process(&self.user, &self.uid, &args[0]) }
                    _ => {
                        println!("Unexpected args");
                        exit(-1);
                    }
                }
            }

            Action::MakeNestedDir => {
                if args.len() > 0 {
                    for arg in args {
                        let target = PathBuf::from(arg);
                        fs::make_nested_dir(&self.work_path, &target, false);
                    }
                } else {
                    let target = PathBuf::from(".");
                    fs::make_nested_dir(&self.work_path, &target, false);
                }
            }

            Action::SymlinkToLink => {
                if args.len() > 0 {
                    for arg in args {
                        let target = PathBuf::from(arg);
                        fs::symlink_to_link(&self.work_path, &target, false);
                    }
                } else {
                    let target = PathBuf::from(".");
                    fs::symlink_to_link(&self.work_path, &target, false);
                }
            }

            Action::LinkToSymlink => {
                let mut args_ = args.clone();
                let link_src_dir_str;

                if args_.len() > 0 {
                    link_src_dir_str = args_[0].clone();
                } else {
                    link_src_dir_str = String::from("/");
                    println!("No source dir found, finding from /");
                }
                let link_src_dir = PathBuf::from(&link_src_dir_str);
                args_.remove(0);

                if args_.len() > 0 {
                    for arg in args_ {
                        let target = PathBuf::from(arg);
                        fs::link_to_symlink(&self.work_path, &target, &link_src_dir, false);
                    }
                } else {
                    let target = PathBuf::from(".");
                    fs::link_to_symlink(&self.work_path, &target, &link_src_dir, false);
                }
            }

            Action::DumpMemory => {
                let input;
                if args.len() > 0 {
                    input = args[0].clone();
                } else {
                    input = String::from("proc");
                }
                println!("Dump process detail to {}", input);
                let target = PathBuf::from(&input);
                process::dump_proc(&self.user, &self.uid, &self.work_path, &target);
            }

            Action::MemoryDetail => {
                process::get_proc_mem_detail(&self.user, &self.uid);
            }

            Action::None => {}
            Action::ILLEGAL => {}
        }

        func::save_rm_stack(&self.data_path, &rm_stack);
    }
}