use std::env;

use once_cell::sync::Lazy;

pub const DEBUG: Lazy<bool> = Lazy::new(|| {
    match env::var("DEBUG") {
        Ok(value) => {
            if value.parse::<u8>().unwrap() > 0 { true } else { false }
        }
        Err(_) => false
    }
});

// For external package macro
#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => {{
        if *hina_core::macros::DEBUG {
            eprint!("[{}][{}]", "DEBUG".green(), format!("{}:{}", file!(), line!()).cyan());
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
        if *hina_core::macros::DEBUG {
            eprint!("[{}][{}]", "DEBUG".green(), format!("{}:{}", file!(), line!()).cyan());
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
        if *hina_core::macros::DEBUG {
            $(
                {
                    eprint!("[{}][{}]", "DEBUG".green(), format!("{}:{}", file!(), line!()).cyan());
                    eprint!(" ");
                    eprint!("{:?} = {:#?}",stringify!($expression),&$expression);
                    eprintln!();
                }
            )*
        }
    )
}

// For local use
#[macro_export]
macro_rules! debug_fn_inline {
    ($($expression:expr), *) => {
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        if *crate::macros::DEBUG {
            eprint!("[{}][{}]", "DEBUG".green(), format!("{}:{}", file!(), line!()).cyan());
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
macro_rules! debugln_inline {
    ($($arg:tt)*) => {{
        if *crate::macros::DEBUG {
            eprint!("[{}][{}]", "DEBUG".green(), format!("{}:{}", file!(), line!()).cyan());
            eprint!(" ");
            eprintln!($($arg)*);
        }
    }};
}