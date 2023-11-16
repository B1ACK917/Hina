use std::collections::HashMap;
use std::path::PathBuf;

use crate::core::config::RMRecord;
use crate::core::error::HinaError;

pub trait HinaModuleRun {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &HashMap<String, String>,
           _rm_stack: &mut Vec<RMRecord>,
           _target: &PathBuf,
           _arg_num: usize) -> Result<(), HinaError> {
        let err = format!("Function run not implemented");
        Err(HinaError::NotImplementedError(err))
    }
}
