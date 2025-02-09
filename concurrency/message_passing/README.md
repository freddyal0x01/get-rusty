# Using Message Passing to Transfer Data Between Threads

One increasingly popular approach to ensuring safe concurrency is message
passing, where threads or actors communicate by sending each other messages
containing data.

To accomplish message-sending concurrency, Rust’s standard library provides an
implementation of channels. A channel is a general programming concept by which
data is sent from one thread to another.

A channel has two halves: a transmitter and a receiver. The transmitter half is
the upstream location where you put rubber ducks into the river, and the
receiver half is where the rubber duck ends up downstream.

One part of your code calls methods on the transmitter with the data you want to
send, and another part checks the receiving end for arriving messages. A channel
is said to be closed if either the transmitter or receiver half is dropped.

Example Code:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
```

We’re using thread::spawn to create a new thread and then using move to move tx
into the closure so the spawned thread owns tx. The spawned thread needs to own
the transmitter to be able to send messages through the channel. The transmitter
has a send method that takes the value we want to send. The send method returns
a Result<T, E> type, so if the receiver has already been dropped and there’s
nowhere to send a value, the send operation will return an error.

If we want to receive the message we will need to have the receiver get the
message.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
```

Receiver Functions:

1. `recv`: Short for receive, it blocks the main thread’s execution and wait
   until a value is sent down the channel. Once a value is sent, recv will
   return it in a Result<T, E>. When the transmitter closes, recv will return an
   error to signal that no more values will be coming.
2. `try_recv`: Doesn't block the main thread execution, it will immediately
   return a Result<T, E> immediately: an Ok value holding a message if one is
   available and an Err value if there aren’t any messages this time.
   - Using `try_recv` is useful if this thread has other work to do while
     waiting for messages: we could write a loop that calls try_recv every so
     often, handles a message if one is available, and otherwise does other work
     for a little while until checking again.

## Channels & Ownership Transference

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
```

Problem: We are trying to send a variable in the spawned thread after we already
sent it down the channel. A compile time error will occur because the `send`
function takes ownership of its parameter, and when the value is moved, the
receiver takes ownership of it. This stops us from accidentally using the value
again after sending it; the ownership system checks that everything is okay.

## Sending Multiple Values and Seeing the Receiver Waiting

Spawned thread example that sends multiple messages.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
```

This time, the spawned thread has a vector of strings that we want to send to
the main thread. We iterate over them, sending each individually, and pause
between each by calling the thread::sleep function with a Duration value of 1
second.

In the main thread, we’re not calling the recv function explicitly anymore:
instead, we’re treating rx as an iterator. For each value received, we’re
printing it. When the channel is closed, iteration will end.

## Creating Multiple Producers by Cloning the Transmitter

`mpsc` was an acronym for multiple producer, single consumer.

```rust
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    // --snip--
```

This time, before we create the first spawned thread, we call clone on the
transmitter. This will give us a new transmitter we can pass to the first
spawned thread. We pass the original transmitter to a second spawned thread.
This gives us two threads, each sending different messages to the one receiver.

We will now get messages from the 2 thread in a nondeterministic order.
