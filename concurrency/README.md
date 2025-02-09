# Concurrency

Read through the different readmes in the respective for a fundamental
understanding of concurrency

# Extensible Concurrency with the `Sync` and `Send` Traits

## Send Trait

`Send` allows ownership transfer of types between threads.

- A type that implements `Send` can be transferred safely from one thread to
  another.
- Most primitive types in Rust (e.g., `i32`, `f64`, `String`, etc.) implement
  `Send`.
- Types that contain non-thread-safe data (e.g., raw pointers, `Rc<T>`) do not
  implement `Send`.

Example of `Send`:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        let data = String::from("Hello,thread!");
        println!("{}", data);
    });

    handle.join().unwrap();
}
```

String is `Send`, allowing it to be safely transferred to the spawned thread.

## `Sync` Trait

`Sync` allows shared access to a type across multiple threads.

- A type is `Sync` if it is safe to be referenced from multiple threads
  simultaneously.
- This typically means that immutable references (`&T`) to the type can be
  shared between threads without data races.
- Primitive types (e.g., `i32`, `f64`, references to `Send` types like
  `&Mutex<T>`) are `Sync`.
- `Rc<T>` is not `Sync` because it is not thread-safe, but `Arc<T>` is because
  it implements atomic reference counting.

Example of `Sync`:

```rust
use std::sync::Arc;
use std::thread;

fn main() { let data = Arc::new(42);

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                println!("Value: {}", *data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

}
```

In this example, `Arc<T>` allows multiple threads to safely share the same value
without data races.

## Extending Concurrency with Send and Sync

- Send enables concurrency by allowing data to be moved between threads,
  ensuring no two threads access the same mutable data simultaneously.
- Sync allows data to be shared between threads, enabling efficient concurrent
  reads while preventing unsafe concurrent mutations.

### How to Check if a Type is Send or Sync

You can use the following code snippets to check if a type implements `Send` or
`Sync`:

```rust
fn is_send<T: Send>() {}
fn is_sync<T: Sync>() {}

is_send::<i32>(); // i32 is Send
is_sync::<i32>(); // i32 is Sync

is_send::<Rc<i32>>(); // Compile-time error: Rc<i32> is not Send
is_sync::<Rc<i32>>(); // Compile-time error: Rc<i32> is not Sync
```

## Custom Types and Send/Sync

When creating custom types, Rust will automatically implement `Send` and `Sync`
if all fields also implement those traits. You may need to explicitly mark types
as `!Send` or `!Sync` (using `PhantomData` or unsafe code) when necessary.

## Summary

- Send → Allows ownership transfer between threads.
- Sync → Allows shared access across threads.
- They ensure memory safety in concurrent programs by leveraging Rust’s type
  system at compile time.
- Using Arc, Mutex, and RwLock with these traits enables safe concurrent
  programming.
