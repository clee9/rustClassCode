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

    println!("Enter a word. To exit, type 'exit'");
    let mut word = collections::string::init_string();
    while word != "exit" {
        io::stdin().read_line(&mut word).expect("what did you type?");
        v.push(word);

    }
}
