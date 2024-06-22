use std::io;

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
        let _n = io::stdin().read_line(&mut input).unwrap();
        //println!("read {} bytes", n);
        strip(&mut input);
        //println!("read|{}|", input);
        if input.to_ascii_lowercase().starts_with("done") {
            break;
        }
        tasks.push(input);
    }

    print_tasks(&tasks);
}
