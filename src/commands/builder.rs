use super::Command;
use crate::args::{Argument, ArgumentValue};

pub struct CommandBuilder {
    name: String,
    args: Vec<Box<dyn Argument>>,
    callback: Box<dyn Fn(Vec<String>, Vec<ArgumentValue>) -> bool>
}

impl CommandBuilder {
    /// ```
    /// use commandor::prelude::*;
    /// 
    /// fn main() {
    ///     let manager = Manager::new(vec![
    ///         CommandBuilder::new("test", |_, _| {
    ///             println!("Hello, World!");
    /// 
    ///             true
    ///         }).build()
    ///     ]);
    /// 
    ///     manager.execute(std::env::args().skip(1).collect());
    /// }
    /// ```
    pub fn new<T: Fn(Vec<String>, Vec<ArgumentValue>) -> bool + 'static>(name: &str, callback: T) -> Self {
        Self {
            name: String::from(name),
            callback: Box::new(callback),
            args: Vec::new()
        }
    }

    pub fn with_args(mut self, args: Vec<Box<dyn Argument>>) -> Self {
        self.args = args;
        
        self
    }

    pub fn build(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Command for CommandBuilder {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    
    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, args: Vec<String>, values: Vec<ArgumentValue>) -> bool {
        (self.callback)(args, values)
    }
}
