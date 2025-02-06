fn main() {
    // Strings are stored as a collection of UTF-8 encoded bytes.

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1  + &s2;

    println!("{}{}", s3, s2)
}
