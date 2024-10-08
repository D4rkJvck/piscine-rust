type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        let grade = None;
        WorkEnvironment { grade }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Box::new(Worker {
            role,
            name,
            next: self.grade.take(),
        });

        self.grade = Some(new_worker);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        let last_worker = self.grade.take();

        if let Some(worker) = last_worker {
            self.grade = worker.next;
            return Some(worker.name);
        };

        None
    }

    pub fn last_worker(&mut self) -> Option<(String, String)> {
        match &self.grade {
            Some(worker) => Some((worker.name.clone(), worker.role.clone())),
            None => None,
        }
    }
}
