use std::cell::{Cell, RefCell};

pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Self {
        let drops = Cell::new(0);
        let states: RefCell<Vec<bool>> = RefCell::new(Vec::new());
        Self { drops, states }
    }

    pub fn new_worker(&self, cmd: String) -> (usize, Thread) {
        let pid = self.work_tracker();
        self.states.borrow_mut().push(false);
        (pid, Thread::new_thread(pid, cmd, self))
    }

    pub fn is_dropped(&self, pid: usize) -> bool {
        self.states.borrow()[pid]
    }

    pub fn work_tracker(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn add_drop(&self, pid: usize) {
        let states = self.states.borrow_mut()[pid];
        match states {
            false => {
                self.drops.set(self.drops.get() + 1);
                self.states.borrow_mut()[pid] = true;
            }
            true => panic!("{} is already dropped", pid),
        }
    }
}

//__________________________________________________________________________________
//

pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(pid: usize, cmd: String, parent: &'a Workers) -> Self {
        Self { pid, cmd, parent }
    }

    pub fn skill(self) {
        drop(self)
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    }
}
