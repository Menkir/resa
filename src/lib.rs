#[macro_use]
extern crate serde_derive;

extern crate reqwest;
use reqwest::*;

pub trait Solutions<T, E>{
    fn search(&mut self, txt: &str)->Result<T>;
    fn filter(&mut self, amount_results: usize)-> &mut Self;
    fn preview(&self)->String;
}

pub mod stackoverflow;