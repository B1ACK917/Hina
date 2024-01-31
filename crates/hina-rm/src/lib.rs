use std::path::PathBuf;

use colored::Colorize;

use hina_core::debug_fn;
use hina_core::error::HinaError;
use hina_core::func::{execute_command, execute_command_in_terminal, gen_rand_str, get_execute_target};
use hina_core::globals::RAND_STR_LEN;
use hina_core::shared::{Flag, HinaModuleRun, RMRecord};

#[derive(Clone, Debug)]
pub struct Remove;

impl HinaModuleRun for Remove {
    fn run(&self,
           _work_path: &PathBuf,
           _data_path: &PathBuf,
           _recycle_path: &PathBuf,
           _user: &String,
           _uid: &String,
           _flags: &Flag,
           _rm_stack: &mut Vec<RMRecord>,
           _arg: Option<&String>,
    ) -> Result<(), HinaError> {
        debug_fn!(_work_path,_data_path,_recycle_path,_user,_uid,_flags,_rm_stack,_arg);
        let _help = _flags.parse_bool(vec!["help"]);
        if _help {
            Remove::print_help()?;
            return Ok(());
        }
        match _arg {
            None => {}
            Some(arg) => {
                let remove_target = get_execute_target(_work_path, &PathBuf::from(arg))?;
                let mut recycle_bin = _recycle_path.clone();
                let file_name = gen_rand_str(RAND_STR_LEN);
                recycle_bin.push(file_name.clone());

                let command = String::from(format!("mv \"{}\" \"{}\"", remove_target.display(), recycle_bin.display()));
                execute_command(&command)?;
                _rm_stack.push(RMRecord::new(
                    recycle_bin.display().to_string(),
                    remove_target.display().to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl Remove {
    fn print_help() -> Result<(), HinaError> {
        debug_fn!();
        execute_command_in_terminal("man", vec!["hina-rm"])?;
        Ok(())
    }
}