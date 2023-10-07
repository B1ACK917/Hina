#[derive(Debug, Clone)]
pub enum Action {
    Remove,
    Process,
    None,
}

#[derive(Debug, Clone)]
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

        let mut args_ = Vec::new();
        for arg in &args[2..] {
            args_.push(arg.clone());
        }
        let config = Config { action: action_, args: args_ };

        return Ok(config);
    }

    pub fn get_action(&self) -> &Action {
        return &self.action;
    }

    pub fn get_args(&self) -> &Vec<String> {
        return &self.args;
    }

    pub fn arg_num(&self)->u8 {
        return self.args.len() as u8;
    }
}