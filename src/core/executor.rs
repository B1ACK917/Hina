use crate::core::config::{Action, Config};
use crate::core::func;

use std::env;
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug, Clone)]
pub struct Executor {
    config: Config,
    work_path: PathBuf,
    home_path: PathBuf,
    user: String,
}

impl Executor {
    pub fn build(config: Config) -> Executor {
        let work_path_str = env::current_dir().unwrap().display().to_string();
        let home_path_str = func::get_home();
        return Executor {
            config,
            work_path: PathBuf::from(work_path_str),
            home_path: PathBuf::from(home_path_str),
            user: func::get_user(),
        };
    }

    pub fn run(self) {
        let args = self.config.get_args();
        let arg_num = self.config.arg_num();
        match self.config.get_action() {
            Action::Remove => {
                println!("OK");
            }
            Action::Process => {
                match arg_num {
                    0 => { func::show_user_all_process(&self.user); }
                    1 => { func::show_user_spec_process(&self.user, &args[0]) }
                    _ => {
                        println!("Unexpected args");
                        exit(-1);
                    }
                }
            }
            Action::None => {}
        }
    }
}