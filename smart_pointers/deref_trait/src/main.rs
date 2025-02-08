// The Deref trait allows you to treat pointers like regular references.
// It also allows you to customize the behavior of the dereference operator *

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
