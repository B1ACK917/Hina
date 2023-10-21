use std::fs;
use std::os::unix::fs::{MetadataExt, symlink};
use std::path::PathBuf;

use crate::core::func;
use crate::core::global::{DEBUG, MAX_RECURSIVE_DEPTH};

fn make_nested_dir_recursive(cur_path: &PathBuf,
                             cur_depth: i8,
                             max_depth: i8) {
    if cur_depth > max_depth {
        return;
    }

    for entry in cur_path.read_dir().unwrap() {
        let filepath = entry.unwrap().path();
        if filepath.is_dir() {
            make_nested_dir_recursive(&filepath, cur_depth + 1, max_depth);
        }
    }

    for entry in cur_path.read_dir().unwrap() {
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

pub fn make_nested_dir(work_path: &PathBuf,
                       input_path: &PathBuf,
                       recursive: bool) {
    let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
    let target = func::get_execute_target(work_path, input_path);
    make_nested_dir_recursive(&target, 0, max_depth);
}

fn symlink_to_link_recursive(cur_path: &PathBuf,
                             cur_depth: i8,
                             max_depth: i8) {
    if cur_depth > max_depth {
        return;
    }

    for entry in cur_path.read_dir().unwrap() {
        let filepath = entry.unwrap().path();
        if filepath.is_dir() {
            symlink_to_link_recursive(&filepath, cur_depth + 1, max_depth);
        } else {
            if filepath.is_symlink() {
                let file_src = filepath.read_link().unwrap();
                let file_src_canon = func::get_execute_target(cur_path, &file_src);
                fs::remove_file(&filepath).unwrap();
                if file_src_canon.exists() {
                    fs::hard_link(&file_src_canon, &filepath).unwrap();
                    if *DEBUG {
                        println!("Hard link {} -> {}", filepath.display(), file_src_canon.display())
                    }
                } else {
                    if *DEBUG {
                        println!("Symbol link source {} doesn't exist", file_src_canon.display())
                    }
                }
            }
        }
    }
}

pub fn symlink_to_link(work_path: &PathBuf,
                       input_path: &PathBuf,
                       recursive: bool) {
    let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
    let target = func::get_execute_target(work_path, input_path);
    symlink_to_link_recursive(&target, 0, max_depth);
}

fn link_to_symlink_recursive(cur_path: &PathBuf,
                             link_src: &PathBuf,
                             cur_depth: i8,
                             max_depth: i8) {
    if cur_depth > max_depth {
        return;
    }

    for entry in cur_path.read_dir().unwrap() {
        let filepath = entry.unwrap().path();
        if filepath.is_dir() {
            link_to_symlink_recursive(&filepath, link_src, cur_depth + 1, max_depth);
        } else {
            let meta = fs::metadata(&filepath).unwrap();
            let inode = meta.ino();
            let command = String::from(format!("find {} -inum {}", link_src.display(), inode));
            let find_str = func::execute_command(&command);
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
}

pub fn link_to_symlink(work_path: &PathBuf,
                       input_path: &PathBuf,
                       link_src_disk: &PathBuf,
                       recursive: bool) {
    let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
    let target = func::get_execute_target(work_path, input_path);
    let link_src = func::get_execute_target(work_path, link_src_disk);
    link_to_symlink_recursive(&target, &link_src, 0, max_depth);
}

pub fn rename_recursive(cur_path: &PathBuf,
                        in_str: &String,
                        out_str: &String,
                        cur_depth: i8,
                        max_depth: i8) {
    if cur_depth > max_depth {
        return;
    }

    for entry in cur_path.read_dir().unwrap() {
        let filepath = entry.unwrap().path();
        if filepath.is_dir() {
            rename_recursive(&filepath, in_str, out_str, cur_depth + 1, max_depth);
        } else {
            let filename = filepath.file_name().unwrap().to_str().unwrap();
            let mut new_path = filepath.parent().unwrap().to_path_buf();
            if filename.contains(in_str) {
                let new_filename = filename.replace(in_str, out_str);
                new_path.push(new_filename);
                fs::rename(&filepath, &new_path).unwrap();
                println!("{} -> {}", &filepath.display(), &new_path.display())
            }
        }
    }
}

pub fn rename(work_path: &PathBuf,
              input_path: &PathBuf,
              in_str: &String,
              out_str: &String,
              recursive: bool) {
    let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
    let target = func::get_execute_target(work_path, input_path);
    rename_recursive(&target, in_str, out_str, 0, max_depth);
}