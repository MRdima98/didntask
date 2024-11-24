use core::str;
use didntask::cli_input::parse_input;
use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub mod input;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut writing = false;

    parse_input(args.clone())?;

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
