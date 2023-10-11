#[derive(Debug, Clone)]
pub enum Action {
    Remove,
    Restore,
    EmptyTrash,
    Process,
    MakeNestedDir,
    SymlinkToLink,
    LinkToSymlink,
    None,
    ILLEGAL,
}

#[derive(Debug, Clone)]
pub struct Config {
    action: Action,
    args: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let action_: Action;
        if args.len() < 2 {
            action_ = Action::None;
        } else {
            action_ = match &args[1] as &str {
                "rm" => { Action::Remove }
                "remove" => { Action::Remove }
                "rs" => { Action::Restore }
                "restore" => { Action::Restore }
                "et" => { Action::EmptyTrash }
                "empty-trash" => { Action::EmptyTrash }
                "mkndir" => { Action::MakeNestedDir }
                "s2l" => { Action::SymlinkToLink }
                "l2s" => { Action::LinkToSymlink }
                "ps" => { Action::Process }
                _ => { Action::ILLEGAL }
            };
        }

        if matches!(action_,Action::ILLEGAL) {
            return Err("Not a legal action");
        }

        let mut args_ = Vec::new();
        if args.len() > 2 {
            for arg in &args[2..] {
                args_.push(arg.clone());
            }
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

    pub fn arg_num(&self) -> u8 {
        return self.args.len() as u8;
    }
}