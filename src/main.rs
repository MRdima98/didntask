use cli_input::parse_input;
use core::{fmt, str};
use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut writing = false;

    parse_input()?;

    let path = args.get(1).expect("Guess there is no file?").clone();

    for el in args.iter() {
        if el == "--write" {
            writing = true;
        }
    }

    let mut file = File::open(path.clone())?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;

    let data = str::from_utf8(&data)?;
    let re = Regex::new(r"\/\/.*")?;
    let Some(cap) = re.captures(data) else {
        return Ok(());
    };
    let mut clean_data = "".to_string();
    let cap = cap.iter();

    if cap.clone().count() != 0 {
        print!("Here's the comments you wish to be removed!:\n\n")
    }

    for x in cap {
        let tmp = x.unwrap();
        print!("{}\n", tmp.as_str());
        clean_data = data[0..tmp.start()].to_string() + &data[(tmp.end() + 1)..data.len()];
    }

    if writing {
        write_to_file(path, clean_data)?;
        print!("You comments has been purge succesffully!");
    }

    Ok(())
}

fn write_to_file(path: String, clean_data: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

    file.write_all(clean_data.as_bytes())?;
    Ok(())
}

mod cli_input {
    use std::env;
    use std::error::Error;
    use std::fs::exists;

    use crate::InputError;
    pub const NO_ARGS: usize = 2;

    pub fn parse_input() -> Result<(), Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();

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
            return Ok(());
        };
        Ok(())
    }
}
