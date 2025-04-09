pub enum Command {
    Add(String),
    List,
    Toogle(u32),
    Remove(u32),
    Help,
}

pub fn parse_args() -> Command {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => Command::Help,
        2 => match args[1].to_string().as_str() {
            "list" => Command::List,
            "help" => Command::Help,
            _ => Command::Help,
        },
        3 => {
            let arg1 = args[1].to_string();
            let arg2 = args[2].to_string();
            match arg1.as_str() {
                "add" => Command::Add(arg2),
                "remove" => Command::Remove(arg2.parse().unwrap()),
                "toggle" => Command::Toogle(arg2.parse().unwrap()),
                _ => Command::Help,
            }
        }

        _ => Command::Help,
    }
}
