//! # General
//! 
//! A rust written REST search API for error searching. 
//! 
//! ## Example Usage
//! An example usage of this crate by using the StackOverflow REST API
//! ```rust
//! let result = resa::stackoverflow::StackOverflow::search("Compiler Error");
//! ```
#[macro_use]
extern crate serde_derive;

extern crate reqwest;
use reqwest::*;

pub trait Solutions<T, E>{
    /// Apply search on  
    /// 
    /// # Arguments
    /// 
    /// * `txt` - Query for seaching errors. Usually the error message.
    /// 
    /// # Return
    /// 
    /// * Return a generic Type 
    fn search(txt: &str)->Result<T>;
    /// Apply filter on found Result
    ///
    /// # Arguments 
    /// 
    /// * `self` - to apply filter on own StackOverflow struct
    /// 
    /// * `amount_results` - restrict shown results to paritular number. Default is 1.
    fn filter(&mut self, amount_results: usize)-> &mut Self;
}

pub mod stackoverflow;