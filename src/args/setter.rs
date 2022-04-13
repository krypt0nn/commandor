use super::*;

/// Setter argument gets its value after a special separator
/// 
/// Example:
/// 
/// `./example --setter='example value'`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetterArg {
    pub name: String,
    pub aliases: Vec<String>,
    pub delimiter: String,
    pub optional: bool
}

impl SetterArg {
    pub fn new(name: &str, aliases: Vec<&str>, delimiter: &str, optional: bool) -> Box<Self> {
        let mut aliases_new = Vec::with_capacity(aliases.len());

        for alias in aliases {
            aliases_new.push(String::from(alias));
        }

        Box::new(Self {
            name: String::from(name),
            aliases: aliases_new,
            delimiter: String::from(delimiter),
            optional
        })
    }

    pub fn with_name(name: &str) -> Box<Self> {
        Box::new(Self {
            name: String::from(name),
            aliases: Vec::new(),
            delimiter: String::from("="),
            optional: true
        })
    }

    pub fn with_delimiter(name: &str, delimiter: &str) -> Box<Self> {
        Box::new(Self {
            name: String::from(name),
            aliases: Vec::new(),
            delimiter: String::from(delimiter),
            optional: true
        })
    }
}

impl Argument for SetterArg {
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
            for name in &names {
                if args[i].len() < name.len() + self.delimiter.len() {
                    continue;
                }
                
                if &args[i][..name.len()] == name.as_str() && &args[i][name.len()..name.len() + self.delimiter.len()] == self.delimiter {
                    for j in i+1..args.len() {
                        new_args.push(args[j].clone());
                    }

                    return Some((new_args, ArgumentValue {
                        name: self.name.clone(),
                        value: args[i][name.len() + 1..].to_string()
                    }))
                }
            }

            new_args.push(args[i].clone());
        }

        None
    }
}
