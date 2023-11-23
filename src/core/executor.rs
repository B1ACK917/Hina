use std::path::PathBuf;

use colored::Colorize;

use crate::{debug_fn, debug_info};
use crate::core::config::{Config, Flag, Module, RMRecord};
use crate::core::error::HinaError;
use crate::core::func;
use crate::core::global::{DATA_DIR, DEBUG, RECYCLE};
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
        debug_fn!(config);
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
        if args.len() > 0 {
            for arg in args {
                module.run(
                    _work_path,
                    _data_path,
                    _recycle_path,
                    _user,
                    _uid,
                    _flags,
                    _rm_stack,
                    Some(&arg),
                )?
            }
        } else {
            module.run(
                _work_path,
                _data_path,
                _recycle_path,
                _user,
                _uid,
                _flags,
                _rm_stack,
                None,
            )?
        }
        Ok(())
    }

    pub fn run(&self) -> Result<(), HinaError> {
        debug_fn!();
        func::init_data_dir(&self.data_path)?;

        let args = self.config.get_args();
        let flags = self.config.get_flags();
        let mut rm_stack = func::load_rm_stack(&self.data_path)?;

        match self.config.get_target() {
            Module::Remove(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, args)?
            }
            Module::RecycleBin(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, args)?
            }
            Module::MakeNestedDir(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, args)?
            }
            Module::Process(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, args)?
            }
            Module::Rename(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, args)?
            }
            Module::LinkConvert(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, args)?
            }

            Module::None(module) => {
                self.run_iter(module, &self.work_path, &self.data_path, &self.recycle_path, &self.user, &self.uid, flags, &mut rm_stack, args)?
            }
        }

        func::save_rm_stack(&self.data_path, &rm_stack)?;
        Ok(())
    }
}