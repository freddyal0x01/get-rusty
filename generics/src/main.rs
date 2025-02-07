enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let integer = Option::Some(5);
    let float = Option::Some(5.0);
}
