#[derive(Debug, Clone, PartialEq)]
pub struct Boss {
    name: String,
    age: u8,
}

impl Boss {
    pub fn new(n: &str, age: u8) -> Self {
        let name = n.to_owned();
        Boss { name, age }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }
}
