use crate::core::func;

fn get_ps_head() -> String {
    let command = String::from("ps -ef | head -1");
    return func::execute_command(&command);
}

pub fn show_user_all_process(user: &String) {
    let command = format!("ps -ef | grep {}", user);
    let output = func::execute_command(&command);
    println!("{}{}", get_ps_head(), output);
}

pub fn show_user_spec_process(user: &String,
                              process_name: &String) {
    let command = format!("ps -ef | grep {} | grep {}", user, process_name);
    let output = func::execute_command(&command);
    println!("{}{}", get_ps_head(), output);
}