use core::str;
use regex::Regex;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub fn parse_file<'a>(path: String, clean_data: &mut String) {
    let mut file = File::open(path.clone()).expect("Can't open the file.");
    let mut data = vec![];
    let mut indentation = 0;
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

    let chars = data.chars();
    for x in cap {
        let tmp = x.unwrap();
        print!("{}\n", tmp.as_str());
        indentation = tmp.start();
        loop {
            if indentation == 0 {
                break;
            }

            indentation -= 1;
            let char = chars.clone().nth(indentation).unwrap();
            //println!("What is this? ({})", chars.nth(indentation - 10).unwrap());
            println!("My tmp: {:?}", tmp);
            println!("Here is my char: {}", char);
            println!("Identation level: {}", indentation);
            if char != ' ' {
                break;
            }
        }
        *clean_data = data[0..indentation + 1].to_string() + &data[(tmp.end() + 1)..data.len()];
    }
}

pub fn write_to_file(path: String, clean_data: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

    file.write_all(clean_data.as_bytes())?;
    Ok(())
}
