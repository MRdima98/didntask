use didntask::cli_input::{parse_input, parse_options};
use didntask::file_handling::{parse_file, write_to_file};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    parse_input(args.clone())?;

    let path = args.get(1).expect("Guess there is no file?").clone();
    let writing = parse_options(args);
    let mut clean_data = "".to_string();

    parse_file(path.clone(), &mut clean_data);

    if clean_data.len() != 0 && writing {
        write_to_file(path, clean_data)?;
        print!("You comments has been purge successffully!");
    }

    Ok(())
}
