mod cli;
mod storage;
mod task;

fn main() {
    let task = task::Task::new(1, "Buy milk".to_string());
    println!("{}", task.display());
    let command = cli::parse_args();
}
