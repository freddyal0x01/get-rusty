# Using Threads to Run Code Simultaneously

In most current operating systems, an executed program’s code is run in a
process, and the operating system will manage multiple processes at once. Within
a program, you can also have independent parts that run simultaneously. The
features that run these independent parts are called threads. For example, a web
server could have multiple threads so that it could respond to more than one
request at the same time.

## Creating a new thread with `spawn`

To create a new thread, we can call the thread::spawn function and pass it a
closure containing the code we want to run in the new thread.

Example code:

```rust
fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

When the main thread of a Rust program completes, all spawned threads are shut
down, whether or not they have finished running.

## Waiting for All Threads to Finish Using join Handles

The code above not only stops the spawned thread prematurely most of the time
due to the main thread ending, but because there is no guarantee on the order in
which threads run, we also can’t guarantee that the spawned thread will get to
run at all!

We can fix the problem of the spawned thread not running or ending prematurely
by saving the return value of thread::spawn in a variable. The return type of
thread::spawn is JoinHandle. A JoinHandle is an owned value that, when we call
the join method on it, will wait for its thread to finish.

Example code:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

Calling join on the handle blocks the thread currently running until the thread
represented by the handle terminates. Blocking a thread means that thread is
prevented from performing work or exiting.

The two threads will continue alternating, but the main thread waits because of
the call to handle.join() and does not end until the spawned thread is finished.

However, if we implement `handle.join()` before the `for` loop in `main` the
main thread will wait for the spawned thread to finish and then run its `for`
loop, so the output will come from the spawned thread first and then the main
thread afterwards.

Small details, such as where join is called, can affect whether or not your
threads run at the same time.

## Using `move` Closures with Threads

We’ll often use the move keyword with closures passed to thread::spawn because
the closure will then take ownership of the values it uses from the environment,
thus transferring ownership of those values from one thread to another

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```

The closure we pass to thread::spawn takes no arguments: we’re not using any
data from the main thread in the spawned thread’s code. To use data from the
main thread in the spawned thread, the spawned thread’s closure must capture the
values it needs.

Rust infers how to capture v, and because println! only needs a reference to v,
the closure tries to borrow v. However, there’s a problem: Rust can’t tell how
long the spawned thread will run, so it doesn’t know if the reference to v will
always be valid.

Using the `move` keyword before the closure allows us to force the closure to
take ownership of the values it's using rather than allowing Rust to infer that
it should borrow the values.

Example code using `move`

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```
