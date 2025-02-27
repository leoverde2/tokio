use crate::future::Future;
use crate::loom::sync::Arc;
use crate::runtime::scheduler::multi_thread_alt::worker;
use crate::runtime::{
    blocking, driver,
    task::{self, JoinHandle},
    TaskHooks, TaskMeta,
};
use crate::util::RngSeedGenerator;

use std::fmt;

cfg_unstable_metrics! {
    mod metrics;
}

/// Handle to the multi thread scheduler
pub(crate) struct Handle {
    /// Task spawner
    pub(super) shared: worker::Shared,

    /// Resource driver handles
    pub(crate) driver: driver::Handle,

    /// Blocking pool spawner
    pub(crate) blocking_spawner: blocking::Spawner,

    /// Current random number generator seed
    pub(crate) seed_generator: RngSeedGenerator,

    /// User-supplied hooks to invoke for things
    pub(crate) task_hooks: TaskHooks,
}

impl Handle {
    /// Spawns a future onto the thread pool
    pub(crate) fn spawn<F>(me: &Arc<Self>, future: F, id: task::Id, priority: crate::TaskPriority) -> JoinHandle<F::Output>
    where
        F: crate::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        Self::bind_new_task(me, future, id, priority)
    }

    pub(crate) fn shutdown(&self) {
        self.shared.close(self);
        self.driver.unpark();
    }

    pub(super) fn bind_new_task<T>(me: &Arc<Self>, future: T, id: task::Id, priority: crate::TaskPriority) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        let (handle, notified) = me.shared.owned.bind(future, me.clone(), id, priority);

        me.task_hooks.spawn(&TaskMeta {
            #[cfg(tokio_unstable)]
            id,
            _phantom: Default::default(),
        });

        if let Some(notified) = notified {
            me.shared.schedule_task(notified, false);
        }

        handle
    }
}

cfg_unstable! {
    use std::num::NonZeroU64;

    impl Handle {
        pub(crate) fn owned_id(&self) -> NonZeroU64 {
            self.shared.owned.id
        }
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("multi_thread::Handle { ... }").finish()
    }
}
