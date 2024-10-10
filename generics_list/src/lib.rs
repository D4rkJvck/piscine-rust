#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

//_________________________
//
#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        match self.head.take() {
            Some(node) => {
                self.head = Some(Node {
                    value,
                    next: Some(Box::new(node)),
                })
            }
            None => self.head = Some(Node { value, next: None }),
        }
    }

    pub fn pop(&mut self) {
        let last = self.head.take().unwrap();
        self.head = Some(*last.next.unwrap())
    }

    pub fn len(&self) -> usize {
        let mut current = self.head.as_ref().unwrap();
        let mut size = 1;

        while !current.next.is_none() {
            size += 1;
            current = current.next.as_ref().unwrap();
        }

        size
    }
}
