// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


// Structs allow us to have multiple implementation blocks
// Methods in structs get passed &self whereas associated function don't
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {

    // Tuple structs are useful when you want you entire tuple to have a name
    // and be of different type than other tuples
    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    let rect3 = Rectangle::square(25);
    let rect4 = Rectangle::square(55);

    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));

    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
    println!("Can rect hold rect4? {}", rect.can_hold(&rect4));

    println!("rect is {:#?}", rect);

    println!("The area of the rectangle is {} square pixels.", rect.area());
}