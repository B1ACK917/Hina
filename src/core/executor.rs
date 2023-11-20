use std::path::PathBuf;

use crate::core::config::{Config, Flag, RMRecord, Target};
use crate::core::error::HinaError;
use crate::core::func;
use crate::core::global::{DATA_DIR, RECYCLE};
use crate::event::base::HinaModuleRun;

#[derive(Debug, Clone)]
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
        let home_path_str = func::get_home()?;

        let work_path = func::get_current_path()?;
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
            user: func::get_user()?,
            uid: func::get_uid()?,
        })
    }

    fn run_iter(&self,
                module: &impl HinaModuleRun,
                _work_path: &PathBuf,
                _data_path: &PathBuf,
                _recycle_path: &PathBuf,
                _user: &String,
                _uid: &String,
                _flags: &Flag,
                _rm_stack: &mut Vec<RMRecord>,
                args: &Vec<String>) -> Result<(), HinaError> {
        let args_ = func::parse_args_or(args, String::from("."));
        for arg in args_ {
            let arg_path_buf = PathBuf::from(arg);
            let target = func::get_execute_target(_work_path, &arg_path_buf)?;
            module.run(
                _work_path,
                _data_path,
                _recycle_path,
                _user,
                _uid,
                _flags,
                _rm_stack,
                &target,
                args.len(),
            )?
        }
        Ok(())
    }

    pub fn run(&self) -> Result<(), HinaError> {
        func::init_data_dir(&self.data_path)?;

        let args = self.config.get_args();
        let flags = self.config.get_flags();
        let mut rm_stack = func::load_rm_stack(&self.data_path)?;

        match self.config.get_target() {
            Target::Remove(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, args)?
            }
            Target::RecycleBin(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, args)?
            }
            Target::MakeNestedDir(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, args)?
            }
            Target::Process(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, args)?
            }
            Target::Rename(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, args)?
            }
            Target::None => {}
        }

        func::save_rm_stack(&self.data_path, &rm_stack)?;
        Ok(())
    }
}