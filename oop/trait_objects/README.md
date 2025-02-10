# Using Trait Objects That Allow for Values of Different Types

We defined shared behavior using traits.

Trait objects allow for dynamic dispatch—enabling you to write code that can
operate on different types through a common interface defined by a trait. A
trait object points to both an instance of a type implementing our specified
trait and a table used to look up trait methods on that type at runtime. We
create a trait object by specifying some sort of pointer, such as a & reference
or a `Box<T>` smart pointer, then the `dyn` keyword, and then specifying the
relevant trait.

We can use trait objects in place of a generic or concrete type. Wherever we use
a trait object, Rust’s type system will ensure at compile time that any value
used in that context will implement the trait object’s trait. Consequently, we
don’t need to know all the possible types at compile time.

In Rust, we refrain from calling structs and enums “objects” to distinguish them
from other languages’ objects. In a struct or enum, the data in the struct
fields and the behavior in impl blocks are separated, whereas in other
languages, the data and behavior combined into one concept is often labeled an
object.

However, trait objects are more like objects in other languages in the sense
that they combine data and behavior. But trait objects differ from traditional
objects in that we can’t add data to a trait object. Trait objects aren’t as
generally useful as objects in other languages: their specific purpose is to
allow abstraction across common behavior.

Example Code: GUI application

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

```

This works differently from defining a struct that uses a generic type parameter
with trait bounds. A generic type parameter can only be substituted with one
concrete type at a time, whereas trait objects allow for multiple concrete types
to fill in for the trait object at runtime.

## Implementing the Trait

We can implement the trait such as the code below.

```rust
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

## Static vs Dynamic Dispatch

The Monomorphization process performed by the compiler when we use trait bounds
on generics: the compiler generates nongeneric implementations of functions and
methods for each concrete type that we use in place of a generic type parameter.
The code that results from monomorphization is doing static dispatch, which is
when the compiler knows what method you’re calling at compile time.

This is opposed to dynamic dispatch, which is when the compiler can’t tell at
compile time which method you’re calling. In dynamic dispatch cases, the
compiler emits code that at runtime will figure out which method to call. The
`dyn` keyword is used for running dynamic dispatches in a

When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t
know all the types that might be used with the code that’s using trait objects,
so it doesn’t know which method implemented on which type to call. Instead, at
runtime, Rust uses the pointers inside the trait object to know which method to
call. This lookup incurs a runtime cost that doesn’t occur with static dispatch.
Dynamic dispatch also prevents the compiler from choosing to inline a method’s
code, which in turn prevents some optimizations.

## Object Safety for Trait Objects

We can only make object safe traits into trait bounds.

When all of the methods implemented on that trait have 2 properites:

1. The return type is not itself
2. There are no generic parameters.
