fn main() {
    // ---- Ownership Rules ----
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // s is not valid here, it’s not yet declared
    {
        let s = String::from("hello"); // s is valid from this point forward
        println!("{}", s); // do stuff with s
    }
    // this scope is now over, and s is no longer valid

    let x = 5;
    let y = x; // Copy (copies the value of 5 into y) C
               // Rust has a special annotation called the Copy trait that we can place on
               // types like integers, bools, impl, & characters that are stored on the stack

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Move (not a shallow copy, s1 is no longer valid)

    println!("{}, world!", s1);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[..2];
}
