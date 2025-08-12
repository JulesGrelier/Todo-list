#[allow(dead_code)]
pub mod input_database {

        use crate::task::Task;
        use std::{fs::File, io::{ErrorKind, Read}};
        use regex::Regex;

        pub fn get_string_from_database (database_path : &str) -> String {
            let try_opening_file = File::open(database_path);

            let mut file = match try_opening_file {
                Ok(file) => file,
                Err(error) => {
                    match error.kind() {
                        ErrorKind::NotFound => return String::new(),
                        _ => panic!("Source of the problem unknown : {:?}", error),
                    }
                }
            };

            let mut buffer = String::new();
            file.read_to_string(&mut buffer).expect("Error to read file");
            buffer
        }


        pub fn get_tasks(database_path : &str) -> Vec<Task> {

            let tasks_in_text = get_string_from_database(database_path);
            let list_task_in_text : Vec<&str> = tasks_in_text.split("\n").collect();

            let mut tasks : Vec<Task> = Vec::new();
            let pattern = Regex::new(r"^[01] .+").expect("Error to create Regex pattern");

            for task_text in list_task_in_text {
                if !pattern.is_match(task_text) {
                    continue;
                }

                let title = &task_text[2..];
                let done = if &task_text[0..1] == "1" {true} else {false};
                tasks.push(Task::new(title.to_string(), done));
            }
            tasks
        }
    }