use std::io::{self, Write};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Command {
    Add(String),
    Exit,
    List,
    Help,
}

impl ToString for Command {
    fn to_string(&self) -> String {
        String::new()
    }
}

impl TryFrom<String> for Command {
    type Error = &'static str;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let linput = input.to_lowercase();

        if linput.starts_with("add") {
            let task = input[3..].trim();
            Ok(Command::Add(task.to_string()))
        } else if linput.starts_with("done") {
            Ok(Command::Exit)
        } else if linput.starts_with("list") {
            Ok(Command::List)
        } else if linput.starts_with("help") {
            Ok(Command::Help)
        } else {
            Err("Failed to detect known command")
        }
    }
}

fn print_help() {
    //cmd: &Command) {
    for cmd in Command::iter() {
        match cmd {
            Command::Add(_) => println!("add\tadds a task"),
            Command::Help => println!("help\tprints this message"),
            Command::Exit => println!("done\tends the main loop"),
            Command::List => println!("list\tlists the known tasks"),
        }
    }
}

fn print_tasks(tasks: &[String]) {
    for (i, task) in tasks.iter().enumerate() {
        println!("{}\t{}", i, task);
    }
}

fn strip(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
    }
    if s.ends_with('\r') {
        s.pop();
    }
}

fn main() {
    println!("Hello, world!");
    let mut tasks: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();

        let _n = io::stdin().read_line(&mut input).unwrap();
        //println!("read {} bytes", n);
        strip(&mut input);
        //println!("read|{}|", input);

        let parsed_cmd = Command::try_from(input);
        match parsed_cmd {
            Ok(cmd) => match cmd {
                Command::Add(task) => tasks.push(task),
                Command::Exit => break,
                Command::List => print_tasks(&tasks),
                Command::Help => print_help(),
            },
            Err(msg) => println!("{}", msg),
        }
        //if input.to_ascii_lowercase().starts_with("done") {
        //    break;
        //}
        //tasks.push(input);
    }

    print_tasks(&tasks);
}
