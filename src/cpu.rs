const SCREEN_WIDTH: u16 = 64;
const SCREEN_HEIGHT: u16 = 32;
const SCREEN_SIZE: u16 = SCREEN_WIDTH * SCREEN_HEIGHT;

const MEMORY_SIZE: u16 = 4096;

const FONT_NUMBER: usize = 80;
const FONTSET: [u8; FONT_NUMBER] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

/// **CHIP-8**
/// System memory
/// `0x000-0x1FF` Chip 8 interpreter (contains font set in emu)
/// `0x050-0x0A0` Used for the built in 4x5 pixel font set (0-F)
/// `0x200-0xFFF` Program ROM and work RAM
pub struct Chip8{
    opcode: u16, // 2-byte (16-bit)
    memory: [u8; MEMORY_SIZE as usize], // 1-byte (8-bit)
    v: [u8; 16], // Registers, from V0 to VF
    i: u16, // index register
    pc: u16, // program counter

    display: [u8; SCREEN_SIZE as usize], // black or white

    delay_timer: u8, // Time registers (60HZ)
    sound_timer: u8, // When set above zero they'll count down to zero

    stack: [u8; 16], // 16 levels of stack
    sp: u8, // stack pointer

    key: [u8; 16] // HEX based keypad (0x0-0xF)
}

impl Chip8{
    pub fn new() -> Self{
        Self{
            opcode: 0,
            memory: [0; MEMORY_SIZE as usize],
            v: [0; 16],
            i: 0,
            pc: 0,

            display: [0; SCREEN_SIZE as usize],

            delay_timer: 0,
            sound_timer: 0,

            stack: [0; 16],
            sp: 0,

            key: [0; 16],
        }
    }

    /// Normally starts at 0x200
    pub fn initialize(&mut self, start: u16){
        //initializing default values
        self.pc = start;
        self.opcode = 0;
        self.i = 0;
        self.sp = 0;
        self.display = [0; SCREEN_SIZE as usize];
        self.stack = [0; 16];
        self.v = [0; 16];
        self.memory = [0; MEMORY_SIZE as usize];

        //loading font set
        for i in 0..FONT_NUMBER{
            self.memory[i] = FONTSET[i];
        }

        //reset timers
        self.delay_timer = 0;
        self.sound_timer = 0;
    }

    pub fn load_program(&mut self, buffer: Vec<u8>){
        // if program is bigger than memory, display error
        let program_size = buffer.len();
        if program_size >= (MEMORY_SIZE - 512) as usize{
            panic!("Program size is bigger than current memory!");
        }

        for i in 0..program_size{
            self.memory[i + 512] = buffer[i];
        }
    }

    /// Fetches one opcode from the memory at the location specified by the PC (program counter)
    /// Since opcodes are 2-bytes long, and our memory uses a one-byte array,
    /// we join the two bytes from memory to get the actual opcode.
    /// To do this, we'll use the Bitwise Shift and the Bitwise OR operator.
    /// Example:
    /// Fetch bytes to join in one opcode
    ///```
    /// memory[pc]     = 0xA2
    /// memory[pc + 1] = 0xF0
    ///```
    /// Then we'll bitshift the first one make 'space' for the second
    /// This shift 'simply adds zeros'
    ///```
    ///0xA2       0xA2 << 8 = 0xA200   HEX
    ///10100010   1010001000000000     BIN
    ///```
    /// And in the end, we join them with a Bitwise OR
    ///```
    /// 1010001000000000 | // 0xA200
    ///         11110000 = // 0xF0 (0x00F0)
    ///------------------
    ///1010001011110000   // 0xA2F0
    ///```
    pub fn fetch_opcode(&mut self) -> u16{
        let pc = self.pc as usize;
        (self.memory[pc] as u16) << 8 | (self.memory[pc + 1] as u16)
    }

    pub fn emulate_cycle(&mut self){
        self.fetch_opcode();

        // decode code
        // execute code

        //update timers
    }
}
