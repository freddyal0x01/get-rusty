// A trait defines the functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way.
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.

//Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

use std::fmt::{format, Display};

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Default implementation
// We can provide default implementations for traits by adding method bodies to the trait definition.
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("@john_doe"),
        content: String::from("Hello, World!"),
        reply: false,
        retweet: false,
    }
}

// Trait bounds
// We can use trait bound as a parameter to a function.
// &impl Trait is syntax sugar for any type that implements the Summary trait.
// Used for simple cases
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bounds with no syntax sugar
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn dup_notify(item1: &impl Summary, item2: &impl Summary) {
    //
}

// Trait bounds with no syntax sugar, easier when you're dealing with multiple parameters
pub fn dup_notify1<T: Summary>(item1: &T, item2: &T) {
    //
}

fn some_function<T: Display + Clone, U: Clone + Summary>(t: &T, u: &U) -> i32 {
    0
}

// Using the where clause in traits makes it easier to read
fn some_function1<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Summary,
{
    0
}

fn main() {
    let tweet = Tweet {
        username: String::from("@john_doe"),
        content: String::from("Hello, World!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling!"),
    };

    // println!("Tweet summary: {}", tweet.summarize());
    // println!("Article summary: {}", article.summarize());

    println!("{}", returns_summarizable().summarize());
}
