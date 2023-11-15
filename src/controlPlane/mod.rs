use uuid::Uiud;
use std::collections::{VecDeque, HashMap};
use task::{Task, TaskEvent};

struct ControlPlane {
    pending: VecDeque,
    task_map: HashMap<String, Task>,
    event_map: HashMap<String, TaskEvent>,
    workers: Vec<String>,
    worker_task_map: HashMap<String, Uuid>,
    task_worker_map: HashMap<Uuid, String>
}

impl ControlPlane {
    fn SelectWorker() {
        println!("{:?}", "Selects a worker");
    }

    fn UpdateWorker() {
        println!("{:?}", "Updates a worker");
    }

    fn UpdateWorker() {
        println!("{:?}", "Sends work? a worker");
    }
}