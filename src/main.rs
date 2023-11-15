mod task;
mod worker;

use std::time::SystemTime;

use uuid::Uuid;
use task as other_task;

fn main() {
    let task1 = other_task::Task{
        id: Uuid::new_v4(),
        name: "Task-1".to_string(),
        state: other_task::PENDING,
        image: "Image-1".to_string(),
        memory: 1024,
        disk: 1,
        exposed_ports: None,
        port_bindings: None,
        restart_policy: "Yes".to_string(),
        start_time: None,
        end_time: None,
    };

    let task_event1 = other_task::TaskEvent{
        id: Uuid::new_v4(),
        state: other_task::PENDING,
        timestamp: SystemTime::now(),
        task: &task1,
    };

    println!("Task: {:#?}", task1);
    println!("Task Event: {:#?}", task_event1);

    let worker1 = worker::Worker{
        name: "worker-1".to_string(),
        db: None,
        task_count: None,
    };

    println!("Worker: {:#?}", &worker1);

    
}
