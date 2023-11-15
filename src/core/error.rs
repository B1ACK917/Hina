#[derive(Debug)]
pub enum HinaError {
    NotImplementedError(String),
    ConfigParseError(String),
    ExecutorBuildError(String),
    VarError(String),
    WorkPathError(String),
    CommandExecError(String),
    CommandParseError(String),
    BadFileError(String),
    FileNotExistError(String),
    DirNotEmptyError(String),
}