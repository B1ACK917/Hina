#[derive(Debug, Clone)]
pub enum Action {
    Remove,
    Restore,
    EmptyTrash,
    Process,
    ProcessAncestor,
    MakeNestedDir,
    SymlinkToLink,
    LinkToSymlink,
    DumpMemory,
    MemoryDetail,
    None,
    Test,
    ILLEGAL,
}

#[derive(Debug, Clone)]
pub struct Config {
    action: Action,
    args: Vec<String>,
    flags: Vec<String>,
}

impl Config {
    pub fn build(input: &[String]) -> Result<Config, &'static str> {
        let action_: Action;
        if input.len() < 2 {
            action_ = Action::None;
        } else {
            action_ = match &input[1] as &str {
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
                "pa" => { Action::ProcessAncestor }
                "dm" => { Action::DumpMemory }
                "mem" => { Action::MemoryDetail }
                "test" => { Action::Test }
                _ => { Action::ILLEGAL }
            };
        }

        if matches!(action_,Action::ILLEGAL) {
            return Err("Not a legal action");
        }

        let mut args_or_flags = Vec::new();
        if input.len() > 2 {
            for entry in &input[2..] {
                args_or_flags.push(entry.clone());
            }
        }

        let (flags, args) = Config::parse_flag_and_arg(&mut args_or_flags);

        let config = Config { action: action_, args, flags };

        return Ok(config);
    }

    pub fn get_action(&self) -> &Action {
        return &self.action;
    }

    pub fn get_args(&self) -> &Vec<String> {
        return &self.args;
    }

    pub fn get_flags(&self) -> &Vec<String> {
        return &self.flags;
    }

    pub fn arg_num(&self) -> u8 {
        return self.args.len() as u8;
    }

    fn parse_flag_and_arg(input: &mut Vec<String>) -> (Vec<String>, Vec<String>) {
        let mut flags = Vec::new();
        let mut args = Vec::new();
        let _: Vec<_> = input.iter().map(
            |x| {
                if x.starts_with("-") { flags.push(x.clone()) } else { args.push(x.clone()) }
            }
        ).collect();
        return (flags, args);
    }
}