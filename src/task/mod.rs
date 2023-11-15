use iota::iota;
use uuid::Uuid;
use std::{collections::HashMap, time::SystemTime};


pub type State = u8;

iota!{
    pub const PENDING: State = iota;
        , SCHEDULED
        , RUNNING
        , COMPLETED
        , FAILED

}

#[derive(Debug)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub state: State,
    pub image: String, 
    pub memory: u16,
    pub disk: u16,
    pub exposed_ports: Option<HashMap<String, String>>,
    pub port_bindings: Option<HashMap<String, String>>,
    pub restart_policy: String,
    pub start_time: Option<SystemTime>,
    pub end_time: Option<SystemTime>,
}

#[derive(Debug)]
pub struct TaskEvent<'a> {
    pub id: Uuid,
    pub state: State,
    pub timestamp: SystemTime,
    pub task: &'a Task
}