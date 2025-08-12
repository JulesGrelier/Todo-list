#[derive(Debug)]
pub struct Task {
    pub title : String,
    pub done : bool,
}

impl Task {
    pub fn new (title : String, done : bool) -> Self {
        Task {title, done}
    }

    pub fn become_done (&mut self) {
        self.done = true
    }

    pub fn return_database_output(&self) -> String {
        let bool_as_str = if self.done {"1"} else {"0"};
        let text_input = format!("{} {}\n", bool_as_str, self.title);

        return text_input;
    }
}