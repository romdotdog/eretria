use eretria::parse_string;

#[test]
fn statements_no_semi() {
    // without semicolons
    assert!(parse_string(
        &r#"
	data[0] = "hi" 
	fn main() {2} 
	fn main2() 1 + 1
	"#,
    )
    .is_ok());
}

#[test]
fn statements_semi() {
    // with semicolons
    assert!(parse_string(
        r#"
	;;;data[0] = "hi";;;;;  ;;
	;;fn main() {2}; ;;
	;;;fn main2() 1 + 1;;
	"#,
    )
    .is_ok());
}

#[test]
fn unexpected_token() {
    assert!(parse_string("ssdjfhksjdggr").is_err());
}

#[test]
fn invalid_data_syntax() {
    assert!(parse_string("data[word] = \"hi\"").is_err());
}

#[test]
fn data_decimal_escape() {
    assert!(parse_string(r#"data[0] = "\d255\d128""#).is_ok());
}

#[test]
fn data_hex_escape() {
    assert!(parse_string(r#"data[0] = "\xFF\xDD""#).is_ok());
}
