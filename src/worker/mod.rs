use std::collections::{VecDeque, HashMap};
use uuid::Uuid;


use crate::other_task::Task;

#[derive(Debug)]
pub struct Worker {
    pub name: String,
    // pub queue: VecDeque,
    pub db: Option<HashMap<Uuid,Task>>,
    pub task_count: Option<u16>,
}

impl Worker{
    pub fn collect_stats() {
        println!("{:?}", "Collect stats")
    }
    fn run_task() {
        println!("{:?}", "Run Task")
    }
    fn start_task() {
        println!("{:?}", "Start tasks")
    }
    fn stop_task() {
        println!("{:?}", "stop tasks")
    }
}