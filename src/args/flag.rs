use super::*;

/// Flag argument that doesn't receive any value. Always optional
/// 
/// Example:
/// 
/// `./example --yes`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagArg {
    pub name: String,
    pub aliases: Vec<String>
}

impl FlagArg {
    pub fn new(name: &str, aliases: Vec<&str>) -> Box<Self> {
        let mut aliases_new = Vec::with_capacity(aliases.len());

        for alias in aliases {
            aliases_new.push(String::from(alias));
        }

        Box::new(Self {
            name: String::from(name),
            aliases: aliases_new
        })
    }

    pub fn with_name(name: &str) -> Box<Self> {
        Box::new(Self {
            name: String::from(name),
            aliases: Vec::new()
        })
    }
}

impl Argument for FlagArg {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_aliases(&self) -> &Vec<String> {
        &self.aliases
    }

    fn add_alias(&mut self, alias: String) {
        if let Err(_) = self.aliases.binary_search(&alias) {
            self.aliases.push(alias);
        }
    }

    fn is_optional(&self) -> bool {
        true
    }

    fn try_parse(&self, args: &Vec<String>) -> Option<(Vec<String>, ArgumentValue)> {
        if args.len() == 0 {
            return None
        }
        
        let mut new_args = Vec::new();

        let mut names = Vec::with_capacity(self.aliases.len() + 1);

        names.push(&self.name);

        for alias in &self.aliases {
            names.push(alias);
        }

        for i in 0..args.len() {
            if let Ok(_) = names.binary_search(&&args[i]) {
                for j in i+1..args.len() {
                    new_args.push(args[j].clone());
                }

                return Some((new_args, ArgumentValue {
                    name: self.name.clone(),
                    value: self.name.clone()
                }))
            }

            new_args.push(args[i].clone());
        }

        None
    }
}
