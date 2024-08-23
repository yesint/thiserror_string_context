This crate extends [thiserror](https://crates.io/crates/thiserror/) with possibility to add string context to error enums. 

A typical usage is annotating `io::Error` with a file name, but it can be used in any case where a string annotation of the error is required.

# Relation to other similar crates
This crate is not as flexible in adding the context as [anyhow](https://crates.io/crates/anyhow) or [snafu](https://crates.io/crates/snafu), but instead it has a neglegible overhead and is much more ergonomic to use. It only serves a single purpose: adding an explanatory string to the error enum generated with `thiserror`. It doesn't work for any error types other than enums and doesn't support any other context types than strings.

In contrast to the other crate with similar purpose - [thiserror-context](https://crates.io/crates/thiserror-context), this crate does not obliges the user to create two distinct error types with and without the context. Instead the hidded context variant is added to the error enum itself, which makes this solution more elegant and much easier to maintain.

# Usage
The usage is very simple:
 ```ignore
use thiserror::Error;
use thiserror_string_context::*;
// Annotate your error enum with `string_context` attribute.
// This will allow to use `MyError::with_context()` method
// to add a string annotation to your errors.
// You may add a custom error message where `{0}` 
// is your original error variant.
#[thiserror_context("Custom context messag: {0}")]
#[derive(Error,Debug)]
enum MyError {
    #[error("Error 1")]
    Error1,
    #[error("Error 2")]
    Error2,
    #[error("Error 3")]
    Error3,
}
 
fn callme(n: i32) -> Result<(),MyError> {
    match n {
        42 => println!("Nice number!"),
        1 => return Err(MyError::Error1),
        2 => return Err(MyError::Error2),
        _ => return Err(MyError::Error3),
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    callme(42)?;
    // Here we add a context message
    callme(1).with_context(|| "Crashing with value 1")?;
    Ok(())
}
```
This crashes with the following message:
```text
Nice number!
Error: Custom context message: Crashing with value 1

Caused by:
   Error 1
```