use std::{thread, time::Duration};

// Example Thread
// fn main() {
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("Number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("Number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// Example thread with the join function
// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread!");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     handle.join().unwrap();

//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// Example threads using the move closure
fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });

    handle.join().unwrap();
}
