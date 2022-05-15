use std::{
    sync::{
        mpsc::{self, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::Context,
};

use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
    Future, FutureExt,
};

pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// future info (like waker)
pub struct Task {
    // BoxFuture -> Pin<Box<Future<..>>>
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

const MAX_QUEUED_TASKS: usize = 1_000;
pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    let (task_sender, ready_queue) = mpsc::sync_channel::<Arc<Task>>(MAX_QUEUED_TASKS);

    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        // FutureExt::boxed == `Box::pin`
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });

        println!("Spawner: Sending new future(s) as task(s)...");

        self.task_sender.send(task).expect("too many tasks queued!")
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = Arc::clone(arc_self);

        println!("Waker: Hey Executor, wake up! I just sent a task");

        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued!")
    }
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();

            if let Some(mut future) = future_slot.take() {
                let waker_ref = waker_ref(&task);
                let waker = &*waker_ref;
                let mut context = Context::from_waker(waker);

                if future.as_mut().poll(&mut context).is_pending() {
                    println!("Executor: Timer still going... I should come back later");
                    *future_slot = Some(future)
                }
            }
        }
    }
}
