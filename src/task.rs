pub struct Task {
    id: u32,
    text: String,
    done: bool,
}

impl Task {
    pub fn new(id: u32, text: String) -> Self {
        Self {
            id,
            text,
            done: false,
        }
    }

    pub fn display(&self) -> String {
        let status = if self.done { "Done " } else { "Not done " };
        format!("{}: {}", status, self.text)
    }
}
