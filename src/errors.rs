#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// When `Manager.execute()` was called with 0 arguments
    TooFewArguments,

    /// When the command wasn't found
    CommandNotFound(String),

    /// When tried to execute a command without required argument
    ArgumentRequired(String)
}
