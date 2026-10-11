use parking_lot::Mutex;

pub struct DisposalQueue {
    pub pending: Mutex<Vec<(u64, u64)>>, // (Resource ID, Frame to destroy on)
}

impl DisposalQueue {
    pub fn new() -> Self { Self { pending: Mutex::new(Vec::new()) } }
    pub fn dispose(&self, id: u64, frame: u64) {
        self.pending.lock().push((id, frame));
    }
    pub fn purge(&self, completed_frame: u64) {
        let mut pending = self.pending.lock();
        pending.retain(|(_, f)| *f > completed_frame);
    }
}
