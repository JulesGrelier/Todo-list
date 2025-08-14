use crate::domain::{entities::task::Task, ports::protocol_database::{Modifiation, ProtocolDatabase}};

pub struct TaskService<'a, Repo : ProtocolDatabase> {
    database : &'a mut Repo,
}

#[allow(dead_code)]
impl <'a, Repo : ProtocolDatabase>  TaskService<'a, Repo>{

    pub fn new(repo : &'a mut Repo) -> Self {
        Self { database: repo }
    }

    pub fn create_task(&self, task : Task) {
        if self.database.posses_task(&task.id) {
            panic!("There is already a task with this id")
        } else {
            self.database.create(task);
        }
    }

    pub fn edit_task(&self, id : &str, modification : Modifiation) {
        if self.database.posses_task(id) {
            self.database.edit_task(id, modification);
        }
    }

    pub fn remove_task(&self, id : &str) {
        if self.database.posses_task(id) {
            self.database.remove(id);
        } else {
            panic!("There are no tasks with this id")
        }
    }

    pub fn print_tasks(&self) {
        let tasks = self.database.list_all();

        for (index, task) in tasks.iter().enumerate() {
            println!("{}. {}", index.to_string(), task.return_print_output());
        }
    }
    
}