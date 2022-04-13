pub mod args;
pub mod commands;

use commands::Command;

mod errors;
mod tests;

pub use errors::Error;

pub mod prelude {
    pub use crate::args::{
        DefaultArg as Default,
        SetterArg as Setter,
        FlagArg as Flag,
        Argument,
        ArgumentValue
    };

    pub use crate::commands::{Command, CommandBuilder};
    pub use crate::errors::Error;
    pub use super::Manager;
}

pub struct Manager {
    commands: Vec<Box<dyn Command>>,
    default: Option<Box<dyn Fn(Vec<String>) -> bool>>
}

impl Manager {
    pub fn new(commands: Vec<Box<dyn Command>>) -> Self {
        Manager {
            commands,
            default: None
        }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) -> bool {
        match self.get_command(command.get_name()) {
            Some(_) => false,
            None => {
                self.commands.push(command);

                true
            }
        }
    }

    pub fn get_command(&self, name: &str) -> Option<&Box<dyn Command>> {
        for command in &self.commands {
            if command.get_name() == name {
                return Some(command)
            }
        }

        None
    }

    pub fn set_default<T: Fn(Vec<String>) -> bool + 'static>(&mut self, default: T) {
        self.default = Some(Box::new(default));
    }

    pub fn execute(&self, args: Vec<String>) -> Result<bool, Error> {
        if args.len() == 0 {
            return match &self.default {
                Some(default) => Ok(default(args)),
                None => Err(Error::TooFewArguments)
            }
        }

        match self.get_command(args[0].as_str()) {
            None => {
                match &self.default {
                    Some(default) => Ok(default(args)),
                    None => Err(Error::CommandNotFound(args[0].clone()))
                }
            },
            Some(command) => {
                match command.parse_args(args[1..].to_vec()) {
                    Ok(values) => Ok(command.execute(args, values)),
                    Err(err) => Err(err)
                }
            }
        }
    }
}
