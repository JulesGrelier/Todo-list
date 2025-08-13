#[allow(dead_code)]
use crate::task::Task;

pub struct TaskManager {
    pub tasks : Vec<Task>,
}

impl TaskManager {

    pub fn new(tasks : Vec<Task>) -> Self {
        TaskManager {tasks}
    }

    pub fn add_task(&mut self, input : &str, done : bool) {
        for title in input.split(", ") {
            if !title.is_empty() {
                self.tasks.push(Task::new(title.to_string(), done));
            }
        }
    }

    pub fn mark_task_done (&mut self, index : &str) {
        let nb : usize = match index.parse() {
            Ok(nb) => nb,
            Err(_) => {
                println!("The index must be an integer between 1 and the nb of tasks");
                return;
            }
        };

        let try_to_get_task = self.tasks.get_mut(nb-1);

        let current_task = match try_to_get_task {
            Some(task) => task,
            None => {
                println!("THe index must be an integer between 1 and the nb of tasks");
                return;
            }
        };

        current_task.become_done();
    }

    pub fn print_again(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", index+1, task.return_styling_print_output());
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::task_manager::TaskManager;

    #[test]
    fn check_add_with_many_correct_inputs() {
        let mut task_manager = TaskManager::new(Vec::new());
        task_manager.add_task("jour aux echecs, manger plus sain, etudier architecture", false);

        let mut titles: Vec<String> = Vec::new();

        for task in task_manager.tasks {
            titles.push(task.title);
        }

        assert_eq!(
            titles,
            vec!["jour aux echecs".to_string(), "manger plus sain".to_string(), "etudier architecture".to_string()]
        )
    }

    #[test]
    fn check_add_with_empty_input() {
        let mut task_manager = TaskManager::new(Vec::new());
        task_manager.add_task("jour aux echecs, , etudier architecture", false);

        let mut titles: Vec<String> = Vec::new();

        for task in task_manager.tasks {
            titles.push(task.title);
        }

        assert_eq!(
            titles,
            vec!["jour aux echecs".to_string(), "etudier architecture".to_string()]
        )
    }

    #[test]
    fn check_add_with_invalid_comma() {
        let mut task_manager = TaskManager::new(Vec::new());
        task_manager.add_task("jour aux echecs,etudier architecture", false);

        let mut titles: Vec<String> = Vec::new();

        for task in task_manager.tasks {
            titles.push(task.title);
        }

        assert_eq!(
            titles,
            vec!["jour aux echecs".to_string(), "etudier architecture".to_string()]
        )
    }
}


// pub mod print_tasks {

//     use std::{cmp::Ordering};
//     use crate::task_manager::TaskManager;

//     impl TaskManager {
//         pub fn print_tasks(&self) {

//             let spaces: Vec<usize> = vec![
//                 8, //Index_space
//                 39, //Title_space
//                 8, //Done_space
//             ];

//             print_raw(&spaces, &vec!["┌","┬","┬","┐"],     &vec!["─";spaces.len()]);
//             print_raw(&spaces, &vec!["│"; spaces.len()+1], &vec![" Index", " Title", " Done ?"]);
//             print_raw(&spaces, &vec!["├","┼","┼","┤"],     &vec!["─";spaces.len()]);

//             /*
//             ┌────────┬───────────────────────────────────────┬────────┐
//             │ Index  │ Title                                 │ Done ? │
//             ├────────┼───────────────────────────────────────┼────────┤
//             */

//             for (index, task) in self.tasks.iter().enumerate() {
//                 let done_as_str = if task.done {"Yes"} else {"Nope"};

//                 print_raw(&spaces, &vec!["│"; spaces.len()+1], &vec![
//                     &format!(" {}", (index+1).to_string()),
//                     &format!(" {}", task.title), 
//                     &format!(" {}", done_as_str),
//                 ]); //Add space in format!
//             }

//             /*
//             │ 1      │ Training X3 a week                    │ Yes    │
//             │ 2      │ To cool healthier                     │ Nope   │
//             │ 3      │ To play at chess                      │ Nope   │
//             └────────┴───────────────────────────────────────┴────────┘
//             */

//             print_raw(&spaces, &vec!["└","┴","┴","┘"], &vec!["─";spaces.len()]);

//         }
//     }

//     fn trim_string(current_string : &str, nb_characters : usize) -> String {

//         if current_string == "─" {
//             return "─".repeat(nb_characters);
//         }

//         let formatted_string = match current_string.len().cmp(&nb_characters) {
//             Ordering::Less => format!("{:<nb_characters$}", current_string),
//             Ordering::Equal => current_string.to_string(),
//             Ordering::Greater => {
//                 let new_nb_characters = nb_characters - 3;
//                 format!("{:<new_nb_characters$}...", current_string)
//             }
//         };
//         formatted_string
//     }

//     fn print_raw(spaces : &Vec<usize>, limits : &Vec<&str>, text_columns : &Vec<&str>) {
//         let nb_columns = spaces.len();

//         if limits.len() != nb_columns+1 { panic!("Incorrect number of limits") }
//         if text_columns.len() != nb_columns { panic!("Incorrect number of columns") }

//         for index in 0..nb_columns {
//             print!("{}{}", limits[index], trim_string(text_columns[index], spaces[index]));
//         }

//         println!("{}", limits[nb_columns])
//     }
// }