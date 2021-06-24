pub fn precedence(op: &str) -> Option<u8> {
    match op {
        "|" => Some(0),
        "^" => Some(1),
        "&" => Some(2),
        "==" | "!=" => Some(3),
        ">" | "<" | ">=" | "<=" => Some(4),
        ">>" | "<<" => Some(5),
        "+" | "-" => Some(6),
        "*" | "/" => Some(7),
        "**" => Some(8),
        _ => None,
    }
}
