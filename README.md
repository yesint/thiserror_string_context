# thiserror_string_context

This crate extends [thiserror](https://crates.io/crates/thiserror/) with possibility to add string context to error enums.
A typical usage is annotating `io::Error` with a file name, but it can be used in any case where a string annotation of the error is required.

## Relation to other similar crates
This crate is not as flexible in adding the context as [anyhow](https://crates.io/crates/anyhow) or [snafu](https://crates.io/crates/snafu), but instead it has a neglegible overhead and is much more ergonomic to use. It only serves a single purpose: adding an explanatory string to the error enum generated with `thiserror`. It doesn't work for any error types other than enums and doesn't support any other context types than strings.
In contrast to the other crate with similar purpose - [thiserror-context](https://crates.io/crates/thiserror-context), this crate does not obliges the user to create two distinct error types with and without the context. Instead the hidded context variant is added to the error enum itself, which makes this solution more elegant and much easier to maintain.

## Usage
The usage is very simple:
 ```rust
use thiserror::Error;
use thiserror_string_context::*;
// Annotate your error enum with `string_context` attribute.
// This will allow to use `MyError::with_context()` method
// to add a string annotation to your errors.
// You may add a custom error message where `{0}`
// is your original error variant.
#[string_context("Custom context message: {0}")]
#[derive(Error,Debug)]
enum MyError {
    #[error("Slight underflow happened!")]
    Underflow,
    #[error("slight overflow happened!")]
    Overflow,
    #[error("too far from correct!")]
    TooFar,
}
fn check_number(n: i32) -> Result<(),MyError> {
    match n {
        42 => println!("Correct number!"),
        41 => return Err(MyError::Underflow),
        43 => return Err(MyError::Overflow),
        _ => return Err(MyError::TooFar),
    }
    Ok(())
}
fn initiate_error() -> anyhow::Result<()> {
    // Here we add a context message
    check_number(41).with_context(|| "Crashing with value 41")?;
    Ok(())
}
```

This crashes with the following message:
```text
Custom context message: Crashing with value 41
Caused by:
    Slight underflow happened!
```

## Matching on error enums with context
When the context is added to the error enum a hidden variant is added to it, which makes matching on enum variants somewhat tedious. The method `unwrap_context` retuns a tuple where the first element is `Option<String>` containing the context (if there is any) and the second is the enum itself "peeled" from the context. This allows very simple matching:
```rust
if let Err(err) = initiate_error() {
    // Run different actions on different error variants
    match err.unwrap_context() {
        // Different actions could be performed on the same
        // variant with and without the context
        (Some(ctx),MyError::Underflow) => {...},
        (None,MyError::Underflow) => {...},
        // The context could be ignored
        (_,MyError::Overflow) => {...},
        // The wildcard pattern is required
        _ => {...},
    }
}
```

License: MIT OR Apache-2.0
