// function pointers

// fn add_one(x: i32) -> i32 {
//     x + 1
// }

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

// fn main() {
//     let answer = do_twice(add_one, 5);

//     println!("The answer is: {answer}");
// }

// Returning Closures

fn main() {}

// code will compile
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// code won't compile
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }
