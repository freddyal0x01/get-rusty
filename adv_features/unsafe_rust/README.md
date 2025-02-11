# Unsafe Rust

Unsafe rust exists for 2 reasons:

1. Static analysis is conservative. Rust will reject a valid program if it can't
   guarantee that the program is memory safe.
2. The underlying computer hardware is inherently unsafe. If Rust didn’t let you
   do unsafe operations, you couldn’t do certain tasks. Rust needs to allow you
   to do low-level systems programming, such as directly interacting with the
   operating system or even writing your own operating system.

To switch to `unsafe` Rust, use the unsafe keyword and then start a new block
that holds the unsafe code. You can take five actions in unsafe Rust that you
can’t in safe Rust, which we call _unsafe superpowers_.

Those superpowers include the ability to:

1. Dereference a raw pointer
2. Call an unsafe function or method
3. Access or modify a mutable static variable
4. Implement an unsafe trait
5. Access fields of a `union`

## Dereferencing a Raw Pointer

Unsafe rust has 2 types of raw pointers that are similar to references:

- `*const T`, which points to a value of type `T` in read-only memory. Immutable
  raw pointer
- `*mut T`, which points to a mutable value of type `T` in read-write memory.
  Mutable raw pointer

The asterisk isn’t the dereference operator; it’s part of the type name. In the
context of raw pointers, immutable means that the pointer can’t be directly
assigned to after being dereferenced.

Different from references and smart pointers, raw pointers:

- Are allowed to ignore the borrowing rules by having both immutable and mutable
  pointers or multiple mutable pointers to the same location
- Aren’t guaranteed to point to valid memory
- Are allowed to be null
- Don’t implement any automatic cleanup

We’ve created raw pointers by using `as` to cast an immutable and a mutable
reference into their corresponding raw pointer types.

Creating a pointer does no harm; it’s only when we try to access the value that
it points at that we might end up dealing with an invalid value.

## Calling an Unsafe Function or Method

Unsafe functions and methods look exactly like regular functions and methods,
but they have an extra `unsafe` before the rest of the definition.

The `unsafe` keyword in this context indicates the function has requirements we
need to uphold when we call this function, because Rust can’t guarantee we’ve
met these requirements. By calling an unsafe function within an `unsafe` block,
we’re saying that we’ve read this function’s documentation and take
responsibility for upholding the function’s contracts.

Example:

```rust
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

```

We must call the dangerous function within a separate unsafe block. If we try to
call dangerous without the unsafe block, we’ll get an error.

With the `unsafe` block, we’re asserting to Rust that we’ve read the function’s
documentation, we understand how to use it properly, and we’ve verified that
we’re fulfilling the contract of the function.

Bodies of unsafe functions are effectively `unsafe` blocks, so to perform other
unsafe operations within an unsafe function, we don’t need to add another
`unsafe` block.

## Creating a Safe Absctraction over Unsafe Code

Just because a function contains unsafe code doesn’t mean we need to mark the
entire function as unsafe. In fact, wrapping unsafe code in a safe function is a
common abstraction.

We can use an unsafe block inside of a safe function.

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

## Using `extern` Functions to call External Code

Sometimes, your Rust code might need to interact with code written in another
language. For this, Rust has the keyword `extern` that facilitates the creation
and use of a _Foreign Function Interface (FFI)_. An FFI is a way for a
programming language to define functions and enable a different (foreign)
programming language to call those functions.

Example Code: Setting up an integration with the `abs` function from C.

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

Within the `extern "C"` block, we list the names and signatures of external
functions from another language we want to call. The `"C"` part defines which
_application binary interface (ABI)_ the external function uses: the ABI defines
how to call the function at the assembly level. The `"C"` ABI is the most common
and follows the C programming language’s ABI.

This usage of `extern` does not require unsafe.

We can also use extern to create an interface that allows other languages to
call Rust functions.

Example Code: Creating a library for other languages to use.

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

We also need to add a `#[no_mangle]` annotation to tell the Rust compiler not to
mangle the name of this function. _Mangling_ is when a compiler changes the name
we’ve given a function to a different name that contains more information for
other parts of the compilation process to consume but is less human readable.

## Accessing or Modifying a Mutable Static Variable

In Rust, global variables are called _static_ variables.

Static variables have to use the `static` keyword and can be mutable or
immutable. They must have a static lifetime.

A subtle difference between constants and immutable static variables is that
values in a static variable have a fixed address in memory. Using the value will
always access the same data. Constants, on the other hand, are allowed to
duplicate their data whenever they’re used. Another difference is that static
variables can be mutable. Accessing and modifying mutable static variables is
_unsafe_.

## Implementing an Unsafe Trait

We can use `unsafe` to implement an unsafe trait. A trait is unsafe when at
least one of its methods has some invariant that the compiler can’t verify. We
declare that a trait is `unsafe` by adding the `unsafe` keyword before `trait`
and marking the implementation of the trait as `unsafe` too.

Example Code:

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

By using `unsafe impl`, we’re promising that we’ll uphold the invariants that
the compiler can’t verify.

## Accessing Fields of a Union

The final action that works only with `unsafe` is accessing fields of a _union_.
A union is similar to a `struct`, but only one declared field is used in a
particular instance at one time. Unions are primarily used to interface with
unions in C code. Accessing union fields is unsafe because Rust can’t guarantee
the type of the data currently being stored in the union instance.

## When to use Unsafe Code

Using `unsafe` to take one of the five actions (superpowers) just discussed
isn’t wrong or even frowned upon. But it is trickier to get `unsafe` code
correct because the compiler can’t help uphold memory safety. When you have a
reason to use `unsafe` code, you can do so, and having the explicit `unsafe`
annotation makes it easier to track down the source of problems when they occur.
