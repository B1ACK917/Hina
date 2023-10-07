use std::process::{Command, Stdio};

pub fn get_home() -> String {
    return std::env::var("HOME").unwrap();
}

pub fn get_user() -> String {
    return std::env::var("USER").unwrap();
}

pub fn show_user_all_process(user: &String) {
    let all_process = Command::new("ps")
        .arg("-ef") // Should replace to -aux
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = Command::new("grep")
        .arg(user)
        .stdin(all_process.stdout.unwrap())
        .output()
        .unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
}

pub fn show_user_spec_process(user: &String, process_name: &String) {
    let all_process = Command::new("ps")
        .arg("-ef")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = Command::new("grep")
        .arg(user)
        .stdin(all_process.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = Command::new("grep")
        .arg(process_name)
        .stdin(output.stdout.unwrap())
        .output()
        .unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
}