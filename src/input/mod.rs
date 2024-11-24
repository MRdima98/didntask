#[cfg(test)]
mod tests {
    use didntask::cli_input::parse_input;

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
}
