use chrono::prelude::*;
use strum_macros::AsRefStr;
use rand::Rng;
use lipsum::lipsum;

#[derive(AsRefStr)]
pub enum TaskStatus {
    Running,
    Pending,
    Finished,
    Crashed,
    Killed,
    KeyboardInterrupt,
}

pub struct Task {
    pub status: TaskStatus,
    pub create_time: DateTime<Utc>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub log: String,
}

impl Task {
    pub fn new() -> Task {
        Task {
            status: TaskStatus::Pending,
            create_time: Utc::now(),
            start_time: None,
            end_time: None,
            log: String::new(),
        }
    }

    pub fn random() -> Task {
        let mut task = Task::new();
        task.status = match rand::random::<u8>() % 5 {
            0 => TaskStatus::Running,
            1 => TaskStatus::Pending,
            2 => TaskStatus::Finished,
            3 => TaskStatus::Crashed,
            4 => TaskStatus::Killed,
            _ => TaskStatus::KeyboardInterrupt,
        };
        task.log = String::from(lipsum(rand::random::<usize>() % 1000));
        task
    }
}