use std::path::PathBuf;

use crate::core::error::HinaError;
use crate::core::executor::ExecFlag;

pub trait HinaModuleRun {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &ExecFlag,
           _rm_stack: &mut Vec<String>,
           _target: &PathBuf) -> Result<(), HinaError> {
        let err = format!("Function run not implemented");
        Err(HinaError::NotImplementedError(err))
    }
}
