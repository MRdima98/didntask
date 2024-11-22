use core::str;
use regex::Regex;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("test/testFiles/simpleComment.rs")?;

    let mut data = vec![];
    file.read_to_end(&mut data)?;
    let data = str::from_utf8(&data)?;

    let re = Regex::new(r"\/\/.*")?;
    let cap = re.captures(data).unwrap();
    let mut clean_data = "".to_string();
    for x in cap.iter() {
        let tmp = x.unwrap();
        print!("Match {}\n", tmp.as_str());
        print!("Starting index {}\n", tmp.start());
        print!("Ening index {}\n", tmp.end());
        clean_data = data[0..tmp.start()].to_string() + &data[tmp.end()..data.len()];
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("test/testFiles/simpleComment.rs")?;

    file.write_all(clean_data.as_bytes())?;

    print!("Starting data: {:?}\n", data);
    print!("Filtered data: {:?}\n", clean_data);

    Ok(())
}
