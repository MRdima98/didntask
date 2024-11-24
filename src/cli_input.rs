use core::fmt;
use std::error::Error;
use std::fs::exists;
use std::usize;

pub const NO_ARGS: usize = 2;

#[derive(Debug)]
enum InputError {
    NoFile,
}

impl Error for InputError {}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad input!")
    }
}

pub fn parse_input(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < NO_ARGS {
        println!(
            "\nYOU SHALL PASS AN ARG!\n\
                It shall be a directory or file?\n\
                Consider passing the whole current directory: didntask .\n\n\
                Here are the options:\n\
                \t --write => This option writes the modified lines to the file"
        );
        return Err(Box::new(InputError::NoFile));
    }

    if !exists(args[1].clone())? {
        println!(
            "\nWHAT GAMES ARE YOU PLAYING? THERE IS NO SUCH FILE!\n\
                Check out the path and try again!\n\n\
                Here are the options:\n\
                \t --write => This option writes the modified lines to the file"
        );
        return Err(Box::new(InputError::NoFile));
    };
    Ok(())
}
