use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use colored::Colorize;

use hina_core::{debug_fn, debugln};
use hina_core::error::HinaError;
use hina_core::func::{execute_command_in_terminal, get_execute_target, parse_path_or};
use hina_core::globals::MAX_RECURSIVE_DEPTH;
use hina_core::shared::{Flag, HinaModuleRun, RMRecord};

#[derive(Debug, Clone)]
pub struct Rename;

impl HinaModuleRun for Rename {
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
            Rename::print_help()?;
            return Ok(());
        }
        let in_str = _flags.parse_string(vec!["i", "input"]);
        let out_str = _flags.parse_string(vec!["o", "output"]);
        let append_str = _flags.parse_string(vec!["a", "append"]);
        let num = _flags.parse_uint(vec!["n", "num"]);
        let recursive = _flags.parse_bool(vec!["r", "recursive"]);
        let rename_sym = _flags.parse_bool(vec!["s", "symlink"]);
        let rename_dir = _flags.parse_bool(vec!["d", "dir"]);

        let target = get_execute_target(_work_path, &parse_path_or(_arg, ".")?)?;
        Rename::rename(&target, &in_str, &out_str, &append_str, num, recursive, rename_sym, rename_dir)?;
        Ok(())
    }
}

impl Rename {
    fn print_help() -> Result<(), HinaError> {
        debug_fn!();
        execute_command_in_terminal("man", vec!["hina-rn"])?;
        Ok(())
    }

    fn rename_string(name: &String,
                     in_str: &String,
                     out_str: &String,
                     append_str: &String,
                     num: usize) -> Option<String> {
        debug_fn!(name,in_str,out_str,append_str,num);
        let mut renamed = name.replace(in_str, out_str);
        renamed.insert_str(num, append_str);
        return if renamed != name.as_str() {
            Some(renamed)
        } else {
            None
        };
    }

    fn rename_recursive(cur_path: &PathBuf,
                        in_str: &String,
                        out_str: &String,
                        append_str: &String,
                        num: usize,
                        rename_sym: bool,
                        rename_dir: bool,
                        cur_depth: usize,
                        max_depth: usize) -> Result<(), HinaError> {
        debug_fn!(cur_path,in_str,out_str,append_str,num,rename_sym,cur_depth,max_depth);
        if cur_depth > max_depth {
            return Ok(());
        }
        debugln!("Working in {}",&cur_path.display());
        for entry in cur_path.read_dir().unwrap() {
            let filepath = entry.unwrap().path();
            if filepath.is_dir() {
                Rename::rename_recursive(
                    &filepath,
                    in_str,
                    out_str,
                    append_str,
                    num,
                    rename_sym,
                    rename_dir,
                    cur_depth + 1,
                    max_depth,
                )?;
                if rename_dir {
                    let filename = filepath.file_name().unwrap().to_str().unwrap().to_string();
                    let mut new_path = filepath.parent().unwrap().to_path_buf();
                    match Rename::rename_string(&filename, in_str, out_str, append_str, num) {
                        None => {}
                        Some(new_name) => {
                            new_path.push(new_name);
                            fs::rename(&filepath, &new_path).unwrap();
                            debugln!("{} -> {}", &filepath.display(), &new_path.display())
                        }
                    }
                }
            } else {
                if rename_sym && filepath.is_symlink() {
                    let src = filepath.read_link().unwrap();
                    let file_src = String::from(src.to_str().unwrap());
                    match Rename::rename_string(&file_src, in_str, out_str, append_str, num) {
                        None => {}
                        Some(new_src) => {
                            fs::remove_file(&filepath).unwrap();
                            symlink(&new_src, &filepath).unwrap();
                            debugln!("Symbol link {} -> {}", filepath.display(), new_src)
                        }
                    }
                } else if !rename_sym {
                    let filename = filepath.file_name().unwrap().to_str().unwrap().to_string();
                    let mut new_path = filepath.parent().unwrap().to_path_buf();
                    match Rename::rename_string(&filename, in_str, out_str, append_str, num) {
                        None => {}
                        Some(new_name) => {
                            new_path.push(new_name);
                            fs::rename(&filepath, &new_path).unwrap();
                            debugln!("{} -> {}", &filepath.display(), &new_path.display())
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn rename(target: &PathBuf,
              in_str: &String,
              out_str: &String,
              append_str: &String,
              num: usize,
              recursive: bool,
              rename_sym: bool,
              rename_dir: bool) -> Result<(), HinaError> {
        debug_fn!(target,in_str,out_str,append_str,num,recursive,rename_sym);
        let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
        Rename::rename_recursive(target, in_str, out_str, append_str, num, rename_sym, rename_dir, 0, max_depth)?;
        Ok(())
    }
}