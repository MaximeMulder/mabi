use std::fs::File;
use std::io::Read;

pub fn read(path: &str) -> Box<[u16]> {
    let mut file = File::open(path).unwrap();
    let size = file.metadata().unwrap().len();
    let length = if size % 2 == 0 {
        (size / 2).try_into().unwrap()
    } else {
        panic!();
    };

    let mut instructions = vec![0u16; length].into_boxed_slice();
    let slice: &mut [u8] = cast(&mut instructions);
    file.read_exact(slice).unwrap();
    instructions
}

fn cast(slice: &mut [u16]) -> &mut [u8] {
    let length = 2 * slice.len();
    unsafe {
        std::slice::from_raw_parts_mut(
            slice.as_mut_ptr().cast::<u8>(),
            length
        )
    }
}
