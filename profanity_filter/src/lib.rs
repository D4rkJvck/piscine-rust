pub struct Message {
    content: String,
    _user: String,
}

impl Message {
    pub fn new(content: String, _user: String) -> Self {
        Message { content, _user }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn send_ms(&self) -> Option<&str> {
        match self.get_content() {
            content if !content.contains("stupid") && content != "" => Some(&content),
            _ => None,
        }
    }
}

pub fn check_ms(msg: &Message) -> (bool, &str) {
    match msg.send_ms() {
        Some(msg) => (true, msg),
        None => (false, "ERROR: illegal"),
    }
}
