use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use crate::core::config::RMRecord;
use crate::core::error::HinaError;
use crate::core::error::HinaError::DirReadError;
use crate::core::func::{parse_flag_bool, parse_flag_string, parse_flag_u};
use crate::core::global::{DEBUG, MAX_RECURSIVE_DEPTH};
use crate::event::base::HinaModuleRun;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MakeNestedDir;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Rename;

impl HinaModuleRun for MakeNestedDir {
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
        MakeNestedDir::make_nested_dir(_target, parse_flag_bool(_flags, "r"))?;
        Ok(())
    }
}

impl MakeNestedDir {
    fn make_nested_dir_recursive(cur_path: &PathBuf,
                                 cur_depth: usize,
                                 max_depth: usize) -> Result<(), HinaError> {
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
            Err(err) => { return Err(DirReadError(err.to_string())); }
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
                        if *DEBUG {
                            println!("{} -> {}", filepath.display(), dir.display());
                        }
                    }
                }
            }
            Err(err) => { return Err(DirReadError(err.to_string())); }
        };
        Ok(())
    }

    fn make_nested_dir(target: &PathBuf,
                       recursive: bool) -> Result<(), HinaError> {
        let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
        MakeNestedDir::make_nested_dir_recursive(target, 0, max_depth)?;
        Ok(())
    }
}

impl HinaModuleRun for Rename {
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
        let in_str = parse_flag_string(_flags, "i");
        let out_str = parse_flag_string(_flags, "o");
        let append_str = parse_flag_string(_flags, "a");
        let num = parse_flag_u(_flags, "n");
        let recursive = parse_flag_bool(_flags, "r");
        let rename_sym = parse_flag_bool(_flags, "s");
        Rename::rename(_target, &in_str, &out_str, &append_str, num, recursive, rename_sym)?;
        Ok(())
    }
}

impl Rename {
    fn rename_string(name: &String,
                     in_str: &String,
                     out_str: &String,
                     append_str: &String,
                     num: usize) -> Option<String> {
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
                        cur_depth: usize,
                        max_depth: usize) -> Result<(), HinaError> {
        if cur_depth > max_depth {
            return Ok(());
        }
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
                    cur_depth + 1,
                    max_depth,
                )?;
            } else {
                if rename_sym && filepath.is_symlink() {
                    let src = filepath.read_link().unwrap();
                    let file_src = String::from(src.to_str().unwrap());
                    match Rename::rename_string(&file_src, in_str, out_str, append_str, num) {
                        None => {}
                        Some(new_src) => {
                            fs::remove_file(&filepath).unwrap();
                            symlink(&new_src, &filepath).unwrap();
                            println!("Symbol link {} -> {}", filepath.display(), new_src)
                        }
                    }
                } else if !rename_sym && !filepath.is_symlink() {
                    let filename = filepath.file_name().unwrap().to_str().unwrap().to_string();
                    let mut new_path = filepath.parent().unwrap().to_path_buf();
                    match Rename::rename_string(&filename, in_str, out_str, append_str, num) {
                        None => {}
                        Some(new_name) => {
                            new_path.push(new_name);
                            fs::rename(&filepath, &new_path).unwrap();
                            println!("{} -> {}", &filepath.display(), &new_path.display())
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
              rename_sym: bool) -> Result<(), HinaError> {
        let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
        Rename::rename_recursive(target, in_str, out_str, append_str, num, rename_sym, 0, max_depth)?;
        Ok(())
    }
}

// fn symlink_to_link_recursive(cur_path: &PathBuf,
//                              cur_depth: usize,
//                              max_depth: usize) {
//     if cur_depth > max_depth {
//         return;
//     }
//
//     for entry in cur_path.read_dir().unwrap() {
//         let filepath = entry.unwrap().path();
//         if filepath.is_dir() {
//             symlink_to_link_recursive(&filepath, cur_depth + 1, max_depth);
//         } else {
//             if filepath.is_symlink() {
//                 let file_src = filepath.read_link().unwrap();
//                 let file_src_canon = func::get_execute_target(cur_path, &file_src);
//                 fs::remove_file(&filepath).unwrap();
//                 if file_src_canon.exists() {
//                     fs::hard_link(&file_src_canon, &filepath).unwrap();
//                     if *DEBUG {
//                         println!("Hard link {} -> {}", filepath.display(), file_src_canon.display())
//                     }
//                 } else {
//                     if *DEBUG {
//                         println!("Symbol link source {} doesn't exist", file_src_canon.display())
//                     }
//                 }
//             }
//         }
//     }
// }
//
// pub fn symlink_to_link(work_path: &PathBuf,
//                        input_path: &PathBuf,
//                        recursive: bool) {
//     let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
//     let target = func::get_execute_target(work_path, input_path)?;
//     symlink_to_link_recursive(&target, 0, max_depth);
// }
//
// fn link_to_symlink_recursive(cur_path: &Result<PathBuf, HinaError>,
//                              link_src: &Result<PathBuf, HinaError>,
//                              cur_depth: usize,
//                              max_depth: usize) -> Result<(), HinaError> {
//     if cur_depth > max_depth {
//         return Ok(());
//     }
//
//     for entry in cur_path.read_dir().unwrap() {
//         let filepath = entry.unwrap().path();
//         if filepath.is_dir() {
//             link_to_symlink_recursive(&filepath, link_src, cur_depth + 1, max_depth);
//         } else {
//             let meta = fs::metadata(&filepath).unwrap();
//             let inode = meta.ino();
//             let command = String::from(format!("find {} -inum {}", link_src.display(), inode));
//             let find_str = func::execute_command(&command)?;
//             let file_src_list: Vec<&str> = find_str.trim().split("\n").collect();
//             let mut file_src = "";
//             if file_src_list.len() == 1 {
//                 file_src = file_src_list[0];
//                 fs::remove_file(&filepath).unwrap();
//                 symlink(&file_src, &filepath).unwrap();
//             } else if file_src_list.len() > 1 {
//                 println!("Multiple src found, skip link convert for {}", filepath.display());
//             } else {
//                 println!("No src found, skip link convert for {}", filepath.display());
//             }
//             if *DEBUG {
//                 println!("{} inode num -> {}", &filepath.display(), inode);
//                 println!("Symbol link {} -> {}", filepath.display(), file_src);
//                 dbg!(&file_src_list);
//             }
//         }
//     }
//     Ok(())
// }
//
// pub fn link_to_symlink(work_path: &PathBuf,
//                        input_path: &PathBuf,
//                        link_src_disk: &PathBuf,
//                        recursive: bool) {
//     let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
//     let target = func::get_execute_target(work_path, input_path);
//     let link_src = func::get_execute_target(work_path, link_src_disk);
//     link_to_symlink_recursive(&target, &link_src, 0, max_depth);
// }