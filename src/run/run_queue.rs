use super::run::Run;
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub type RunQueue = VecDeque<Run>;

pub struct Scheduler {
    waiting_queue: Mutex<RunQueue>,
    cvar: Condvar,
    executed_queue: 
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            waiting_queue: Mutex::new(RunQueue::new()),
            cvar: Condvar::new(),
        }
    }

    pub fn add_run(&self, run: Run) {
        let mut waiting_queue = self.waiting_queue.lock().unwrap();
        waiting_queue.push_back(run);
    }

    pub fn watch(&self) {
        loop {
            if (true/* TODO */) {
                self.cvar.notify_one();
            }
        }
    }
}
