use std::sync::atomic::{AtomicU64, Ordering};

/// Generates unique ids.
#[derive(Debug)]
pub struct IdGenerator {
    counter: AtomicU64,
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self {
            counter: AtomicU64::new(1),
        }
    }
}

impl IdGenerator {
    /// Atomically generate a unique id.
    pub fn generate_id(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }

    /// Generate a unique id. Requires exclusive access to `self`.
    pub fn generate_id_sync(&mut self) -> u64 {
        let id = *self.counter.get_mut();
        *self.counter.get_mut() += 1;
        id
    }
}
