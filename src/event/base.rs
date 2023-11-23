use std::path::PathBuf;

use crate::core::config::{Flag, RMRecord};
use crate::core::error::HinaError;

pub trait HinaModuleRun {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &Flag,
           _rm_stack: &mut Vec<RMRecord>,
           _arg: Option<&String>) -> Result<(), HinaError> {
        let err = format!("Function run not implemented");
        Err(HinaError::NotImplementedError(err))
    }
}
