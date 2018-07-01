# resa
Rust API for error searching on particular platform like i.e StackOverflow. The mainpurpose is to decrease the amount of time taking research for certain compiler issues.

## Installation
Apply changes on your `Cargo.toml`
```` toml
[dependencies]
resa = "0.1.0"
````

## Binary
`cargo install --features=cli`

## API Example usage
```` rust
extern crate resa;
use resa::*;

fn main(){
    let mut s: StackOverflow = StackOverflow::search("Compiler Error")
                            .filter(3);

    for error in s.items{
        println!("(:?)", error);
    }
}
````

