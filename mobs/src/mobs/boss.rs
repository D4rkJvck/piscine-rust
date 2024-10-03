#[derive(Debug, Clone, PartialEq)]
pub struct Boss {
    pub name: String,
    pub age: u8,
}

impl Boss {
    pub fn new(n: &str, age: u8) -> Self {
        let name = n.to_owned();
        Boss { name, age }
    }
}
