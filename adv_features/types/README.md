# Advanced Types

## Using the Newtype Pattern for Type Safety and Abstraction

The Trait rule: The orphan rule states that we’re only allowed to implement a
trait on a type if either the trait or the type are local to our crate.

We can get around this orphan rule by using the _newtype pattern_, which
involves creating a new type in a tuple struct. The tuple struct will have one
field and be a thin wrapper around the type we want to implement a trait for.
Then the wrapper type is local to our crate, and we can implement the trait on
the wrapper.

The newtype pattern is also useful for including statically enforcing that
values are never confused and indicating the units of a value.

We can also use the newtype pattern to abstract away some implementation details
of a type: the new type can expose a public API that is different from the API
of the private inner type.

## Creating Type Synonyms with Type Aliases

Rust provides the ability to declare a type alias to give an existing type
another name. For this we use the type keyword.

Example: We can create a type alias for `Kilometers` to `i32`. Values that have
the type `Kilometers` will be treated the same as values of type `i32`:

```rust
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
```

Because `Kilometers` and `i32` are the same type, we can add values of both
types and we can pass `Kilometers` values to functions that take `i32`
parameters.

The main use case for type synonyms is to reduce repetition. For example, we
might have a lengthy type like this:

```rust
Box<dyn Fn() + Send + 'static>
```

A Type alias can writing a shorter version of the same type more manageable by
reducing repetition and making it easier to read..

```rust
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
    }
```

Type aliases are also commonly used with the `Result<T, E>` type for reducing
repetition.

Example using the `std::io` module in the standard library.

```rust
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```

The `Result<..., Error>` is repeated a lot. As such, `std::io` has this type
alias declaration:

```rust
type Result<T> = std::result::Result<T, std::io::Error>;
```

## The Never Type that Never Returns

Rust has a special type named `!` that’s known in type theory lingo as the
_empty type_ because it has no values. We prefer to call it the _never type_
because it stands in the place of the return type when a function will never
return.

Example:

```rust
fn bar() -> ! {
    // --snip--
}
```

This code is read as “the function `bar` returns never.” Functions that return
never are called _diverging functions_.

The never type is useful when you're trying to continue a loop. The `continue`
keyword has a never type.

Keywords that have a never type:

- `continue` for a loop
- `loop`
- `panic!` macro

## Dynamically Sized Types and the Sized Trait

Rust needs to know certain details about its types, such as how much space to
allocate for a value of a particular type.

This leaves one corner of its type system a little confusing at first: the
concept of dynamically sized types. Sometimes referred to as DSTs or unsized
types, these types let us write code using values whose size we can know only at
runtime.

A DST is the `str` type

Example Code:

```rust
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

Rust needs to know how much memory to allocate for any value of a particular
type, and all values of a type must use the same amount of memory.

This code won't compile.

```rust
let s1: &str = "Hello there!";
let s2: &str = "How's it going?";
```

We can get the code to compile by using `&str` instead of `str`.

The string slice structure (`&str`) will store 2 values:

1. An address pointing to the location of the string in memory
2. The length of the string

Both the address and the length value of the string have a type of usize, we
know their size at compile time.

Every trait is a dynamically sized type we can refer to by using the name of the
trait.

To work with DSTs, Rust provides the `Sized` trait to determine whether or not a
type’s size is known at compile time. This trait is automatically implemented
for everything whose size is known at compile time. In addition, Rust implicitly
adds a bound on `Sized` to every generic function.

Generic Function:

```rust
fn generic<T>(t: T) {
    // --snip--
}
```

is treated like this:

```rust
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

By default, generic functions will work only on types that have a known size at
compile time.

If we don't use the size at compile time, we can use the `?Sized` trait bound to
allow any type that may or may not be of a size known at compile time.

```rust
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```
