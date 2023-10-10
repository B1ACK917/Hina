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
                       recursive: bool) {
    let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
    let cur_path = work_path.clone();
    make_nested_dir_recursive(&cur_path, 0, max_depth);
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
            make_nested_dir_recursive(&filepath, cur_depth + 1, max_depth);
        } else {
            if filepath.is_symlink() {
                let file_src = filepath.read_link().unwrap();
                fs::remove_file(&filepath).unwrap();
                if file_src.exists() {
                    fs::hard_link(&file_src, &filepath).unwrap();
                    if *DEBUG {
                        println!("Hard link {} -> {}", filepath.display(), file_src.display())
                    }
                } else {
                    if *DEBUG {
                        println!("Symbol link source {} doesn't exist", file_src.display())
                    }
                }
            }
        }
    }
}

pub fn symlink_to_link(work_path: &PathBuf,
                       recursive: bool) {
    let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
    let cur_path = work_path.clone();
    symlink_to_link_recursive(&cur_path, 0, max_depth);
}

fn link_to_symlink_recursive(cur_path: &PathBuf,
                             link_src_dir: &PathBuf,
                             cur_depth: i8,
                             max_depth: i8) {
    if cur_depth > max_depth {
        return;
    }

    for entry in cur_path.read_dir().unwrap() {
        let filepath = entry.unwrap().path();
        if filepath.is_dir() {
            make_nested_dir_recursive(&filepath, cur_depth + 1, max_depth);
        } else {
            let meta = fs::metadata(&filepath).unwrap();
            let inode = meta.ino();
            let command = String::from(format!("find {} -inum {}", link_src_dir.display(), inode));
            let find_str = func::execute_command(&command);
            let file_src_list: Vec<&str> = find_str.trim().split("\n").collect();
            let mut file_src = "";
            if file_src_list.len() > 1 {
                println!("Multiple src found, skip link convert for {}", filepath.display());
            } else {
                file_src = file_src_list[0];
                fs::remove_file(&filepath).unwrap();
                symlink(&file_src, &filepath).unwrap();
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
                       link_src_disk: &PathBuf,
                       recursive: bool) {
    let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
    let cur_path = work_path.clone();
    link_to_symlink_recursive(&cur_path, link_src_disk, 0, max_depth);
}