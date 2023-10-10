use std::fs;
use std::path::PathBuf;

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
        } else {
            let mut dir = cur_path.clone();
            dir.push(&filepath.file_stem().unwrap());
            fs::create_dir(&dir).unwrap();
            dir.push(&filepath.file_name().unwrap());
            fs::rename(&filepath, &dir).unwrap();
            if *DEBUG {
                println!("{} -> {}", filepath.display(), dir.display());
            }
        }
    }
}

pub fn make_nested_dir(work_path: &PathBuf, recursive: bool) {
    let max_depth = if recursive { MAX_RECURSIVE_DEPTH } else { 0 };
    let cur_path = work_path.clone();
    make_nested_dir_recursive(&cur_path, 0, max_depth);
}

fn symlink_to_link_recursive() {}

pub fn symlink_to_link() {}

fn link_to_symlink_recursive() {}

fn find_link_src() {}

pub fn link_to_symlink() {}