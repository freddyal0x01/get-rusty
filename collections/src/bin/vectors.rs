fn main() {
    let a = [1, 2, 3];
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3];

    let third = &v[2];
    // println!("The third element is {}", third);s

    match v.get(3) {
        Some(fourth) => println!("The fourth element is {}", fourth),
        None => println!("There is no fourth element."),
    }


    let mut arr = vec![1, 2, 3, 4, 5];
    
    for i in &arr {
        println!("{}", i);
        println!("{}", &i);
    }
}
