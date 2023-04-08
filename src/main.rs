use std::env;
use std::io::{self, BufRead};

fn main() {
    let mut args = env::args();
    args.next().unwrap(); // Ignore own name.

    let input = match args.next() {
        Some(input) => input,
        None => {
            let input =
                io::stdin()
                    .lock()
                    .lines()
                    .fold("".to_string(), |acc, line| {
                        acc + &line.expect("Failed to read line from pipe")
                            + "\n"
                    });
            input
        },
    };

    let translation = nandu::translate(input).unwrap();
    println!("{translation}");
}