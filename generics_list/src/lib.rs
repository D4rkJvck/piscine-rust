#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

//_________________________
//
#[derive(Clone, Debug)]
pub struct List<T>
where
    T: Clone,
{
    pub head: Option<Node<T>>,
}

impl<T> List<T>
where
    T: Clone,
{
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
        match self.head.take() {
            Some(node) => match node.next {
                Some(next) => self.head = Some(*next),
                None => self.head = None,
            },
            None => self.head = None,
        }
    }

    pub fn len(&self) -> usize {
        let mut size = 0;
        let mut list = self.clone();

        while let Some(_) = list.head {
            size += 1;
            list.pop();
        }

        size
    }
}
