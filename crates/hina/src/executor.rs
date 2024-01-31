use std::path::PathBuf;

use colored::Colorize;

use hina_core::debug_fn;
use hina_core::error::HinaError;
use hina_core::func::{get_current_path, get_home, get_uid, get_user, init_data_dir, load_rm_stack, save_rm_stack};
use hina_core::globals::{DATA_DIR, RECYCLE};

use crate::config::Config;

#[derive(Debug)]
pub struct Executor {
    config: Config,
    work_path: PathBuf,
    data_path: PathBuf,
    recycle_path: PathBuf,
    user: String,
    uid: String,
}

impl Executor {
    pub fn build(config: Config) -> Result<Executor, HinaError> {
        debug_fn!(config);
        let home_path_str = get_home()?;
        let work_path = get_current_path()?;
        let home_path = PathBuf::from(home_path_str);
        let mut data_path = home_path;
        data_path.push(DATA_DIR);
        let mut recycle_path = data_path.clone();
        recycle_path.push(RECYCLE);

        Ok(Executor {
            config,
            work_path,
            data_path,
            recycle_path,
            user: get_user()?,
            uid: get_uid()?,
        })
    }

    pub fn run(&self) -> Result<(), HinaError> {
        debug_fn!();
        init_data_dir(&self.data_path)?;

        let args = self.config.get_args();
        let flags = self.config.get_flags();
        let mut rm_stack = load_rm_stack(&self.data_path)?;
        let module = self.config.get_target();
        if args.len() > 0 {
            for arg in args {
                module.run(&self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, Some(&arg))?
            }
        } else {
            module.run(&self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, None)?
        }

        save_rm_stack(&self.data_path, &rm_stack)?;
        Ok(())
    }
}