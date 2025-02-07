fn main() {

}

// function with lifetime annotations
fn first_word1<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// function without lifetime annotations
// This works because rust is able to infer the lifetime annotations using the 3 elision rules

// 3 elision rules

// 1. Each parameter that is a reference gets its own lifetime parameter

// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters

// 3. If there are multiple input lifetime parameters, but one of them is 
//    &self or &mut self because this is a method, the lifetime of self is assigned to all output 
//    lifetime parameters. Only applies to methods

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}