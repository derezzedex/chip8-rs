/// **CHIP-8**
/// System memory
/// `0x000-0x1FF` Chip 8 interpreter (contains font set in emu)
/// `0x050-0x0A0` Used for the built in 4x5 pixel font set (0-F)
/// `0x200-0xFFF` Program ROM and work RAM
pub struct Chip8{
    opcode: u16, // 2-byte (16-bit)
    memory: [u8; 4096], // 1-byte (8-bit)
    v: [u8; 16], // Registers, from V0 to VF
    i: u16, // index register
    pc: u16, // program counter

    pixels: [u8; 64 * 32], // black or white

    delay_timer: u8, // Time registers (60HZ)
    sound_timer: u8, // When set above zero they'll count down to zero

    stack: [u8; 16], // 16 levels of stack
    sp: u8, // stack pointer

    key: [u8; 16] // HEX based keypad (0x0-0xF)
}

impl Chip8{
    pub fn new(){
    }

    pub fn initialize(&mut self){
    }

    pub fn emulate_cycle(&mut self){
    }
}
