use core::str;
use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::{exists, File, OpenOptions};
use std::io::{Read, Write};

const NO_ARGS: usize = 2;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("Input args: {:?}:", args);

    if args.len() < NO_ARGS {
        println!(
            "YOU SHALL PASS AN ARG!\n\
            It shall be a directory or file?\n\
            Consider passing the whole current directory: didntask .\n"
        );
        return Ok(());
    }

    if !exists(args[1].clone())? {
        println!(
            "WHAT GAMES ARE YOU PLAYING? THERE IS NO SUCH FILE!\n\
            Check out the path and try again!\n"
        );
        return Ok(());
    }

    println!("Input args: {:?}:", args);

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

    //let mut file = OpenOptions::new()
    //    .write(true)
    //    .truncate(true)
    //    .open("test/testFiles/simpleComment.rs")?;
    //
    //file.write_all(clean_data.as_bytes())?;

    print!("Starting data: {:?}\n", data);
    print!("Filtered data: {:?}\n", clean_data);

    Ok(())
}
