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

    #[test]
    fn after_line() {
        let path = "test_files/after_line.rs";
        let mut expected_data = "".to_string();
        parse_file(path.to_string(), &mut expected_data);
        let path = "test_files/after_line_sol.rs";
        let mut file = File::open(path).expect("Can't open the file.");
        let mut data = vec![];
        file.read_to_end(&mut data).expect("Can't read the file.");

        let data = str::from_utf8(&data).expect("Can't parse the chars");
        assert_eq!(data, expected_data)
    }

    #[test]
    fn after_line_without_space() {
        let path = "test_files/after_line_without_space.rs";
        let mut expected_data = "".to_string();
        parse_file(path.to_string(), &mut expected_data);
        let path = "test_files/after_line_without_space_sol.rs";
        let mut file = File::open(path).expect("Can't open the file.");
        let mut data = vec![];
        file.read_to_end(&mut data).expect("Can't read the file.");

        let data = str::from_utf8(&data).expect("Can't parse the chars");
        assert_eq!(data, expected_data)
    }

    #[test]
    fn first_char() {
        let path = "test_files/first_char.rs";
        let mut expected_data = "".to_string();
        parse_file(path.to_string(), &mut expected_data);
        let path = "test_files/first_char_sol.rs";
        let mut file = File::open(path).expect("Can't open the file.");
        let mut data = vec![];
        file.read_to_end(&mut data).expect("Can't read the file.");

        let data = str::from_utf8(&data).expect("Can't parse the chars");
        assert_eq!(data, expected_data)
    }

    #[test]
    fn first_char_file() {
        let path = "test_files/first_char_file.rs";
        let mut expected_data = "".to_string();
        parse_file(path.to_string(), &mut expected_data);
        let path = "test_files/first_char_file_sol.rs";
        let mut file = File::open(path).expect("Can't open the file.");
        let mut data = vec![];
        file.read_to_end(&mut data).expect("Can't read the file.");

        let data = str::from_utf8(&data).expect("Can't parse the chars");
        assert_eq!(data, expected_data)
    }

    #[test]
    fn simple_go() {
        let path = "test_files/simple.go";
        let mut expected_data = "".to_string();
        parse_file(path.to_string(), &mut expected_data);
        let path = "test_files/simple_sol.go";
        let mut file = File::open(path).expect("Can't open the file.");
        let mut data = vec![];
        file.read_to_end(&mut data).expect("Can't read the file.");

        let data = str::from_utf8(&data).expect("Can't parse the chars");
        assert_eq!(data, expected_data)
    }

    #[test]
    fn simple_sql() {
        let path = "test_files/simple.sql";
        let mut expected_data = "".to_string();
        parse_file(path.to_string(), &mut expected_data);
        let path = "test_files/simple_sol.sql";
        let mut file = File::open(path).expect("Can't open the file.");
        let mut data = vec![];
        file.read_to_end(&mut data).expect("Can't read the file.");

        let data = str::from_utf8(&data).expect("Can't parse the chars");
        assert_eq!(data, expected_data)
    }

    #[test]
    fn simple_py_ruby() {
        let path = "test_files/simple.rb";
        let mut expected_data = "".to_string();
        parse_file(path.to_string(), &mut expected_data);
        let path = "test_files/simple_sol.rb";
        let mut file = File::open(path).expect("Can't open the file.");
        let mut data = vec![];
        file.read_to_end(&mut data).expect("Can't read the file.");

        let data = str::from_utf8(&data).expect("Can't parse the chars");
        assert_eq!(data, expected_data)
    }

    #[test]
    fn rust_file() {
        let path = "test_files/simple_comment.rs";
        let expected = r"\/\/";
        let actual = check_extension(path).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn go_python_ruby_file() {
        let path = "test_files/simple.go";
        let expected = r"\/\/";
        let actual = check_extension(path).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn sql_file() {
        let path = "test_files/simple.sql";
        let expected = "--";
        let actual = check_extension(path).unwrap();
        assert_eq!(expected, actual);
    }
}
