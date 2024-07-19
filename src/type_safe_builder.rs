use std::marker::PhantomPinned;

pub struct Worker {
    workload: String, 
    memsize: u128, 
    keep_alive: bool
}
pub struct WorkerBuilder<W> {
    workload: W, 
    memsize: u128, keep_alive: bool,
}

impl<W> WorkerBuilder<W> {
    pub fn memsize(&mut self, memsize: u128) -> &mut Self {
        self.memsize = memsize;
        self
    }

    pub fn keep_alive(&mut self, keep_alive: bool) -> &mut Self {
        self.keep_alive = keep_alive;
        self
    }
}

impl WorkerBuilder<PhantomPinned> {
    pub fn new() -> Self {
        Self { workload: PhantomPinned, memsize: 1028, keep_alive: true }
    }

    pub fn workload(&mut self, workload: impl Into<String>) -> WorkerBuilder<String> {
        WorkerBuilder { 
            workload: workload.into(), 
            memsize: self.memsize, 
            keep_alive: self.keep_alive 
        }
    }
}

impl WorkerBuilder<String> {
    pub fn build(&mut self) -> Worker {
        let workload = self.workload.clone();
        Worker {
            workload: workload,
            memsize: self.memsize,
            keep_alive: self.keep_alive
        }
    }
}
// fn main() {
//     let mut builder = WorkerBuilder::new();
//     // Can't do this, withoout calling workload
//     // let worker = builder.memsize(1028*2).keep_alive(true).build();

//     let worker = builder.keep_alive(true).memsize(1028).workload("workload").build();
// }

