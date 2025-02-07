// static lifetimes means that a reference can live as long as duration of the program. 

fn main() {
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
}