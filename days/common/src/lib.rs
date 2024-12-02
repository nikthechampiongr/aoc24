use std::io::Read;

pub fn read_input() -> String {
    let mut input = std::fs::File::open("input.txt").unwrap();
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();
    buf
}
