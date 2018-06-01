extern crate resa;
extern crate colored;

use resa::Solutions;
use resa::stackoverflow::*;
use std::env;
use colored::*;


pub fn main() {
    let mut args: Vec<_> = env::args().collect();
    args.remove(0);

    let mut query = String::new();
    for arg in args{
        query.push_str(arg.as_str());
        query.push(' ');
    }
    
    println!("{}", "There are some similar issues on StackOverflow:".red());
    StackOverflow::new()
    .search(query.as_str())
    .unwrap()
    .filter(3)
    .items
    .iter()
    .for_each(|entry| println!("{:?}\n{}\n", entry.title, entry.link.trim_matches('"').yellow()));
}
