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
    println!("query: {:?}", query);
    let solutions = StackOverflow::new()
    .search(query.as_str());
    let items = solutions.unwrap().items;
    if items.len() > 0{
        println!("{}", "There are some similar issues on StackOverflow:".red());
        for e in items.iter(){
        println!("{:?}", e.title);
        println!("{}\n", e.link.trim_matches('"').yellow());
        
        }
    } else{
        println!("{}", "There are no some similar issues on StackOverflow".red());
    }
    
}
