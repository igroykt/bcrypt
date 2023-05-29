extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};
use std::{env};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Error: Provide arguments");
    } else if args[1] == "-verify" {
        let result = verify(args[2].to_string(), &args[3]);
        println!("{:?}", result);
    } else if args[1] == "-h" || args[1] == "--help" {
        help();
    } else {
        let result = hash(args[1].to_string(), DEFAULT_COST);
        println!("{:?}", result.unwrap().to_string());
    }
}

fn help() {
    println!("bcrypt v1.0
Author: igroykt (c)2023. Written on Rust.\n
Usage:
    bcrypt <string>
        generate hash
    bcrypt -verify <string> <hash>
        verify hash
    bcrypt -h
        print help
")
}