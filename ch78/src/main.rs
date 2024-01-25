use std::io;

pub mod collections {
    pub mod vector {
        pub fn init_vec() -> Vec<String> {
            Vec::new()
        }
    }

    pub mod string {
        pub fn init_string() -> String {
            String::new()
        }

        pub fn vec_to_string(v: Vec<String>) -> String {
            let mut string = String::from("");
            for word in v {
                string.push_str(&word);
                string.push_str(" ");
            }
            string.to_string()
        }
    }

    pub mod hash {
        use std::collections::HashMap;

        pub fn init_hash() -> HashMap<String, u32> {
            HashMap::new()
        }
        
        pub fn get_hash<'a, 'b>(hash: &'a HashMap<String, u32>, word: &'b String) -> Option<&'a u32> {
            hash.get(word)
        }

        pub fn add_to_hash(hash: & mut HashMap<String, u32>, input: &String, count: u32) {
            hash.insert(input.clone(), count);
        }
    }
}

fn main() {
    let mut v = collections::vector::init_vec();
    let mut hash = collections::hash::init_hash();
    println!("Enter a word. To exit, type 'exit'.");
    println!("To see how many times a word was entered, type 'hash'.");
    loop {
        let mut word = collections::string::init_string();
        io::stdin().read_line(&mut word).expect("what did you type?");
        let word: String = word.trim().parse().expect("Somoething went wrong, try again!");
        match &word[..] {
            "exit" => {
                println!("{}", collections::string::vec_to_string(v.clone()));
                break
            },
            "hash" => {
                for(key, value) in &hash {
                    println!("{key}: {value}");
                }
                println!("Enter another word:");
            },
            _ => {
                let value: Option<&u32> = collections::hash::get_hash(&hash, &word);
                let count;
                match value {
                    Some(v) => count = v + 1,
                    None => count = 1
                }
                collections::hash::add_to_hash(&mut hash, &word, count);
                v.push(word);
                println!("Enter another word:");
            },
        }
    }
}
