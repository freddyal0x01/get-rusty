use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Strings are stored as a collection of UTF-8 encoded bytes.

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);

    println!("{}", s3);

    let hello = String::from("नमस्ते");
    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    };
}
