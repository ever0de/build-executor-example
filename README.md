# build-executor-example

## Together with the Rust async book

<https://rust-lang.github.io/async-book/02_execution/01_chapter.html>

## Output

```shell
Spawner: Sending new future(s) as task(s)...
Spawner: Start!
Executor: Polling TimerFuture...
Executor: Timer still going... I should come back later
Future: Timer completed
Waker: Hey Executor, wake up! I just sent a task
Executor: Polling TimerFuture...
Spawner: Finish!
```
