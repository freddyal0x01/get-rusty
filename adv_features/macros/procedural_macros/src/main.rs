use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}

// Attribute-like macros

#[route(GET, "/")]
fn index() {
    //
}

#[proc_macro_attribute]
pub fn route(
    attr: TokenStream, // GET, "/" - Contents of the attribute
    item: TokenStream // fn index() { ... } - The body of the item that the attribute is attached to
) -> TokenStream {
    // ...
}

// Function-like macros

let sql = sql!(SELECT * FROM posts WHERE id=1);

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // ...
}