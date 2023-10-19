use crate::run::{Run, RunData};
use std::any::Any;

pub struct Logger<'a> {
    run: &'a Run,
}

impl<'a> Logger<'a> {
    pub fn new(run: &Run) -> Logger {
        Logger {
            run,
        }
    }

    pub fn log(&self, key: &str, data: RunData) {
        match data {
            RunData::Series(_) => {}
            RunData::Scalar(_) => {}
            RunData::Blob(_) => {}
        }
    }
}