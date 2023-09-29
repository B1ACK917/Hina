#[derive(Debug)]
enum Action {
    Remove,
    Process,
    None,
}

#[derive(Debug)]
pub struct Config {
    action: Action,
    args: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let action_ = match &args[1] as &str {
            "rm" => { Action::Remove }
            "proc" => { Action::Process }
            _ => { Action::None }
        };

        if matches!(action_,Action::None) {
            return Err("Not a legal action");
        }

        let mut args_=Vec::new();
        for arg in &args[2..] {
            args_.push(arg.clone());
        }
        let config = Config { action: action_, args: args_ };

        return Ok(config);
    }
}