use std::io;
use collections::hash::*;
use collections::vector::*;
use collections::string::*;

mod collections;

fn main() {
    let mut v = vector::init_vec();
    let mut hash = hash::init_hash();
    println!("Enter a word. To exit, type 'exit'.");
    println!("To see how many times a word was entered, type 'hash'.");
    loop {
        let mut word = string::init_string();
        io::stdin().read_line(&mut word).expect("what did you type?");
        let word: String = word.trim().parse().expect("Somoething went wrong, try again!");
        match &word[..] {
            "exit" => {
                println!("{}", string::vec_to_string(v.clone()));
                break
            },
            "hash" => {
                for(key, value) in &hash {
                    println!("{key}: {value}");
                }
                println!("Enter another word:");
            },
            _ => {
                let value: Option<&u32> = hash::get_hash(&hash, &word);
                let count;
                match value {
                    Some(v) => count = v + 1,
                    None => count = 1
                }
                hash::add_to_hash(&mut hash, &word, count);
                v.push(word);
                println!("Enter another word:");
            },
        }
    }
}
