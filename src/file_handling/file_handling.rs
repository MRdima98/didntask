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

    let mut indentation = 0;
    let chars = data.chars();
    let mut newline_indicator = 0;
    let mut first_line_indicator = 1;
    for x in cap {
        let tmp = x.unwrap();
        print!("{}\n", tmp.as_str());
        indentation = tmp.start();
        loop {
            if indentation == 0 {
                first_line_indicator = 0;
                newline_indicator = 1;
                break;
            }

            indentation -= 1;
            let char = chars.clone().nth(indentation).unwrap();

            if char == '\n' {
                newline_indicator = 1;
            }

            if char != ' ' {
                break;
            }
        }
        *clean_data = data[0..indentation + first_line_indicator].to_string()
            + &data[(tmp.end() + newline_indicator)..data.len()];
    }
}

pub fn write_to_file(path: String, clean_data: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

    file.write_all(clean_data.as_bytes())?;
    Ok(())
}
