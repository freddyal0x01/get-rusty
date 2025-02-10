fn main() {
    // Match Expressions

    enum Language {
        English,
        Spanish,
        Russian, 
        Japanese
    }

    let language = Language::English;

    match language {
        Language::English => println!("Hello!"),
        Language::Spanish => println!("Hola!"),
        Language::Russian => println!("привет"),
        Language::Japanese => println!("こんにちは"),
        _ => println!("Unsupported language!")
    }

    // Conditional if let expressions

    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: guest");
    }

    // while let Conditional loops

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loops

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let statements
    let x = 5;

    let (x, y, z) = (1, 2, 3);

    // function parameters
    
    let point = (3, 5);
    print_coordinates(&point);
}


fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}