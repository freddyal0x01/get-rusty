use std::collections::HashMap;

fn main() {
    // let blue = String::from("Blue");
    // let yellow = String::from("Yellow");

    // let mut scores = HashMap::new();

    // scores.insert(blue, 10);
    // scores.insert(yellow, 50);    

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 20);

    // scores.entry(String::from("Yellow")).or_insert(30);
    // scores.entry(String::from("Yellow")).or_insert(40);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // ["hello", "world", "wonderful", "world"]
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}