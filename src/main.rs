use std::env;
use std::io::{self, BufRead};

use atty::Stream;

fn load_stdin() -> io::Result<String> {
    if atty::is(Stream::Stdin) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "stdin is not redirected",
        ));
    }

    let input = io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| {
            acc + &line.expect("failed to read line from pipe") + "\n"
        });

    Ok(input)
}

fn main() {
    env_logger::init();

    let mut args = env::args();
    args.next().unwrap(); // Ignore own name.

    let input = match args.next() {
        Some(input) => {
            log::info!("Input from argument:\n'{input}'");
            input
        },
        None => match load_stdin() {
            Ok(input) => {
                log::info!("Input from stdin pipe:\n'{input}'");
                input
            },
            Err(e) => {
                log::warn!("Aborting because of pipe error: {e}");
                std::process::exit(1);
            },
        },
    };

    let result = nandu::translate(input);
    match result {
        Ok(translation) => println!("{translation}"),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        },
    };
}
