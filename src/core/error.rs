#[derive(Debug)]
pub enum HinaError {
    NotImplementedError(String),
    ConfigParseError(String),
    DirCreateError(String),
    FileCreateError(String),
    FileOpenError(String),
    FileWriteError(String),
    OutOfIndexError(String),
    VarError(String),
    WorkPathError(String),
    CommandExecError(String),
    CommandParseError(String),
    BadFileError(String),
    FileNotExistError(String),
    FileExistError(String),
    DirNotEmptyError(String),
    DirReadError(String),
}