use std::time::Duration;

use timer_future::TimerFuture;

mod executor;
mod timer_future;

fn main() {
    let (executor, spawner) = executor::new_executor_and_spawner();

    spawner.spawn(async {
        println!("Spawner: Start!");
        TimerFuture::new(Duration::from_secs(2)).await;
        println!("Spawner: Finish!");
    });

    // Since the receiver is still alive even when it's done, we `drop' the variable if it exits
    drop(spawner);

    executor.run()
}
