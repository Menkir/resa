# resa [![Build Status](https://travis-ci.org/Menkir/resa.svg?branch=master)](https://travis-ci.org/Menkir/resa)
Rust API for error searching on particular platform like i.e StackOverflow

<span style="color: red">!! not available on crates.io, release on 1st July 18 !!</span>

## Installation
Apply changes on your `Cargo.toml`
```` toml
[dependencies]
resa = "0.1.0"
````

## API Example usage
```` rust
extern crate resa;
use resa::*;

fn main(){
    let mut s: StackOverflow = StackOverflow::search("Compiler Error")
                            .filter(3);

    for issues in s.items{
        println!("(:?)", issues);
    }
}
````

