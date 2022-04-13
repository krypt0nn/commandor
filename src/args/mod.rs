mod default;
mod setter;
mod flag;

pub use default::DefaultArg;
pub use setter::SetterArg;
pub use flag::FlagArg;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentValue {
    pub name: String,
    pub value: String
}

pub trait Argument {
    fn get_name(&self) -> &String;
    fn get_aliases(&self) -> &Vec<String>;
    fn add_alias(&mut self, alias: String);
    fn is_optional(&self) -> bool;
    fn try_parse(&self, args: &Vec<String>) -> Option<(Vec<String>, ArgumentValue)>;
}
