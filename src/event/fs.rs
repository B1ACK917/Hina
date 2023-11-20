use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use crate::core::config::{Flag, RMRecord};
use crate::core::error::HinaError;
use crate::core::error::HinaError::DirReadError;
use crate::core::func::get_execute_target;
use crate::core::global::{DEBUG, MAX_RECURSIVE_DEPTH};
use crate::debugln;
use crate::event::base::HinaModuleRun;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MakeNestedDir;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Rename;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LinkConvert;

impl HinaModuleRun for MakeNestedDir {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &Flag,
           _rm_stack: &mut Vec<RMRecord>,
           _target: &PathBuf,
           _arg_num: usize,
    ) -> Result<(), HinaError> {
        MakeNestedDir::make_nested_dir(_target, _flags.parse_bool("r"))?;
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
           _flags: &Flag,
           _rm_stack: &mut Vec<RMRecord>,
           _target: &PathBuf,
           _arg_num: usize,
    ) -> Result<(), HinaError> {
        let in_str = _flags.parse_string("i");
        let out_str = _flags.parse_string("o");
        let append_str = _flags.parse_string("a");
        let num = _flags.parse_uint("n");
        let recursive = _flags.parse_bool("r");
        let rename_sym = _flags.parse_bool("s");
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
                            debugln!("Symbol link {} -> {}", filepath.display(), new_src)
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
              rename_sym: bool) -> Result<(), HinaError> {
        let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
        Rename::rename_recursive(target, in_str, out_str, append_str, num, rename_sym, 0, max_depth)?;
        Ok(())
    }
}

impl HinaModuleRun for LinkConvert {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &Flag,
           _rm_stack: &mut Vec<RMRecord>,
           _target: &PathBuf,
           _arg_num: usize,
    ) -> Result<(), HinaError> {
        let s2l = _flags.parse_bool("s2l");
        let l2s = _flags.parse_bool("l2s");
        let input = _flags.parse_string("i");
    }
}

impl LinkConvert {
    fn symlink_to_link(filepath: &PathBuf, cur_path: &PathBuf) -> Result<(), HinaError> {
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

    fn link_to_symlink()->Result<(),HinaError> {
            let meta = fs::metadata(&filepath).unwrap();
            let inode = meta.ino();
            let command = String::from(format!("find {} -inum {}", link_src.display(), inode));
            let find_str = func::execute_command(&command)?;
            let file_src_list: Vec<&str> = find_str.trim().split("\n").collect();
            let mut file_src = "";
            if file_src_list.len() == 1 {
                file_src = file_src_list[0];
                fs::remove_file(&filepath).unwrap();
                symlink(&file_src, &filepath).unwrap();
            } else if file_src_list.len() > 1 {
                println!("Multiple src found, skip link convert for {}", filepath.display());
            } else {
                println!("No src found, skip link convert for {}", filepath.display());
            }
            if *DEBUG {
                println!("{} inode num -> {}", &filepath.display(), inode);
                println!("Symbol link {} -> {}", filepath.display(), file_src);
                dbg!(&file_src_list);
            }
        }
    }

    fn convert_recursive(cur_path: &PathBuf,
                         input_path: &PathBuf,
                         convert_type: u8,
                         cur_depth: usize,
                         max_depth: usize) -> Result<(), HinaError> {
        if cur_depth > max_depth {
            return Ok(());
        }

        for entry in cur_path.read_dir().unwrap() {
            let filepath = entry.unwrap().path();
            if filepath.is_dir() {
                LinkConvert::convert_recursive(&filepath, input_path, convert_type, cur_depth + 1, max_depth)?;
            } else {
                if convert_type == 0 {
                    LinkConvert::symlink_to_link(&filepath, cur_path)?;
                } else if convert_type == 1 {}
            }
        }
        Ok(())
    }

    pub fn convert(target: &PathBuf,
                   input_path: &PathBuf,
                   convert_type: u8,
                   recursive: bool) -> Result<(), HinaError> {
        let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
        LinkConvert::convert_recursive(target, input_path, convert_type, 0, max_depth)?;
        Ok(())
    }
}

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