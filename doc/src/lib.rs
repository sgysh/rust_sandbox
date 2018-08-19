// Copyright (c) 2018 Yoshinori Sugino

//! # example
//!
//! ```
//! assert!(true);
//! ```
//!
//! ```should_panic
//! assert!(false);
//! ```

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
