extern crate resa;
extern crate colored;
extern crate extractor;

use resa::Solutions;
use resa::stackoverflow::*;
use colored::*;
use std::process::Command;
use std::ffi::OsString;
use extractor::*;

extern crate clap;
use clap::App;

static DEFAULT_RESULTS: usize = 1;

#[derive(Debug)]
struct Entries {
    entries: Vec<Entry>,
}

#[derive(Debug)]
struct Entry {
    title: String,
    link: String,
}

pub fn main() {
    let matches = App::new("rofl")
        .version("1.0")
        .author("Kirill M. <kimeng@htwg-konstanz.de>")
        .about(
            "Wrapper for Compiler to analyze errors and search for solutions",
        )
        .args_from_usage(
            "<PATH>               'Sets the path file to use'
                              -f, --filter          'Filter Results by highest ranked answers'
                              -r, --results=[AMOUNT]'Sets amount of result which should displayed'
                              -v, --verbose         'Sets the level of verbosity'",
        )
        .get_matches();

    let query = matches.value_of("PATH").unwrap();
    let amount = match matches.value_of("results") {
        Some(a) => a.parse::<usize>().unwrap(),
        None => DEFAULT_RESULTS,
    };

    let mut compiler_query = OsString::from("rustc ");
    compiler_query.push(query);

    let command = Command::new("sh")
        .arg("-c")
        .arg(compiler_query)
        .output()
        .expect("failed to execute process");

    let result = String::from_utf8_lossy(&command.stderr);
    let search_queries = get_error_text_without_error_code(result.to_string());
    let mut so: Vec<StackOverflow> = Vec::with_capacity(search_queries.len());

    for q in search_queries {
        let mut temp = StackOverflow::search(&q);
        if temp.is_ok() {
            so.push(temp.unwrap()); //push found solutions
        } else {
            so.push(StackOverflow::new()); //push empty object
        }
    }

    let mut entries: Vec<Entries> = Vec::new();

    if matches.is_present("filter") {
        for mut s in so.clone().into_iter() {
            let mut temp = Entries { entries: Vec::new() };
            s.filter(amount).clone().items.into_iter().for_each(|item| {
                temp.entries.push(Entry {
                    title: item.title,
                    link: item.link,
                })
            });
            entries.push(temp);
        }
    } else {
        for mut s in so.clone().into_iter() {
            let mut temp = Entries { entries: Vec::new() };
            s.items.into_iter().take(amount).for_each(|item| {
                temp.entries.push(Entry {
                    title: item.title,
                    link: item.link,
                })
            });
            entries.push(temp);
        }
    }

    for e in structure_compiler_output(result.to_string()).iter().zip(
        entries.iter_mut(),
    )
    {
        let (err, sol) = e;
        println!("{}\n", err.replace("\\n", "\n"));
        if sol.entries.len() == 0 {
            println!(
                "{}\n ---EMPTY---\n",
                "There are no similar issues on StackOverflow:".red()
            );
        } else {
            println!(
                "{}",
                "There are some similar issues on StackOverflow:".red()
            );
            for p in sol.entries.iter() {
                println!("{:?}\n{}\n", p.title, p.link.trim_matches('"').yellow())
            }
        }

    }
}
