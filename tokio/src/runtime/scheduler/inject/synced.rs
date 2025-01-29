#![cfg_attr(
    any(not(all(tokio_unstable, feature = "full")), target_family = "wasm"),
    allow(dead_code)
)]

use crate::{runtime::task, TaskPriority};

pub(crate) struct Synced {
    /// True if the queue is closed.
    pub(super) is_closed: bool,

    /// Linked-list head.
    pub(super) heads: [Option<task::RawTask>; TaskPriority::VALUES.len()],

    //pub(super) head: Option<task::RawTask>,
    //pub(super) tail: Option<task::RawTask>,

    /// Linked-list tail.
    pub(super) tails: [Option<task::RawTask>; TaskPriority::VALUES.len()],
}

unsafe impl Send for Synced {}
unsafe impl Sync for Synced {}

impl Synced {
    pub(super) fn pop<T: 'static>(&mut self) -> Option<task::Notified<T>> {
        let idx = self.get_highest_priority_indx();
        let task = self.heads[idx]?;

        self.heads[idx] = unsafe { task.get_queue_next() };

        if self.heads[idx].is_none() {
            self.tails[idx] = None;
        }

        unsafe { task.set_queue_next(None) };

        // safety: a `Notified` is pushed into the queue and now it is popped!
        Some(unsafe { task::Notified::from_raw(task) })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.heads.iter().all(|head| head.is_none())
    }

    pub(crate) fn get_highest_priority_indx(&self) -> usize{
        for (idx, head) in self.heads.iter().enumerate(){
            if head.is_some(){
                return idx
            }
        }
        0
    }
}
