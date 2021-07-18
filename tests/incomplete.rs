use eretria::parse_string;
// test incomplete errors

// functions

#[test]
fn incomplete_function() {
    assert!(parse_string("fn main()").is_err());
}

#[test]
fn invalid_function_name() {
    assert!(parse_string("fn +").is_err());
}

#[test]
fn no_function_name() {
    assert!(parse_string("fn").is_err());
}

// blocks

#[test]
fn ends_eof() {
    assert!(parse_string("fn main() {").is_err());
}

#[test]
fn ends_next_function() {
    assert!(parse_string("fn main() {1 fn").is_err());
}
