use super::*;

/// Default argument gets its value as a second value in arguments list
/// 
/// Example:
/// 
/// `./example --default 'example value'`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultArg {
    pub name: String,
    pub aliases: Vec<String>,
    pub optional: bool
}

impl DefaultArg {
    pub fn new(name: &str, aliases: Vec<&str>, optional: bool) -> Box<Self> {
        let mut aliases_new = Vec::with_capacity(aliases.len());

        for alias in aliases {
            aliases_new.push(String::from(alias));
        }

        Box::new(Self {
            name: String::from(name),
            aliases: aliases_new,
            optional
        })
    }

    pub fn with_name(name: &str) -> Box<Self> {
        Box::new(Self {
            name: String::from(name),
            aliases: Vec::new(),
            optional: true
        })
    }
}

impl Argument for DefaultArg {
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
        self.optional
    }

    fn try_parse(&self, args: &Vec<String>) -> Option<(Vec<String>, ArgumentValue)> {
        if args.len() < 2 {
            return None
        }

        let mut new_args = Vec::new();

        let mut names = Vec::with_capacity(self.aliases.len() + 1);

        names.push(&self.name);

        for alias in &self.aliases {
            names.push(alias);
        }

        for i in 0..args.len() - 1 {
            if let Ok(_) = names.binary_search(&&args[i]) {
                for j in i + 2..args.len() {
                    new_args.push(args[j].clone());
                }

                return Some((new_args, ArgumentValue {
                    name: self.name.clone(),
                    value: args[i + 1].clone()
                }))
            }

            new_args.push(args[i].clone());
        }

        None
    }
}
