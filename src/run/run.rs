use chrono::prelude::*;
use strum_macros::AsRefStr;
use rand::Rng;
use lipsum::lipsum;
use std::rc::Rc;
use std::collections::HashMap;
use std::any::Any;

#[derive(AsRefStr)]
pub enum RunStatus {
    Running,
    Pending,
    Finished,
    Crashed,
    Killed,
    KeyboardInterrupt,
}

pub enum RunData {
    Series(f64),
    Scalar(f64),
    Blob(Vec<u8>),
}

pub struct RunInfo {
    pub id: String,
    pub name: String,
    pub status: RunStatus,
    pub create_time: DateTime<Utc>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

pub struct Run {
    pub info: RunInfo,
    pub log: Rc<String>,
    pub data: HashMap<String, RunData>,
}

impl Run {
    pub fn new() -> Run {
        Run {
            info: RunInfo {
                id: "".into(),
                name: "".into(),
                status: RunStatus::Pending,
                create_time: Utc::now(),
                start_time: None,
                end_time: None,
            },
            log: Rc::new("".into()),
            data: HashMap::new(),
        }
    }

    pub fn random() -> Run {
        let mut run = Run::new();
        run.info.status = match rand::random::<u8>() % 5 {
            0 => RunStatus::Running,
            1 => RunStatus::Pending,
            2 => RunStatus::Finished,
            3 => RunStatus::Crashed,
            4 => RunStatus::Killed,
            _ => RunStatus::KeyboardInterrupt,
        };
        run.log = Rc::new(lipsum(rand::random::<usize>() % 1000));
        run
    }
}