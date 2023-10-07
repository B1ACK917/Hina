use crate::core::config::{Action, Config};
use crate::core::func;

use std::env;
use std::path::PathBuf;
use std::process::exit;
use crate::core::func::remove;
use crate::core::global::DATA_DIR;

#[derive(Debug, Clone)]
pub struct Executor {
    config: Config,
    work_path: PathBuf,
    home_path: PathBuf,
    data_path: PathBuf,
    user: String,
}

impl Executor {
    pub fn build(config: Config) -> Executor {
        let work_path_str = env::current_dir().unwrap().display().to_string();
        let home_path_str = func::get_home();

        let work_path = PathBuf::from(work_path_str);
        let home_path = PathBuf::from(home_path_str);
        let mut data_path = home_path.clone();
        data_path.push(DATA_DIR);

        return Executor {
            config,
            work_path,
            home_path,
            data_path,
            user: func::get_user(),
        };
    }

    pub fn run(&self) {
        func::init_data_dir(&self.data_path);

        let args = self.config.get_args();
        let arg_num = self.config.arg_num();
        let mut rm_stack = func::load_rm_stack(&self.data_path);

        match self.config.get_action() {
            Action::Remove => {
                for arg in args {
                    remove(arg, &self.data_path, &self.work_path, &mut rm_stack);
                }
            }

            Action::Restore => {
                let rm_paths = func::show_rm_stack(&rm_stack);
                func::restore(rm_paths, 0);
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

        func::save_rm_stack(&self.data_path, &rm_stack);
    }
}