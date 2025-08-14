use colored::{ColoredString, Colorize};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub title : String,
    pub done : bool,
    pub id : String,
}

impl Task {

    #[allow(dead_code)]
    pub fn new(title : String) -> Self {
        Task { title, done: false, id: uuid::Uuid::new_v4().to_string() }
    }

    pub fn return_print_output(&self) -> ColoredString {
        if self.done {
            self.title.white()
        } else {
            self.title.strikethrough().truecolor(150, 150, 150)
        }
    }
}