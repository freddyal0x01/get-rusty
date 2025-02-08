// closures are able to capture their environment. Note: they use extra memory

// They do this in 3 ways:
// 1. Taking ownership -> FnOnce.
// FnOnce takes ownership of the variables inside the closures env once
// 2. Borrowing mutably -> FnMut
// FnMut mutably borrows values
// 3. Borrowing immutably -> Fn
// Fn immutably borrows values

fn main() {
    let x = vec![1, 2, 3, 4];

    // we can force a closure to take ownership of a variable by
    // specifying the move keyword in front of a closure definition
    let equal_to_x = move |z| z == x;

    let y = vec![1, 2, 3, 4];

    assert!(equal_to_x(y));
}
