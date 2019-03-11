extern crate clap;

use clap::{Arg, App};

// use std::env;
use std::fs;

// use SliceConcatExt::join;

fn main() {
    

    let contents = fs::read_to_string(get_args())
        .expect("The file is bad");

    // println!("With text:\n{}", contents);
    let mut new_file: String = String::from("");
    for i in contents.split("\n") {
        
        new_file.push_str(&fix_line(i));
        new_file.push_str("\n");
    }
    println!("------------\n\n{}", new_file);

    
}

fn fix_line(line: &str) -> String {
    let _n = 30;
    let mut new_line: String = String::from(line);

    for (i, c) in line.chars().rev().enumerate() {
        if !(check_if_bad(&c) || c==' '){
            let index = line.len() - i;
            // println!("bad = {}", &index);
            new_line = [&new_line[..index], &new_line[index..]].join(&" ".repeat(_n-index));
            // println!("Line below");
            // println!("{}", line);
            // println!("{}", new_line);
            break;
        }
    }
    new_line
}

fn check_if_bad(c: &char) -> bool {
    let mut h = false;
    for i in [';', '{', '}', '.', '(', ')'].iter() {
        if i == c {
            h = true;
            break;
        }
    }
    h
}

fn get_args() -> String {
     let matches = App::new("pynize")
        .version("0.1.0")
        .author("Mark Solomonik <marik.solomonik@gmail.com>")
        .about("A formatter for python programmers using other languages")
        .arg(Arg::with_name("file")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("The file to format"))
        .get_matches();

    let file = matches.value_of("file").unwrap();

    println!("{} to be pythonized", file);

    file.to_string()
}
