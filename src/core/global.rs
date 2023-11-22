use std::collections::HashMap;
use std::env;

use once_cell::sync::Lazy;
use regex::Regex;
use indexmap::IndexMap;

use crate::core::config::Target;
use crate::event::fs::{LinkConvert, MakeNestedDir, Rename};
use crate::event::process::Process;
use crate::event::recycle::{RecycleBin, Remove};

pub static DEBUG: Lazy<bool> = Lazy::new(|| {
    match env::var("DEBUG") {
        Ok(_) => true,
        Err(_) => false
    }
});
pub static TARGET_MAP: Lazy<HashMap<&str, Target>> = Lazy::new(|| {
    HashMap::from([
        ("rm", Target::Remove(Remove)),
        ("rb", Target::RecycleBin(RecycleBin)),
        ("mkndir", Target::MakeNestedDir(MakeNestedDir)),
        ("rn", Target::Rename(Rename)),
        ("lc", Target::LinkConvert(LinkConvert)),
        ("ps", Target::Process(Process)),
    ])
});

pub static HELP_DICT: Lazy<IndexMap<&str, IndexMap<&str, &str>>> = Lazy::new(|| {
    IndexMap::from([
        ("Use Hina recycle bin to remove/restore files", IndexMap::from([
            ("rm", "Remove target to recycle bin."),
            ("rb", "Operations on Hina recycle bin, list bin/restore/etc.")
        ])),
        ("Hina operations on filesystem", IndexMap::from([
            ("mkndir", "Make nested directories for each single file."),
            ("rn", "Batch renaming function, can also rename symbol links by set -s."),
            ("lc", "Link convertor, can convert symlink to hardlink and can also revert it.")
        ])),
        ("Powerful process utils", IndexMap::from([
            ("ps", "Advanced process checker, can see swap/pss/rss utilization and track process ancestor."),
        ])),
    ])
});
pub static DATA_DIR: &str = ".hina";
pub static RM_STACK: &str = "rm.stack";
pub static RECYCLE: &str = "RecycleBin";
pub static RAND_STR_LEN: usize = 16;
pub static MAX_RECURSIVE_DEPTH: usize = 64;
pub static MEM_EXTRACT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<name>\S+):\s+(?P<amount>\d+) kB").unwrap());

#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {{
        if *DEBUG {
            print!("[DEBUG] [{}:{}]: ",file!(),line!());
            println!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! debug_fn {
    ($($expression:expr), *) => (
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        if *DEBUG {
            print!("[DEBUG] [{}:{}]: ",file!(),line!());
            print!("Calling {}(),", name.strip_suffix("::f").unwrap());
            $(
                {
                    print!(" {:?} = {:?}", stringify!($expression), &$expression)
                }
            )*
            println!()
        }
    )
}

#[macro_export]
macro_rules! debug_var {
    ($($expression:expr), *) => (
        $(
            {
                debugln!("{:?} = {:#?}",
                     stringify!($expression),
                     &$expression)
            }
        )*
    )
}