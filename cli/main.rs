use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::io::Read;

#[derive(Serialize, Deserialize, Default, Debug)]
struct LogFile {
    bigrams: HashMap<(u32, u32), u32>,
    trigrams: HashMap<(u32, u32, u32), u32>,
    quadrams: HashMap<(u32, u32, u32, u32), u32>,
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let mut file = OpenOptions::new()
        .read(true) // Allow reading if the file exists
        .write(true) // Allow writing to either create or modify
        .create(true) // Create the file if it doesn't exist
        .open(path)
        .unwrap();
    let mut output = vec![];
    file.read_to_end(&mut output).unwrap();
    let load: LogFile = match bincode::deserialize(&output) {
        Ok(e) => e,
        Err(_) => LogFile::default(),
    };

    let mut meow: Vec<_> = load.bigrams.iter().collect();
    meow.sort_by_key(|k| k.1);
    meow.reverse();
    println!("most common bigrams");
    for i in 0..10 {
        println!("{:?}", meow[i]);
    }

    let mut meow: Vec<_> = load.trigrams.iter().collect();
    meow.sort_by_key(|k| k.1);
    meow.reverse();
    println!("most common trigrams");
    for i in 0..10 {
        println!("{:?}", meow[i]);
    }
    let mut meow: Vec<_> = load.quadrams.iter().collect();
    meow.sort_by_key(|k| k.1);
    meow.reverse();
    println!("most common quadrams");
    for i in 0..10 {
        println!("{:?}", meow[i]);
    }
}
