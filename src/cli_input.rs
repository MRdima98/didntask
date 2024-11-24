use core::fmt;
use std::error::Error;
use std::fs::exists;
use std::usize;

pub const NO_ARGS: usize = 2;

enum InputError {
    NoArgs,
    NoFile,
}

impl Error for InputError {}

impl fmt::Debug for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::NoArgs => {
                write!(
                    f,
                    "\n\tYOU SHALL PASS AN ARG!\n\
                \tIt shall be a directory or file?\n\
                \tConsider passing the whole current directory: didntask .\n\n\
                \tHere are the options:\n\
                \t\t --write => This option writes the modified lines to the file"
                )
            }
            InputError::NoFile => {
                write!(
                    f,
                    "\n\tWHAT GAMES ARE YOU PLAYING? THERE IS NO SUCH FILE!\n\
                    \tCheck out the path and try again!\n\n\
                    \tHere are the options:\n\
                    \t\t --write => This option writes the modified lines to the file"
                )
            }
        }
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad input!")
    }
}

pub fn parse_input(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < NO_ARGS {
        return Err(Box::new(InputError::NoArgs));
    }

    if !exists(args[1].clone())? {
        return Err(Box::new(InputError::NoFile));
    };
    Ok(())
}
