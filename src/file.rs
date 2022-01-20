use std::fs;

pub fn read(path: &str) -> Box<[u8]> {
    fs::read(path).unwrap().into_boxed_slice()
}
