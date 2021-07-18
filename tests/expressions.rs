use eretria::parse_string;

#[test]
fn assignment() {
    assert!(parse_string(&"fn main() a = 1").is_ok());
}

#[test]
fn invalid_assignment() {
    assert!(parse_string(&"fn main() 1 = 2").is_err());
}

#[test]
fn call() {
    assert!(parse_string(&"fn main() main()").is_ok());
}

#[test]

fn _return() {
    assert!(parse_string(&"fn main() return 100").is_ok());
}
