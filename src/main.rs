// main.rs
mod domain {
    pub mod entities {
        pub mod task; // Cela pointe vers domain/entities/task.rs
    }

    pub mod services {
        pub mod task_services;
    }

    pub mod ports {
        pub mod protocol_database;
    }
}

mod adapters {
    pub mod json_database;
}


use crate::{adapters::json_database::JsonDatabase, domain::{
    entities::task::Task,
    //ports::protocol_database::Modifiation,
    services::task_services::TaskService
}};


fn main() {
    let mut json_database = JsonDatabase::new("database.json");
    let json_service = TaskService::new(&mut json_database);

    json_service.create_task(Task::new("aller faire du sport".to_string()));
}
