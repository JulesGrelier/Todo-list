pub mod output_database {

    use crate::task::Task;
    use std::{fs::File, io::Write};

    pub fn write_tasks_in_database(database_path : &str, tasks : &Vec<Task>) {
        let mut file = File::create(database_path)
            .expect("New database impossible to create"); //Réécris par dessus anciennes données

        for task in tasks {
            let _ = file.write(task.return_database_output().as_bytes());
        }
    }
    
}