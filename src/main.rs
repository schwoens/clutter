use crate::task_handler::TaskHandler;

mod task_handler;
mod task;
mod date;

fn main() {

    let args = std::env::args().collect();
    let t = term::stdout().unwrap();

    let mut task_handler = TaskHandler::init(t);
    task_handler.handle_args(args)
}


