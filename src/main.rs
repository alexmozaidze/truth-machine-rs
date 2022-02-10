#![feature(let_else)] // Oh boi, do I love experimental features!

use std::io::{self, prelude::*};

fn main() {
    print!("Input 1 or 0: ");
    io::stdout().flush().unwrap();

    // User could hit Ctrl-D in order to quit the program, in which case it would return `None`
    let Some(input) = io::stdin().lock().lines().next() else {
        println!();
        return;
    };

    match input.unwrap().trim() {
        "0" => println!("0"),
        "1" => loop {
            print!("1");
        },
        "" => eprintln!("Error: You didn't input anything"),
        _ => eprintln!("Error: Only 1 and 0 are allowed"),
    }
}
