// The iterator pattern allows you to perform some task on a sequence of items in turn.
// An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.
// When you use iterators, you don’t have to reimplement that logic yourself.
// Iterators can be used on any data structure

// In Rust, iterators are lazy, meaning they have no effect until you call methods that
// consume the iterator to use it up.

// Also note that the values we get from the calls to next are immutable references to the values in the vector.
// The iter method produces an iterator over immutable references.
// If we want to create an iterator that takes ownership of v1 and returns owned values,
// we can call into_iter instead of iter.
// Similarly, if we want to iterate over mutable references, we can call iter_mut
// instead of iter.

// All iterators implement the Iterator trait from the std library
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

#[test]
fn iterator_demo() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4])
}
