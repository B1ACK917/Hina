use std::collections::HashMap;
use std::path::PathBuf;

use crate::core::config::{Config, Target};
use crate::core::error::HinaError;
use crate::core::func;
use crate::core::global::{DATA_DIR, RECYCLE};
use crate::event::base::HinaModuleRun;

#[derive(Debug, Clone)]
pub struct Executor {
    config: Config,
    work_path: PathBuf,
    data_path: PathBuf,
    recycle_path: PathBuf,
    user: String,
    uid: String,
    flags: ExecFlag,
}

#[derive(Debug, Clone)]
pub struct ExecFlag {
    _recursive: bool,
    _human_readable: bool,
    _input: String,
    _output: String,
    _append: String,
    _num: usize,
}

impl Executor {
    fn parse_flag(flags: &HashMap<String, String>) -> ExecFlag {
        let _recursive = if flags.contains_key("r") {
            true
        } else {
            false
        };

        let _human_readable = if flags.contains_key("h") {
            true
        } else {
            false
        };

        let _input = if flags.contains_key("i") {
            flags["i"].clone()
        } else {
            String::new()
        };

        let _output = if flags.contains_key("o") {
            flags["o"].clone()
        } else {
            String::new()
        };

        let _append = if flags.contains_key("a") {
            flags["a"].clone()
        } else {
            String::new()
        };

        let _num: usize = if flags.contains_key("n") {
            flags["n"].clone().parse().unwrap_or(0)
        } else {
            0
        };

        return ExecFlag {
            _recursive,
            _human_readable,
            _input,
            _output,
            _append,
            _num,
        };
    }

    pub fn build(config: Config) -> Result<Executor, HinaError> {
        let home_path_str = func::get_home()?;

        let work_path = func::get_current_path()?;
        let home_path = PathBuf::from(home_path_str);
        let mut data_path = home_path;
        data_path.push(DATA_DIR);
        let mut recycle_path = data_path.clone();
        recycle_path.push(RECYCLE);

        let flags = Executor::parse_flag(config.get_flags());

        return Ok(Executor {
            config,
            work_path,
            data_path,
            recycle_path,
            user: func::get_user()?,
            uid: func::get_uid()?,
            flags,
        });
    }

    fn run_iter(&self,
                module: &impl HinaModuleRun,
                _work_path: &PathBuf,
                _data_path: &PathBuf,
                _recycle_path: &PathBuf,
                _user: &String,
                _uid: &String,
                _flags: &ExecFlag,
                _rm_stack: &mut Vec<String>,
                args: &Vec<String>) -> Result<(), HinaError> {
        for arg in args {
            let arg_path_buf = PathBuf::from(arg);
            let target = func::get_execute_target(_work_path, &arg_path_buf)?;
            module.run(
                _work_path,
                _data_path,
                _recycle_path,
                _user,
                _uid,
                _flags,
                _rm_stack,
                &target,
            )?
        }
        Ok(())
    }

    pub fn run(&self) -> Result<(), HinaError> {
        func::init_data_dir(&self.data_path);

        let args = self.config.get_args();
        let mut rm_stack = func::load_rm_stack(&self.data_path);

        match self.config.get_target() {
            Target::Remove(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, &self.flags, &mut rm_stack, args,)?
            }
            Target::RecycleBin(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, &self.flags, &mut rm_stack, args,)?
            }
            //
            // Target::Restore => {
            //     if rm_stack.len() > 0 {
            //         let rm_paths = func::show_rm_stack(&rm_stack);
            //         let mut input = String::new();
            //         io::stdin().read_line(&mut input).unwrap();
            //         let index: i8 = input.trim().parse().unwrap_or(-1);
            //         recycle::restore(
            //             rm_paths,
            //             index,
            //             &mut rm_stack,
            //         );
            //     } else {
            //         println!("Recycle bin empty.");
            //     }
            // }
            //
            // Target::EmptyTrash => {
            //     recycle::empty_trash_bin(
            //         &self.data_path,
            //         &mut rm_stack,
            //     );
            // }
            //
            // Target::Process => {
            //     match arg_num {
            //         0 => {
            //             process::show_user_all_process(
            //                 &self.user,
            //                 &self.uid,
            //             )?
            //         }
            //         1 => {
            //             process::show_user_spec_process(
            //                 &self.user,
            //                 &self.uid,
            //                 &args[0],
            //             )?
            //         }
            //         _ => {
            //             println!("Unexpected args");
            //             exit(-1);
            //         }
            //     }
            // }
            //
            // Target::ProcessAncestor => {
            //     if arg_num > 0 {
            //         for arg in args {
            //             process::get_process_ancestor(arg.parse().unwrap_or(1));
            //         }
            //     }
            // }
            //
            // Target::MakeNestedDir => {
            //     let args_ = func::parse_args_or(args, String::from("."));
            //     for arg in &args_ {
            //         let target = PathBuf::from(arg);
            //         fs::make_nested_dir(
            //             &self.work_path,
            //             &target,
            //             self.flags.recursive,
            //         );
            //     }
            // }
            //
            // Target::SymlinkToLink => {
            //     let args_ = func::parse_args_or(args, String::from("."));
            //     for arg in &args_ {
            //         let target = PathBuf::from(arg);
            //         fs::symlink_to_link(
            //             &self.work_path,
            //             &target,
            //             self.flags.recursive,
            //         );
            //     }
            // }
            //
            // Target::LinkToSymlink => {
            //     let mut args_ = args.clone();
            //     let link_src_dir_str;
            //
            //     if args_.len() > 0 {
            //         link_src_dir_str = args_[0].clone();
            //     } else {
            //         link_src_dir_str = String::from("/");
            //         println!("No source dir found, finding from /");
            //     }
            //     let link_src_dir = PathBuf::from(&link_src_dir_str);
            //     args_.remove(0);
            //
            //     args_ = func::parse_args_or(&args_, String::from("."));
            //     for arg in &args_ {
            //         let target = PathBuf::from(arg);
            //         fs::link_to_symlink(
            //             &self.work_path,
            //             &target,
            //             &link_src_dir,
            //             self.flags.recursive,
            //         );
            //     }
            // }
            //
            // Target::Rename => {
            //     let args_ = func::parse_args_or(args, String::from("."));
            //     for arg in &args_ {
            //         let target = PathBuf::from(arg);
            //         fs::rename(
            //             &self.work_path,
            //             &target,
            //             &self.flags.input,
            //             &self.flags.output,
            //             &self.flags.append,
            //             self.flags.num,
            //             self.flags.recursive,
            //         );
            //     }
            // }
            //
            // Target::RenameSym => {
            //     let args_ = func::parse_args_or(args, String::from("."));
            //     for arg in &args_ {
            //         let target = PathBuf::from(arg);
            //         fs::rename_symbol_link(
            //             &self.work_path,
            //             &target,
            //             &self.flags.input,
            //             &self.flags.output,
            //             &self.flags.append,
            //             self.flags.num,
            //             self.flags.recursive,
            //         );
            //     }
            // }
            //
            // Target::DumpMemory => {
            //     let args_ = func::parse_args_or(args, String::from("proc"));
            //     for arg in &args_ {
            //         println!("Dump process detail to {}", arg);
            //         let target = PathBuf::from(arg);
            //         process::dump_proc(
            //             &self.user,
            //             &self.uid,
            //             &self.work_path,
            //             &target,
            //         );
            //     }
            // }
            //
            // Target::MemoryDetail => {
            //     let args_ = func::parse_args_or(args, String::from("pid"));
            //     process::get_proc_mem_detail(
            //         &self.user,
            //         &self.uid,
            //         &args_[0],
            //         self.flags.human_readable,
            //     );
            // }
            //
            // Target::Test => {
            //     if *DEBUG {
            //         func::test();
            //     }
            // }

            Target::None => {}
        }

        func::save_rm_stack(&self.data_path, &rm_stack);
        Ok(())
    }
}