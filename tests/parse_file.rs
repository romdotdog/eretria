use eretria::parse_file;

#[test]
fn parse_file_test() {
    assert!(parse_file("tests/parse_file.er").is_ok());
}
