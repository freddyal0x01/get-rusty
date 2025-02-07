// We use generics to create definitions for items like function signatures or structs,
// which we can then use with many different concrete data types.

fn main() {
    let num_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(num_list);

    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest_char = get_largest(char_list);

    println!("The largest number is {}", largest_char);
}

fn get_largest<T: PartialOrd + Copy>(num_list: Vec<T>) -> T {
    let mut largest = num_list[0];

    for num in num_list {
        if num > largest {
            largest = num;
        }
    }
    largest
}
