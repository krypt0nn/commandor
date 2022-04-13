use crate::prelude::*;

fn get_test_args() -> Vec<String> {
    vec![
        "./example".to_string(),

        // DefaultArg
        "--default".to_string(),  "0".to_string(),
        "-d".to_string(),         "1".to_string(),
        "-default".to_string(),   "2".to_string(),

        // SetterArg
        "--setter=0".to_string(),
        "-s=1".to_string(),
        "-setter=2".to_string(),

        // SetterArg (empty value)
        "--setter=".to_string(),
        "-s=".to_string(),
        "-setter=".to_string(),

        // FlagArg
        "--flag".to_string(),
        "-f".to_string(),
        "-flag".to_string()
    ]
}

#[test]
fn test_default_arg() {
    let mut args = get_test_args();
    let default = Default::new("--default", vec!["-d", "-default"], true);

    for i in 0..3 {
        let found = default.try_parse(&args);

        assert_eq!(found.is_some(), true);

        let (new_args, arg) = found.unwrap();

        assert_eq!(arg.value, i.to_string());

        args = new_args;
    }
}

#[test]
fn test_setter_arg() {
    let mut args = get_test_args();
    let setter = Setter::new("--setter", vec!["-s", "-setter"], "=", true);

    for i in 0..6 {
        let found = setter.try_parse(&args);

        assert_eq!(found.is_some(), true);

        let (new_args, arg) = found.unwrap();

        if i < 3 {
            assert_eq!(arg.value, i.to_string());
        }

        else {
            assert_eq!(arg.value, "");
        }

        args = new_args;
    }
}

#[test]
fn test_flag_arg() {
    let mut args = get_test_args();
    let flag = Flag::new("--flag", vec!["-f", "-flag"]);

    for _ in 0..3 {
        let found = flag.try_parse(&args);

        assert_eq!(found.is_some(), true);

        let (new_args, _) = found.unwrap();

        args = new_args;
    }
}

#[test]
fn test_manager() {
    let (s, r) = std::sync::mpsc::channel::<&str>();

    let cs = s.clone();
    let ds = s.clone();

    let mut manager = Manager::new(vec![
        CommandBuilder::new("test", move |_, values: Vec<ArgumentValue>| {
            cs.send("command");

            for value in values {
                if value.name == "--message" && value.value == "hello" {
                    cs.send("hello");
                }
            }

            true
        }).with_args(vec![
            Default::with_name("--message")
        ]).build()
    ]);

    manager.set_default(move |_| {
        ds.send("default");

        true
    });

    assert_eq!(manager.get_command("test").is_some(), true);
    assert_eq!(manager.get_command("non_existing").is_none(), true);

    manager.execute(vec!["non_existing".to_string()]);

    assert_eq!(r.recv(), Ok("default"));

    manager.execute(vec!["test".to_string(), "--message".to_string(), "hello".to_string()]);

    assert_eq!(r.recv(), Ok("command"));
    assert_eq!(r.recv(), Ok("hello"));
}
