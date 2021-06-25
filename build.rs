use std::{
    cmp::Ordering,
    env::var,
    fs::File,
    io::{BufWriter, Error, Write},
    path::Path,
};

struct BuildFile {
    handle: BufWriter<File>,
}

impl BuildFile {
    pub fn new(filename: &str) -> BuildFile {
        let path = Path::new(&var("OUT_DIR").expect("missing OUT_DIR env variable")).join(filename);
        BuildFile {
            handle: BufWriter::new(
                File::create(path).unwrap_or_else(|_| panic!("could not create file {}", filename)),
            ),
        }
    }

    pub fn done(mut self) {
        self.handle.flush().expect("could not flush file")
    }
}

const OPS: [(&str, u8); 16] = [
    ("|", 0),
    ("^", 1),
    ("&", 2),
    ("==", 3),
    ("!=", 3),
    (">=", 4),
    ("<=", 4),
    (">", 4),
    ("<", 4),
    (">>", 5),
    ("<<", 5),
    ("+", 6),
    ("-", 6),
    ("*", 7),
    ("/", 7),
    ("**", 8),
];

fn main() -> Result<(), Error> {
    let mut opfile = BuildFile::new("operators.rs");
    write!(
        opfile.handle,
        "{}",
        "pub fn precedence(op: &str) -> Option<u8> { match op {"
    )?;

    let mut iter = OPS.iter();
    let mut last_prec: &u8 = &u8::MAX;
    while let Some((op, prec)) = iter.next() {
        match prec.partial_cmp(last_prec).expect("unexpected NaN") {
            Ordering::Equal => write!(opfile.handle, " | \"{}\"", op)?,
            Ordering::Greater => write!(opfile.handle, " => Some({}), \"{}\"", last_prec, op)?,
            Ordering::Less => write!(opfile.handle, "\"{}\"", op)?,
        };
        last_prec = prec;
    }

    write!(
        opfile.handle,
        " => Some({}), _ => None, {}",
        last_prec, "} }"
    )?;

    opfile.done();

    Ok(())
}
