# Patterns and Matching

_Patterns_ are a special syntax in Rust for matching against the structure of
types, both complex and simple. Using patterns in conjunction with `match`
expressions and other constructs gives you more control over a program’s control
flow. A pattern consists of some combination of the following:

- Literals
- Destructured arrays, enums, structs, or tuples
- Variables
- Wildcards
- Placeholders

## All the Places Patterns Can Be Used

### `match` Arms

Formally, match expressions are defined as the keyword match, a value to match
on, and one or more match arms that consist of a pattern and an expression to
run if the value matches that arm’s pattern, like this:

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

### Conditional `if let` Expressions

`if let` expressions mainly as a shorter way to write the equivalent of a
`match` that only matches one case. Optionally, `if let` can have a
corresponding `else` containing code to run if the pattern in the `if let`
doesn’t match.

Example Code:

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

This conditional structure lets us support complex requirements.

You can see that `if let` can also introduce shadowed variables in the same way
that match arms can: the line `if let Ok(age) = age` introduces a new shadowed
`age` variable that contains the value inside the `Ok` variant.

The downside of using `if let` expressions is that the compiler doesn’t check
for exhaustiveness, whereas with `match` expressions it does. If we omitted the
last `else` block and therefore missed handling some cases, the compiler would
not alert us to the possible logic bug.

### `while let` Conditional Loops

Similar in construction to if let, the while let conditional loop allows a while
loop to run for as long as a pattern continues to match.

Example Code:

```rust
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
```

### `for` Loops

In a `for` loop, the value that directly follows the keyword `for` is a pattern.
For example, in `for x in y` the `x` is the pattern.

```rust
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
```

We adapt an iterator using the enumerate method so it produces a value and the
index for that value, placed into a tuple.

### `let` Statements

Every time you’ve used a let statement like this you’ve been using patterns,
although you might not have realized it! More formally, a let statement looks
like this:

```rust
let PATTERN = EXPRESSION;
```

To see the pattern matching aspect of let more clearly, consider Listing 18-4,
which uses a pattern with let to destructure a tuple.

```rust
let (x, y, z) = (1, 2, 3);
```

Here, we match a tuple against a pattern.

### Function Parameters

Function parameters can also be patterns.

The example code below declares a function named foo that takes one parameter
named x of type i32, should by now look familiar.

```rust
fn foo(x: i32) {
    // code goes here
}
```

The `x` part is a pattern! As we did with `let`, we could match a tuple in a
function’s arguments to the pattern.

## Refutability: Whether a Pattern Might Fail to Match

Patterns come in two forms: refutable and irrefutable. Patterns that will match
for any possible value passed are _irrefutable_. An example would be `x` in the
statement `let x = 5`; because `x` matches anything and therefore cannot fail to
match.

Patterns that can fail to match for some possible value are refutable. An
example would be Some(x) in the expression if let Some(x) = a_value because if
the value in the a_value variable is None rather than Some, the Some(x) pattern
will not match.

Function parameters, `let` statements, and `for` loops can only accept
irrefutable patterns, because the program cannot do anything meaningful when
values don’t match.

The `if let` and `while let` expressions accept refutable and irrefutable
patterns, but the compiler warns against irrefutable patterns because by
definition they’re intended to handle possible failure: the functionality of a
conditional is in its ability to perform differently depending on success or
failure.

Example Code:

```rust
let Some(x) = some_option_value;
```

This code won't compile because if `some_option_value` was a `None` value, it
would fail to match the pattern `Some(x)`, meaning the pattern is refutable.
However, the `let` statement can only accept an irrefutable pattern because
there is nothing valid the code can do with a `None` value.

Example code with a refutable pattern:

```rust
    if let Some(x) = some_option_value {
        println!("{x}");
    }
```

If we have a refutable pattern where an irrefutable pattern is needed, we can
fix it by changing the code that uses the pattern: instead of using `let`, we
can use `if let`. Then if the pattern doesn’t match, the code will just skip the
code in the curly brackets, giving it a way to continue validly.

However, if we give `if let` an irrefutable pattern (a pattern that will always
match), such as `x` the compiler will give us a warning.

Example code with a irrefutable pattern:

```rust
    if let x = 5 {
        println!("{x}");
    };
```

For this reason, match arms must use refutable patterns, except for the last
arm, which should match any remaining values with an irrefutable pattern. Rust
allows us to use an irrefutable pattern in a `match` with only one arm, but this
syntax isn’t particularly useful and could be replaced with a simpler `let`
statement.
