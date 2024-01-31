use std::fs;
use std::os::unix::fs::{MetadataExt, symlink};
use std::path::PathBuf;

use colored::Colorize;

use hina_core::{debug_fn, debugln};
use hina_core::error::HinaError;
use hina_core::func::{execute_command, execute_command_in_terminal, get_execute_target, parse_path_or, split_and_remove_blank};
use hina_core::globals::MAX_RECURSIVE_DEPTH;
use hina_core::shared::{Flag, HinaModuleRun, RMRecord};

#[derive(Debug, Clone)]
pub struct LinkConvert;

impl HinaModuleRun for LinkConvert {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &Flag,
           _rm_stack: &mut Vec<RMRecord>,
           _arg: Option<&String>,
    ) -> Result<(), HinaError> {
        debug_fn!(_work_path,_data_path,_recycle_path,_user,_uid,_flags,_rm_stack,_arg);
        let _help = _flags.parse_bool(vec!["help"]);
        if _help {
            LinkConvert::print_help()?;
            return Ok(());
        }
        let s2l = _flags.parse_bool(vec!["s2l"]);
        let l2s = _flags.parse_bool(vec!["l2s"]);
        let recursive = _flags.parse_bool(vec!["r", "recursive"]);
        let input = PathBuf::from(_flags.parse_string(vec!["i", "input"]));
        let src_path = get_execute_target(_work_path, &input)?;

        let target = get_execute_target(_work_path, &parse_path_or(_arg, ".")?)?;
        if s2l {
            LinkConvert::convert(&target, &src_path, 0, recursive)?;
        } else if l2s {
            LinkConvert::convert(&target, &src_path, 1, recursive)?;
        }
        Ok(())
    }
}

impl LinkConvert {
    fn print_help() -> Result<(), HinaError> {
        debug_fn!();
        execute_command_in_terminal("man", vec!["hina-lc"])?;
        Ok(())
    }

    fn symlink_to_link(filepath: &PathBuf, cur_path: &PathBuf) -> Result<(), HinaError> {
        debug_fn!(filepath,cur_path);
        if filepath.is_symlink() {
            let file_src = filepath.read_link().unwrap();
            match get_execute_target(cur_path, &file_src) {
                Ok(file_src_canon) => {
                    fs::remove_file(&filepath).unwrap();
                    fs::hard_link(&file_src_canon, &filepath).unwrap();
                    debugln!("Hard link {} -> {}", filepath.display(), file_src_canon.display())
                }
                Err(err) => { println!("{:?}", err) }
            };
        }
        Ok(())
    }

    fn link_to_symlink(filepath: &PathBuf, src_path: &PathBuf) -> Result<(), HinaError> {
        debug_fn!(filepath,src_path);
        if filepath.is_symlink() {
            return Ok(());
        }
        let meta = fs::metadata(&filepath).unwrap();
        let inode = meta.ino();
        let command = String::from(format!("find {} -inum {}", src_path.display(), inode));
        let find_str = execute_command(&command)?;
        let file_src_list = split_and_remove_blank(&find_str, "\n")?;
        if file_src_list.len() == 1 {
            let file_src = &file_src_list[0];
            fs::remove_file(filepath).unwrap();
            symlink(&file_src, &filepath).unwrap();
            debugln!("{} inode num -> {}", &filepath.display(), inode);
            debugln!("Symbol link {} -> {}", &filepath.display(), file_src);
        } else if file_src_list.len() > 1 {
            debugln!("Multiple src found, skip link convert for {}", filepath.display());
        } else {
            debugln!("No src found, skip link convert for {}", filepath.display());
        }
        debugln!("{:?}",file_src_list);
        Ok(())
    }

    fn convert_recursive(cur_path: &PathBuf,
                         src_path: &PathBuf,
                         convert_type: u8,
                         cur_depth: usize,
                         max_depth: usize) -> Result<(), HinaError> {
        debug_fn!(cur_path,src_path,convert_type,cur_depth,max_depth);
        if cur_depth > max_depth {
            return Ok(());
        }

        for entry in cur_path.read_dir().unwrap() {
            let filepath = entry.unwrap().path();
            if filepath.is_dir() {
                LinkConvert::convert_recursive(&filepath, src_path, convert_type, cur_depth + 1, max_depth)?;
            } else {
                if convert_type == 0 {
                    LinkConvert::symlink_to_link(&filepath, cur_path)?;
                } else if convert_type == 1 {
                    LinkConvert::link_to_symlink(&filepath, src_path)?;
                }
            }
        }
        Ok(())
    }

    pub fn convert(target: &PathBuf,
                   src_path: &PathBuf,
                   convert_type: u8,
                   recursive: bool) -> Result<(), HinaError> {
        debug_fn!(target,src_path,convert_type,recursive);
        let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
        LinkConvert::convert_recursive(target, src_path, convert_type, 0, max_depth)?;
        Ok(())
    }
}