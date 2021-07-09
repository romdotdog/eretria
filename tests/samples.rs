use eretria::parse_file;
use std::fs;

#[test]
fn test_samples() {
    let paths = fs::read_dir("./tests/samples").unwrap();

    for file in paths {
        let path = file.expect("could not open file").path();
        println!(
            "parsing {:?}",
            path.file_name().expect("no file name found")
        );
        parse_file(path).unwrap();
    }
}
