use std::collections::HashMap;
use std::env;

use indexmap::IndexMap;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::core::config::Module;
use crate::event::fs::{LinkConvert, MakeNestedDir, Rename};
use crate::event::process::Process;
use crate::event::recycle::{RecycleBin, Remove};

pub static DEBUG: Lazy<bool> = Lazy::new(|| {
    match env::var("DEBUG") {
        Ok(_) => true,
        Err(_) => false
    }
});
pub static MODULE_MAP: Lazy<HashMap<&str, Module>> = Lazy::new(|| {
    HashMap::from([
        ("rm", Module::Remove(Remove)),
        ("rb", Module::RecycleBin(RecycleBin)),
        ("mkndir", Module::MakeNestedDir(MakeNestedDir)),
        ("rn", Module::Rename(Rename)),
        ("lc", Module::LinkConvert(LinkConvert)),
        ("ps", Module::Process(Process)),
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
macro_rules! debug_info {
    () => {
        eprint!("[{}][{}]", "DEBUG".green(), format!("{}:{}", file!(), line!()).cyan());
    };
}

#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {{
        if *DEBUG {
            debug_info!();
            eprint!(" ");
            eprintln!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! debug_fn {
    ($($expression:expr), *) => {
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        if *DEBUG {
            debug_info!();
            eprint!(" Calling {}(),", name.strip_suffix("::f").unwrap());
            $(
                {
                    eprint!(" {:?} = {:?}", stringify!($expression), &$expression);
                }
            )*
            eprintln!();
        }
    };
}

#[macro_export]
macro_rules! debug_var {
    ($($expression:expr), *) => (
        if *DEBUG {
            $(
                {
                    debug_info!();
                    eprint!(" ");
                    eprint!("{:?} = {:#?}",stringify!($expression),&$expression);
                }
            )*
        }
    )
}