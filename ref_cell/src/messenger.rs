pub use std::{cell::RefCell, rc::Rc};

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
    pub fn new(logger: &'l dyn Logger, max: usize) -> Self {
        let value = RefCell::new(0);

        Self { logger, value, max }
    }

    pub fn set_value(&self, track_value: &Rc<usize>) {
        *self.value.borrow_mut() = Rc::strong_count(track_value) * 100 / self.max;

        match *self.value.borrow() {
            100.. => self.logger.error("Error: you are over your quota!"),
            70..=99 => self.logger.warning(
                format!(
                    "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                    self.value.borrow()
                )
                .as_str(),
            ),
            _ => {}
        };
    }

    pub fn peek(&self, track_value: &Rc<usize>) {
        println!("track_value: {:?}", Rc::strong_count(track_value));
        self.logger.info(&format!(
            "Info: you are using up to {}% of your quota",
            Rc::strong_count(track_value) * 100 / self.max
        ))
    }
}
