pub mod messenger;

use std::{cell::RefCell, collections::HashMap, rc::Rc};
pub use self::messenger::*;

pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(track_value: usize) -> Self {
        let mapped_messages: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
        let all_messages: RefCell<Vec<String>> = RefCell::new(Vec::new());
        let track_value = Rc::new(track_value);

        Self {
            track_value,
            mapped_messages,
            all_messages,
        }
    }
}

impl Logger for Worker {
    fn info(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Info".to_string(), msg.to_string());

        self.all_messages.borrow_mut().push(msg.to_string());
    }

    fn warning(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Warning".to_string(), msg.to_string());

        self.all_messages.borrow_mut().push(msg.to_string());
    }

    fn error(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Error".to_string(), msg.to_string());

        self.all_messages.borrow_mut().push(msg.to_string());
    }
}
