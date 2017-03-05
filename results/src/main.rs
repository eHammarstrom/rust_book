/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

use std::fs::File;
use std::io::ErrorKind;

struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {
            value: value,
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("File could not be created: {:?}", e),
            }
        },
        Err(error) => panic!("File could not be read: {:?}", error),
    };

    let g = Guess::new(32);
    println!("Value of Guess g: {}", g.value());
}
