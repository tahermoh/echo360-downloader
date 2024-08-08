//! Credit to zkldi
//! https://gist.github.com/zkldi/d283779f4fe42567da65552c7cd9cdc6
//!
//!
//! Support for blocking or async executions while in immediate mode.
//!
//! These are backed by a tokio threadpool impl.

use std::fmt::Debug;
use std::num::Wrapping;
use std::ops::ControlFlow;
use std::sync::Arc;
use std::time::Duration;

use eframe::egui::mutex::Mutex;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;

/// A task is a way of performing long running tasks without blocking the egui thread,
/// in a way that is ergonomic for egui usage.
pub struct Task<T> {
	recv: Option<oneshot::Receiver<T>>,
	value: Option<T>,
	debounce: Option<Arc<Mutex<TaskDebounce>>>,
}

impl<T: Debug> Debug for Task<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let debounce = self.debounce.as_ref().map(|f| *f.lock());

		f.debug_struct("Task")
			.field("recv", &self.recv)
			.field("value", &self.value)
			.field("debounce", &debounce)
			.finish()
	}
}

/// Handle debouncing/deferring tasks.
#[derive(Debug, Clone, Copy)]
struct TaskDebounce {
	cur_idx: Wrapping<u16>,
	duration: Duration,
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
	fn update_debounce(&mut self) {
		let Some(db) = &mut self.debounce else {
			return;
		};

		let mut db = db.lock();

		db.cur_idx += 1;
	}

	/// Fire a task that blocks on a future. Use this for IO-bound tasks.
	pub fn fire_async<F>(&mut self, future: F)
	where
		F: core::future::Future<Output = T> + Send + 'static,
	{
		self.update_debounce();

		let (sender, recv) = oneshot::channel();

		self.recv = Some(recv);

		let db = self.debounce.clone();

		tokio::spawn(async move {
			if handle_debounce(db).await.is_break() {
				return;
			}

			let _ = sender.send(future.await);
		});
	}

	/// Fire an expensive task without blocking the main thread.
	pub fn fire<F>(&mut self, func: F)
	where
		F: FnOnce() -> T + Send + 'static,
	{
		self.update_debounce();

		let (sender, recv) = oneshot::channel();
		self.recv = Some(recv);

		let db = self.debounce.clone();

		tokio::spawn(async move {
			if handle_debounce(db).await.is_break() {
				return;
			}

			let result = tokio::task::spawn_blocking(func).await.unwrap();

			let _ = sender.send(result);
		});
	}

	/// Create a new task.
	pub fn new() -> Self {
		Self {
			recv: None,
			value: None,
			debounce: None,
		}
	}

	/// Add debouncing to this task. The code will not be ran for `duration` seconds.
	///
	/// If another fire is emitted in this time, the previous task is
	/// cancelled.
	pub fn with_debounce(mut self, duration: Duration) -> Self {
		self.debounce = Some(Arc::new(Mutex::new(TaskDebounce {
			cur_idx: Wrapping(0),
			duration,
		})));

		self
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

#[must_use = "The control flow state must be handled."]
async fn handle_debounce(debounce: Option<Arc<Mutex<TaskDebounce>>>) -> ControlFlow<(), ()> {
	if let Some(db) = debounce {
		let our_index = db.lock().cur_idx;
		let duration = db.lock().duration;

		tokio::time::sleep(duration).await;

		// we're not the right task for the job anymore
		// cancel...
		if our_index != db.lock().cur_idx {
			return ControlFlow::Break(());
		}
	}

	ControlFlow::Continue(())
}
