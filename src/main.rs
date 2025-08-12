mod input_database;
mod output_database;
mod task;
mod task_manager;

use crate::input_database::input_database::get_tasks;
use crate::output_database::output_database::write_tasks_in_database;
use crate::task_manager::TaskManager;

fn main() {
    const DATABASE_PATH : &'static str = "database.txt";

    let tasks = get_tasks(DATABASE_PATH);
    let mut task_manager = TaskManager::new(tasks);

    loop {
        println!("Say that you want !");
        
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error input user");

        input.pop(); //Enleve retour à la ligne en trop

        let keyword_and_request = divide_input(&input);

        match keyword_and_request[0] {
            "add" => task_manager.add_task(keyword_and_request[1], false),
            "done" => task_manager.mark_task_done(keyword_and_request[1]),
            "print" => task_manager.print_tasks(),
            "save" => write_tasks_in_database(DATABASE_PATH, &task_manager.tasks),
            "quit" | _ => break,
        }
    }

    write_tasks_in_database(DATABASE_PATH, &task_manager.tasks);
}

fn divide_input(input : &String) -> [&str;2] {
    let index_first_space = match input.find(' ') {
        Some(index) => index,
        None => return [input.as_str(), ""],
    };

    let keyword = &input[..index_first_space];
    let request = &input[(index_first_space+1)..];

    [keyword, request]
}