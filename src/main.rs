use chrono::prelude::*;
use std::collections::HashMap;

use std::io::{self, Write};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Command {
    Add(String),
    Exit,
    List,
    Complete(usize),
    Help,
}

#[derive(Debug, Hash)]
struct Task {
    desc: String,
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
}

impl TryFrom<String> for Command {
    type Error = &'static str;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let linput = input.to_lowercase();

        if linput.starts_with("add") {
            let task = input[3..].trim();
            Ok(Command::Add(task.to_string()))
        } else if linput.starts_with("fin") {
            let task = input[3..].trim().to_string();
            let task_id = match task.parse::<usize>() {
                Ok(i) => i,
                Err(_) => return Err("format error for task_id"),
            };
            Ok(Command::Complete(task_id))
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
    for cmd in Command::iter() {
        match cmd {
            Command::Add(_) => println!("add\tadds a task"),
            Command::Complete(_) => println!("fin\tmarks a task as complete"),
            Command::Help => println!("help\tprints this message"),
            Command::Exit => println!("done\tends the main loop"),
            Command::List => println!("list\tlists the known tasks"),
        }
    }
}

fn add_task(tasks: &mut HashMap<usize, Task>, desc: String) {
    let task = Task {
        desc,
        start: Utc::now(),
        end: None,
    };
    let task_id = tasks.len();

    tasks.insert(task_id, task);
}

fn complete_task(tasks: &mut HashMap<usize, Task>, task_id: usize) {
    if let Some(task) = tasks.get_mut(&task_id) {
        task.end = Some(Utc::now());
    }
    //match tasks.get_mut(&task_id) {
    //Some(task) => task.end = Some(Utc::now()),
    //None => (),
    //}
}

fn print_tasks(tasks: &HashMap<usize, Task>) {
    for (tid, task) in tasks.iter() {
        if task.end.is_none() {
            println!("{}\t{}", tid, task.desc);
        }
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
    let mut tasks: HashMap<usize, Task> = HashMap::new();

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
                Command::Add(task) => add_task(&mut tasks, task),
                Command::Complete(task) => complete_task(&mut tasks, task),
                Command::Exit => break,
                Command::List => print_tasks(&tasks),
                Command::Help => print_help(),
            },
            Err(msg) => println!("{}", msg),
        }
    }

    print_tasks(&tasks);
}
