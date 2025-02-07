fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

// Generic lifetime annotations describe the relationships of the lifetimes of multiple references and how they relate to each other
// They don't change the lifetime of a reference but simply explain between different lifetimes
// Generic lifetime annotations always start with a '
// The borrow checker uses the lifetime of the variables to determine the lifetime of the return value
// The lifetime of the return value is the same as the lifetime of the variable with the shortest lifetime

// Generic lifetime annotation specifications
// &i32 - a reference
// &'a i32 - a reference with an explicit lifetime
// &'a mut i32 - a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
