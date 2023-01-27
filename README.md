<h1 align="center">🦀 commandor</h1>

CLI commands manager in Rust

## Example

### CommandBuilder

```rs
use commandor::prelude::*;

fn main() {
    let manager = Manager::new(vec![
        CommandBuilder::new("greet", |_, values: Vec<ArgumentValue>| {
            for value in values {
                // Each of `value.name` will be `--name` because
                // there're no other arguments
                println!("Hello, {}", value.value);
            }

            true
        }).with_args(vec![
            Default::new("--name", vec!["-n"], false)
        ]).build()
    ]);

    match manager.execute(std::env::args().skip(1).collect()) {
        Ok(executed) => println!("Executed: {}", executed),
        
        Err(Error::TooFewArguments) => eprintln!("Arguments required"),
        Err(Error::CommandNotFound(command)) => eprintln!("Command {} not found", command),
        Err(Error::ArgumentRequired(argument)) => eprintln!("Argument {} required", argument)
    }
}
```

### Command trait

```rs
use commandor::prelude::*;

struct Greet {
    args: Vec<Box<dyn Argument>>
}

impl Greet {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            args: vec![
                Default::new("--name", vec!["-n"], false)
            ]
        })
    }
}

impl Command for Greet {
    fn get_name(&self) -> &str {
        "greet"
    }

    fn get_args(&self) -> &Vec<Box<dyn Argument>> {
        &self.args
    }

    fn execute(&self, _: Vec<String>, values: Vec<ArgumentValue>) -> bool {
        for value in values {
            // Each of `value.name` will be `--name` because
            // there're no other arguments
            println!("Hello, {}", value.value);
        }

        true
    }
}

fn main() {
    let manager = Manager::new(vec![
        Greet::new()
    ]);

    manager.execute(std::env::args().skip(1).collect());
}
```

Author: [Nikita Podvirnyy](https://github.com/krypt0nn)

Licensed under [GNU GPL 3.0](LICENSE)
