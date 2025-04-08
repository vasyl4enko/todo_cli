//  task.rs: структура Task, метод new, метод display

struct Task {
    id: u32,
    text: String,
    done: bool,
}

impl Task {
    fn new(text: String) -> Self {
        Self {
            id: 0,
            text,
            done: false,
        }
    }

    fn display(&self) {
        let status = if self.done { "Done " } else { "Not done " };
        println!("{}: {}", status, self.text);
    }
}
