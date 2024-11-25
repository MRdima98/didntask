use core::str;
use regex::Regex;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub fn parse_file<'a>(path: String, clean_data: &mut String) {
    let mut file = File::open(path.clone()).expect("Can't open the file.");
    let mut data = vec![];
    file.read_to_end(&mut data).expect("Can't read the file.");

    let data = str::from_utf8(&data).expect("Can't parse the chars");
    let re = Regex::new(r"\/\/.*").expect("Down bad regex.");
    let Some(cap) = re.captures(data) else {
        return;
    };
    let cap = cap.iter();

    if cap.clone().count() != 0 {
        print!("Here's the comments you wish to be removed!:\n\n")
    }

    for x in cap {
        let tmp = x.unwrap();
        print!("{}\n", tmp.as_str());
        *clean_data = data[0..tmp.start()].to_string() + &data[(tmp.end() + 1)..data.len()];
    }
}

pub fn write_to_file(path: String, clean_data: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

    file.write_all(clean_data.as_bytes())?;
    Ok(())
}
