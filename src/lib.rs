#![doc = include_str!("../README.md")]

pub use thiserror_string_context_macro::string_context;

pub trait AddErrorContext<T,E> {
    fn with_context<'a>(self, f: impl FnOnce()->&'a str) -> std::result::Result<T, E>;
}

#[cfg(test)]
mod tests {
    use thiserror::Error;
    use super::*;

    #[string_context("Custom context messag: {0}")]
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

    #[test]
    fn test1() -> anyhow::Result<()> {
        callme(42)?;
        callme(1).with_context(|| "Crashing with value 1")?;
        Ok(())
    }


}