use didntask::file_handling::file_handling::{parse_file, write_to_file};
use didntask::input::cli_input::{parse_input, parse_options};
use std::env;
use std::error::Error;
use std::fs::read_dir;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    parse_input(args.clone())?;

    let path = args.get(1).expect("Guess there is no file?").clone();
    let mut directory: Vec<String> = vec![];
    let tmp = path.clone();
    let real_path = Path::new(&tmp);
    if real_path.is_dir() {
        for path in read_dir(real_path)? {
            let path = path?;
            let path = path.path();
            let real_path = Path::new(&path);
            if real_path.is_file() {
                directory.push(path.to_str().unwrap().to_string());
            }
        }
    } else {
        directory.push(path.clone());
    }
    parse_directory(directory, args);

    Ok(())
}

fn parse_directory(directory: Vec<String>, args: Vec<String>) {
    for path in directory {
        let writing = parse_options(args.clone());
        let mut clean_data = "".to_string();

        parse_file(path.clone(), &mut clean_data);

        if clean_data.len() != 0 && writing {
            write_to_file(path, clean_data)
                .expect("Looks like something happened during write, idk...");
            print!("You comments has been purge successffully!");
        }
    }
}
