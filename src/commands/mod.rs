use crate::errors::Error;
use crate::args::{Argument, ArgumentValue};

mod builder;

pub use builder::CommandBuilder;

pub trait Command {
    fn get_name(&self) -> &str;
    fn get_args(&self) -> &Vec<Box<dyn Argument>>;

    fn execute(&self, args: Vec<String>, values: Vec<ArgumentValue>) -> bool;

    fn parse_args(&self, mut args: Vec<String>) -> Result<Vec<ArgumentValue>, Error> {
        let mut values = Vec::new();

        for arg in self.get_args() {
            let mut found = false;

            while let Some((new_args, found_arg)) = arg.try_parse(&args) {
                args = new_args;

                values.push(found_arg);

                found = true;
            }

            if !arg.is_optional() && !found {
                return Err(Error::ArgumentRequired(arg.get_name().clone()))
            }
        }

        Ok(values)
    }
}
