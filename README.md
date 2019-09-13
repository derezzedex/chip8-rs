# chip8-rs
CHIP-8 Emulator built in Rust.  
Window, input handling and graphics by [glium](https://github.com/glium/glium).

### Run
All ROMs are found (or can be included) at the same directory as the `Cargo.toml` inside the `roms` folder.
By default if no argument containing a file path is used, `roms/tests/test_01.ch8` will be loaded.
```
cargo run
cargo run tests/test_02.ch8
```

### ROMS
The repo includes a tests folder with two roms that are used in this [site](https://austinmorlan.com/posts/chip8_emulator/#results) to check the CPU status.  
`test_01.ch8` is `test_opcode.ch8`  
`test_02.ch8` is `BC_test.ch8`  

### Source
https://archive.org/stream/byte-magazine-1978-12/1978_12_BYTE_03-12_Life#page/n109/mode/2up  
http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1  
http://mattmik.com/files/chip8/mastering/chip8.html  
http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/  
https://austinmorlan.com/posts/chip8_emulator
