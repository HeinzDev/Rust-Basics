use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("test.txt").expect("Unable to create the file");

    file.write_all(b"Test file being created").expect("Unable to write to the file"); // 'b' denotes a byteslice
    println!("File contents:\n\n{:?}", file);
}
