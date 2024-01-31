use std::fs;
use std::path::PathBuf;

use colored::Colorize;

use hina_core::{debug_fn, debugln};
use hina_core::error::HinaError;
use hina_core::func::{execute_command_in_terminal, get_execute_target, parse_path_or};
use hina_core::globals::MAX_RECURSIVE_DEPTH;
use hina_core::shared::{Flag, HinaModuleRun, RMRecord};

#[derive(Debug, Clone)]
pub struct MakeNestedDir;

impl HinaModuleRun for MakeNestedDir {
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
            MakeNestedDir::print_help()?;
            return Ok(());
        }
        let recursive = _flags.parse_bool(vec!["r", "recursive"]);

        let target = get_execute_target(_work_path, &parse_path_or(_arg, ".")?)?;
        MakeNestedDir::make_nested_dir(&target, recursive)?;
        Ok(())
    }
}

impl MakeNestedDir {
    fn print_help() -> Result<(), HinaError> {
        debug_fn!();
        execute_command_in_terminal("man", vec!["hina-mkndir"])?;
        Ok(())
    }

    fn make_nested_dir_recursive(cur_path: &PathBuf,
                                 cur_depth: usize,
                                 max_depth: usize) -> Result<(), HinaError> {
        debug_fn!(cur_path,cur_depth,max_depth);
        if cur_depth > max_depth {
            return Ok(());
        }

        // Dealing with dirs first to avoid infinite recursive
        match cur_path.read_dir() {
            Ok(dir_entries) => {
                for entry in dir_entries {
                    let filepath = entry.unwrap().path();
                    if filepath.is_dir() {
                        MakeNestedDir::make_nested_dir_recursive(&filepath, cur_depth + 1, max_depth)?;
                    }
                }
            }
            Err(err) => { return Err(HinaError::DirReadError(err.to_string())); }
        };

        match cur_path.read_dir() {
            Ok(dir_entries) => {
                for entry in dir_entries {
                    let filepath = entry.unwrap().path();
                    if !filepath.is_dir() {
                        let mut dir = cur_path.clone();
                        dir.push(&filepath.file_stem().unwrap());
                        if !dir.exists() {
                            fs::create_dir(&dir).unwrap();
                        }
                        dir.push(&filepath.file_name().unwrap());
                        fs::rename(&filepath, &dir).unwrap();
                        debugln!("{} -> {}", filepath.display(), dir.display());
                    }
                }
            }
            Err(err) => { return Err(HinaError::DirReadError(err.to_string())); }
        };
        Ok(())
    }

    fn make_nested_dir(target: &PathBuf,
                       recursive: bool) -> Result<(), HinaError> {
        debug_fn!(target,recursive);
        let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
        MakeNestedDir::make_nested_dir_recursive(target, 0, max_depth)?;
        Ok(())
    }
}