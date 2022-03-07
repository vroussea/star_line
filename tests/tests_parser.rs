#[cfg(test)]
mod tests_parser {
    use star_line::functions::parser;

    #[test]
    fn convert_simple_line() {
        let line = String::from("1001");
        assert_eq!(parser::into_vec(line).unwrap(), vec![true, false, false, true]);
    }

    #[test]
    fn line_too_short() {
        let line = String::from("");
        assert!(parser::into_vec(line).is_err());
    }

    #[test]
    fn line_with_wrong_characters() {
        let line = String::from("100a");
        assert!(parser::into_vec(line).is_err());
    }
}