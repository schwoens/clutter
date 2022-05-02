use term::color;

use crate::task_handler::TaskHandler;

mod task_handler;
mod task;
mod test;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    let task_handler = match TaskHandler::init() {
        Ok(th) => th,
        Err(s) => {
            print_error(s);
            return;
        },
    };

    handle_args(task_handler, args);
}

fn handle_args(task_handler: TaskHandler, args: Vec<String>) {
    if args.len() > 1 {
        match args[1].as_str() {
            "edit" | "e" => match task_handler.edit() {
                Ok(_) => (),
                Err(e) => print_error(e),
            },
            "complete" | "c" => {
                if args.len() < 3 {
                    print_error("Missing argument <identifier>".to_string());
                    return;
                }
                task_handler.complete(&args[2]);
            },
            "list" | "l" | "" => match task_handler.list() {
                Ok(o) => println!("{}", o),
                Err(e) => print_error(e),
            },
            _ => {
                print_error("Invalid arguments".to_string());
                std::process::exit(0);
            },
        }
    } else {
        match task_handler.list() {
            Ok(o) => println!("{}", o),
            Err(e) => print_error(e),
        };
    }
}

fn print_error(message: String) {
    let mut t = term::stdout().unwrap();
    t.fg(color::BRIGHT_RED).unwrap();
    writeln!(t, "{}", message).unwrap();
    t.reset().unwrap();
}





