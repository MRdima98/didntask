#[cfg(test)]
mod validation {
    use crate::cli_input::{parse_input, parse_options};

    #[test]
    fn should_err_less_that_two_args() {
        let args = vec!["gibberish".to_string()];
        let res = parse_input(args);
        match res {
            Ok(_) => panic!(),
            Err(_) => {}
        }
    }

    #[test]
    fn should_err_with_incorrect_path() {
        let args = vec!["gibberish".to_string(), "asdf".to_string()];
        let res = parse_input(args);
        match res {
            Ok(_) => panic!(),
            Err(_) => {}
        }
    }

    #[test]
    fn should_ok_simple_comment() {
        let args = vec![
            "gibberish".to_string(),
            "../testFiles/simpleComment".to_string(),
        ];
        let _ = parse_input(args);
    }

    #[test]
    fn should_be_true_with_write_option() {
        let args = vec![
            "gibberish".to_string(),
            "../testFiles/simpleComment".to_string(),
            "--write".to_string(),
        ];
        assert!(parse_options(args));
    }

    #[test]
    fn should_be_false_without_write_option() {
        let args = vec![
            "gibberish".to_string(),
            "../testFiles/simpleComment".to_string(),
        ];
        assert!(!parse_options(args));
    }
}
