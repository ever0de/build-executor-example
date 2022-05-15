use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

use futures::Future;

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Executor: Polling TimerFuture...");

        let mut shared_state = match self.shared_state.lock() {
            Ok(state) => state,
            Err(error) => {
                eprintln!("{error}");
                return Poll::Pending;
            }
        };

        if shared_state.completed {
            return Poll::Ready(());
        }

        shared_state.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = Arc::clone(&shared_state);
        let _join_handle = thread::Builder::new()
            .name("timer_future".into())
            .spawn(move || {
                thread::sleep(duration);
                let mut shared_state = thread_shared_state.lock().unwrap();
                shared_state.completed = true;

                println!("Future: Timer completed");

                if let Some(waker) = shared_state.waker.take() {
                    waker.wake();
                }
            })
            .unwrap();

        Self { shared_state }
    }
}
