fn main() {
    let string1 = String::from("abcd");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// The lifetime of a return value always have to be tied to the lifetime of one of our parameters
// This is because if we pass back a reference from a function 
// it has to be a reference of something that's passed in and not something that's created in the function
fn longest<'a>(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result
}