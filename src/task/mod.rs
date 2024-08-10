//! Credit to zkldi
//! https://gist.github.com/zkldi/d283779f4fe42567da65552c7cd9cdc6
//!
//!
//! Support for blocking or async executions while in immediate mode.
//!
//! These are backed by a tokio threadpool impl.

use std::fmt::Debug;

use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;

/// A task is a way of performing long running tasks without blocking the egui thread,
/// in a way that is ergonomic for egui usage.
#[derive(Default)]
pub struct Task<T> {
    recv: Option<oneshot::Receiver<T>>,
    value: Option<T>,
}

impl<T: Debug> Debug for Task<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Task")
            .field("recv", &self.recv)
            .field("value", &self.value)
            .finish()
    }
}

/// What's the state of this task?
pub enum TaskState<T> {
    /// Nothing has been fired yet. Call [`Task::fire_future`] or [`Task::fire_blocking`]
    /// to start processing something.
    NotFired,
    /// The task is processing. The previous result is provided here, incase you wish to
    /// render that while you are loading another result.
    Loading(Option<T>),
    /// The task finished, here's the value!
    Ok(T),
}

impl<T> TaskState<T> {
    /// Get the result of this task. If we are currently working out new results, this
    /// function returns the previous result we had while it's worked on.
    ///
    /// This is useful for 0-downtime displays of data.
    pub fn get(self) -> Option<T> {
        match self {
            TaskState::NotFired => None,
            TaskState::Loading(t) => t,
            TaskState::Ok(t) => Some(t),
        }
    }

    /// Get the result of this task without showing stale data.
    ///
    /// That is, if a new [`Task::fire`] is done, this function will return
    /// None until that fire finishes.
    pub fn get_no_stale(self) -> Option<T> {
        match self {
            TaskState::NotFired => None,
            TaskState::Loading(_) => None,
            TaskState::Ok(t) => Some(t),
        }
    }
}

impl<T> Task<T>
where
    T: Send + 'static,
{
    pub fn get(&mut self) -> Option<&T> {
        self.state().get_no_stale()
    }

    /// Fire a task that blocks on a future. Use this for IO-bound tasks.
    pub fn fire_async<F>(&mut self, future: F)
    where
        F: core::future::Future<Output = T> + Send + 'static,
    {
        let (sender, recv) = oneshot::channel();

        self.recv = Some(recv);

        tokio::spawn(async move {
            let _ = sender.send(future.await);
        });
    }

    /// Fire an expensive task without blocking the main thread.
    pub fn fire<F>(&mut self, func: F)
    where
        F: FnOnce() -> T + Send + 'static,
    {
        let (sender, recv) = oneshot::channel();
        self.recv = Some(recv);

        tokio::spawn(async move {
            let result = tokio::task::spawn_blocking(func).await.unwrap();

            let _ = sender.send(result);
        });
    }

    /// Get the state of this task. Returns an error if the background task failed.
    pub fn try_state(&mut self) -> Result<TaskState<&T>, ()> {
        let v = match &mut self.recv {
            Some(recv) => match recv.try_recv() {
                Ok(v) => {
                    self.value = Some(v);
                    self.recv = None;

                    TaskState::Ok(self.value.as_ref().unwrap())
                }
                Err(TryRecvError::Empty) => TaskState::Loading(self.value.as_ref()),
                Err(TryRecvError::Closed) => {
                    return Err(());
                }
            },
            None => {
                if self.value.is_some() {
                    // is_some -> unwrap as a borrowck hack
                    TaskState::Ok(self.value.as_ref().unwrap())
                } else {
                    TaskState::NotFired
                }
            }
        };

        Ok(v)
    }

    /// Get the state of this task. Panics if the background task fails.
    pub fn state(&mut self) -> TaskState<&T> {
        self.try_state().expect("background task panicked")
    }
}
