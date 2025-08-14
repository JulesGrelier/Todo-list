use crate::domain::entities::task::Task;
use crate::domain::ports::protocol_database::{ProtocolDatabase, Modifiation};

use std::fs;


pub struct JsonDatabase {
    path : String,
}

impl JsonDatabase {
    pub fn new(path : &str) -> Self {
        JsonDatabase { path : path.to_string() }
    }

    fn rewrite_json_data(&self, tasks : Vec<Task>) {
        let updated_json = serde_json::to_string_pretty(&tasks).expect("Impossible de sérialiser le JSON");
        fs::write(&self.path[..], updated_json).expect("Impossible d'écrire dans le fichier");
    }

    fn return_index_task(&self, id : &str) -> usize {
        let tasks = self.list_all();

        for (index, task) in tasks.iter().enumerate() {
            if task.id == id {
                return index;
            }
        };

        println!("Defaillance pour return_index_task");
        return 0;
    }
}

impl ProtocolDatabase for JsonDatabase {

    fn posses_task(&self, id : &str) -> bool {
        let tasks = self.list_all();

        let _ = match tasks.iter().find(|t| t.id == id) {
            Some(_) => return true,
            None => return false,
        };
    }

    fn create(&self, task : Task) {
        let mut tasks = self.list_all();
        tasks.push(task);

        self.rewrite_json_data(tasks);
    }

    fn edit_task(&self, id : &str,  modification : Modifiation) {
        let mut tasks = self.list_all();
        let index = self.return_index_task(id);

        let _ = match modification {
            Modifiation::Title(title) => tasks[index].title = title,
            Modifiation::Done(done) => tasks[index].done = done,
        };

        self.rewrite_json_data(tasks);
    }

    fn remove(&self, id : &str) {
        let mut tasks = self.list_all();
        let index = self.return_index_task(id);

        tasks.remove(index);
        self.rewrite_json_data(tasks);
    }

    fn list_all(&self) -> Vec<Task> {
        let json_data = fs::read_to_string(&self.path[..]).expect("Impossible de lire le fichier");

        let tasks : Vec<Task> = serde_json::from_str(&json_data).expect("Impossible de désérialiser le JSON");
        tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_list_all_with_empty_json() {
        let json_database = JsonDatabase::new("database.json");
        json_database.create(Task::new("title".to_string()));
        
    }
}