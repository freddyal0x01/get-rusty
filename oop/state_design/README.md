# Implementing an Object-Oriented Design Pattern

The _state pattern_ is an object-oriented design pattern. The crux of the
pattern is that we define a set of states a value can have internally. The
states are represented by a set of _state objects_, and the value’s behavior
changes based on its state.

The state objects share functionality: in Rust, of course, we use structs and
traits rather than objects and inheritance. Each state object is responsible for
its own behavior and for governing when it should change into another state. The
value that holds a state object knows nothing about the different behavior of
the states or when to transition between states.

The advantage of using the state pattern is that, when the business requirements
of the program change, we won’t need to change the code of the value holding the
state or the code that uses the value. We’ll only need to update the code inside
one of the state objects to change its rules or perhaps add more state objects.

## State Pattern Trade-offs

Downsides:

1. The states implement the transitions between states, some of the states are
   coupled to each other. If we add another state between PendingReview and
   Published, such as Scheduled, we would have to change the code in
   PendingReview to transition to Scheduled instead.
2. We've duplicated some logic. To eliminate some of the duplication, we might
   try to make default implementations for the request_review and approve
   methods on the State trait that return self; however, this would violate
   object safety, because the trait doesn’t know what the concrete self will be
   exactly. We want to be able to use State as a trait object, so we need its
   methods to be object safe.

## Encoding States and Behavior as Types

Rather than encapsulating the states and transitions completely so outside code
has no knowledge of them, we’ll encode the states into different types.
Consequently, Rust’s type checking system will prevent attempts to use draft
posts where only published posts are allowed by issuing a compiler error.

Example code related to a blog site with posts and draft posts:

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

Both the `Post` and `DraftPost` structs have a private `content` field that
stores the blog post text. The structs no longer have the `state` field because
we’re moving the encoding of the state to the types of the structs. The `Post`
struct will represent a published post, and it has a `content` method that
returns the `content`.

### Implementing Transitions as Transformations into Different Types

In the example code above, We want to enforce the rule that a draft post has to
be reviewed and approved before it can be published.

A post in the pending review state should still not display any content. Let’s
implement these constraints by adding another struct, `PendingReviewPost`,
defining the `request_review` method on `DraftPost` to return a
`PendingReviewPost`, and defining an `approve` method on `PendingReviewPost` to
return a `Post`.

In the library code above, the `request_review` and `approve` methods return new
instances rather than modifying the struct they’re called on, so we need to add
more `let post =` shadowing assignments to save the returned instances. We also
can’t have the assertions about the draft and pending review posts’ contents be
empty strings, nor do we need them: we can’t compile code that tries to use the
content of posts in those states any longer.
