use eretria::parse_string;

#[test]
fn ident() {
    assert!(parse_string(&"fn main() hi").is_ok());
}

#[test]
fn number() {
    assert!(parse_string(&"fn main() 23928392").is_ok());
}

#[test]
fn float() {
    assert!(parse_string(&"fn main() 100.2").is_ok());
}

#[test]
fn negative_number() {
    assert!(parse_string(&"fn main() -221111").is_ok());
}

#[test]
fn negative_float() {
    assert!(parse_string(&"fn main() -202.5").is_ok());
}

#[test]
fn obscure_decimal_number() {
    assert!(parse_string(&"fn main() 0d1234567890").is_ok());
}

#[test]
fn binary_number() {
    assert!(parse_string(&"fn main() 0b10101001").is_ok());
}

#[test]
fn invalid_binary() {
    assert!(parse_string(&"fn main() 0b1221").is_err());
}

#[test]
fn parentheses() {
    assert!(parse_string(&"fn main() (-100)").is_ok());
}
