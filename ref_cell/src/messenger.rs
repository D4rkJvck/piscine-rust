use std::{cell::RefCell, rc::Rc};
use crate::Worker;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'l> {
    pub logger: &'l dyn Logger,
    pub value: RefCell<usize>,
    pub max: usize,
}

impl<'l> Tracker<'l> {
    pub fn new(logger: &'l Worker, max: usize) -> Self {
        let value = RefCell::new(0);

        Self { logger, value, max }
    }

    pub fn set_value(&self, track_value: &Rc<usize>) {
        match Rc::strong_count(track_value) * 100 / self.max {
            100.. => self.logger.error("Error: you are over your quota!"),
            70..100 => self.logger.warning(
                format!(
                    "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                    track_value
                )
                .as_str(),
            ),
            _ => println!("Not working...")
        };
    }

    pub fn peek(&self, track_value: &Rc<usize>) {
        self.logger.info(&format!(
            "Info: you are using up to {}% of your quota",
            Rc::strong_count(track_value) * 100 / self.max
        ))
    }
}
