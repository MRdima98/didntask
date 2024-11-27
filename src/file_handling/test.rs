#[cfg(test)]
mod validation {
    use core::str;
    use std::{fs::File, io::Read};

    use crate::file_handling::file_handling::*;

    #[test]
    fn simple_comment() {
        let path = "test_files/simple_comment.rs";
        let mut expected_data = "".to_string();
        parse_file(path.to_string(), &mut expected_data);
        let path = "test_files/simple_comment_sol.rs";
        let mut file = File::open(path).expect("Can't open the file.");
        let mut data = vec![];
        file.read_to_end(&mut data).expect("Can't read the file.");

        let data = str::from_utf8(&data).expect("Can't parse the chars");
        assert_eq!(data, expected_data)
    }

    #[test]
    fn two_level_ident() {
        let path = "test_files/two_ident.rs";
        let mut expected_data = "".to_string();
        parse_file(path.to_string(), &mut expected_data);
        let path = "test_files/two_ident_sol.rs";
        let mut file = File::open(path).expect("Can't open the file.");
        let mut data = vec![];
        file.read_to_end(&mut data).expect("Can't read the file.");

        let data = str::from_utf8(&data).expect("Can't parse the chars");
        assert_eq!(data, expected_data)
    }
}
