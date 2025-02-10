# Characteristics of OOP Languages

3 Characteristics of OOP:

1. Objects
2. Encapsulation
3. Inheritance

## Objects

Object-oriented programs are made up of objects. An object packages both data
and the procedures that operate on that data. The procedures are typically
called methods or operations.

Rust is object-oriented: structs and enums have data, and impl blocks provide
methods on structs and enums. Even though structs and enums with methods aren’t
called objects, they provide the same functionality.

## Encapsulation

Implementation details of an object aren’t accessible to code using that
object.The only way to interact with an object is through its public API; code
using the object shouldn’t be able to reach into the object’s internals and
change data or behavior directly. This enables the programmer to change and
refactor an object’s internals without needing to change the code that uses the
object.

We can use the `pub` keyword to decide which modules, types, functions, and
methods in our code should be public, and by default everything else is private.

Example: Average Collection Struct

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```

The struct is marked pub so that other code can use it, but the fields within
the struct remain private. This is important in this case because we want to
ensure that whenever a value is added or removed from the list, the average is
also updated.

```rust
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

The public methods `add`, `remove`, and `average` are the only ways to access or
modify data in an instance of `AveragedCollection`. When an item is added to
`list` using the `add` method or removed using the `remove` method, the
implementations of each call the private `update_average` method that handles
updating the `average` field as well.

We leave the `list` and `average` fields private so there is no way for external
code to add or remove items to or from the `list`.

Because we’ve encapsulated the implementation details of the struct
`AveragedCollection`, we can easily change aspects, such as the data structure,
in the future.

## Inheritance

Inheritance is a mechanism whereby an object can inherit elements from another
object’s definition, thus gaining the parent object’s data and behavior without
you having to define them again.

Rust doesn't have the ability to use inheritance in a object.
