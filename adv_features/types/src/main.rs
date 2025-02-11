// Newtype Pattern

// use std::fmt;

// struct Wrapper(Vec<String>);

// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }

// fn main() {
//     let w = Wrapper(vec![String::from("hello"), String::from("world")]);
//     println!("w = {}", w);
// }

// Type Aliases

// type Kilometers = i32;

// fn main() {
//     let x: i32 = 5;
//     let y: Kilometers = 5;

//     println!("x + y = {}", x + y);
// }

// Never Type

// fn bar() -> ! {
//     // --snip--
// }

// fn main() {
//     bar()
// }

// DSTs

// fn main() {
//     let s1: &str = "Hello there!";
//     let s2: &str = "How's it going?";
// }
