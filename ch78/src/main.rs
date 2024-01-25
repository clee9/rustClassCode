use std::io;

pub mod collections {
    pub mod vector {
        pub fn init_vec() -> Vec<String> {
            Vec::new()
        }
        
        pub fn p() {
            println!("3");
        }
    }

    pub mod string {
        pub fn init_string() -> String {
            String::new()
        }
    }

    pub mod hash {

    }
}

fn main() {
    let mut v = collections::vector::init_vec();

    
    loop {
        let mut word = collections::string::init_string();
        println!("Enter a word. To exit, type 'exit'");
        io::stdin().read_line(&mut word).expect("what did you type?");
        let word: String = word.trim().parse().expect("Somoething went wrong, try again!");
        match &word[..] {
            "exit" => break,
            "output" => todo!("print vec as string"),
            "hash" => todo!("hash"),
            _ => v.push(word),
        }
    }
}
