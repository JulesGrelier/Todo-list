use crate::domain::entities::task::Task;

#[allow(dead_code)]
pub enum Modifiation {
    Title(String),
    Done(bool),
}

pub trait ProtocolDatabase {
    fn posses_task(&self, id : &str) -> bool;
    fn create(&self, task : Task);
    fn edit_task(&self, id : &str,  modification : Modifiation);
    fn remove(&self, id : &str);
    fn list_all(&self) -> Vec<Task>;
}