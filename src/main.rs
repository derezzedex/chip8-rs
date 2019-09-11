use std::fs::File;
use std::io::Read;

mod cpu;

fn main() {
    let mut file = File::open("rom.ch8")
        .expect("Couldn't open file!");

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Couldn't read to buffer!");

    println!("Buffer size: {:?}", buffer.len());
    println!("Buffer: {:?}", buffer);
}
