use std::collections::HashMap;
use std::env;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::core::config::Target;
use crate::event::fs::{MakeNestedDir, Rename};
use crate::event::process::Process;
use crate::event::recycle::{RecycleBin, Remove};

pub static DEBUG: Lazy<bool> = Lazy::new(|| {
    match env::var("DEBUG") {
        Ok(_) => true,
        Err(_) => false
    }
});
pub static TARGET_MAP: Lazy<HashMap<&str, (Target, &str)>> = Lazy::new(|| {
    HashMap::from([
        ("rm", (Target::Remove(Remove), "Remove target to recycle bin.")),
        ("rb", (Target::RecycleBin(RecycleBin), "Recycle bin utils.")),
        ("mkndir", (Target::MakeNestedDir(MakeNestedDir), "Make a nested dir for each file.")),
        ("ps", (Target::Process(Process), "Process utils")),
        ("rn", (Target::Rename(Rename), "Renaming specific pattern in files/links/dirs")),
        // ("s2l", (Target::SymlinkToLink, "Convert symbol link to hard link")),
        // ("l2s", (Target::LinkToSymlink, "Convert hard link to symbol link")),
    ])
});
// pub static HELP_DICT: Lazy<HashMap<Target, &str>> = Lazy::new(|| {
//     HashMap::from([])
// });
pub static DATA_DIR: &str = ".hina";
pub static RM_STACK: &str = "rm.stack";
pub static RECYCLE: &str = "RecycleBin";
pub static RAND_STR_LEN: usize = 16;
pub static MAX_RECURSIVE_DEPTH: usize = 64;
pub static MEM_EXTRACT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<name>\S+):\s+(?P<amount>\d+) kB").unwrap());