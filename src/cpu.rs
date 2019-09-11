use rand::Rng;

const SCREEN_WIDTH: u16 = 64;
const SCREEN_HEIGHT: u16 = 32;
const SCREEN_SIZE: u16 = SCREEN_WIDTH * SCREEN_HEIGHT;

const MEMORY_SIZE: u16 = 4096;

const FONT_NUMBER: usize = 80;
/// Each font number is 5 bytes long.
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
    pub fn decode_opcode(&mut self) -> u16{
        let pc = self.pc as usize;
        (self.memory[pc] as u16) << 8 | (self.memory[pc + 1] as u16)
    }

    /// When executing a opcode, there are common parts used to store the metadata
    /// like the register that'll be used or the bytes to be added, so we
    /// separate the opcode in parts, they being:
    ///```
    /// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
    /// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
    /// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
    /// y - A 4-bit value, the upper 4 bits of the low byte of the instruction
    /// kk or byte - An 8-bit value, the lowest 8 bits of the instruction
    ///```
    pub fn execute_opcode(&mut self){

        let nnn = (self.opcode & 0xFFF0) >> 4;
        let n =   (self.opcode & 0x000F) as u8;
        let x =   (self.opcode & 0x0F00) >> 8;
        let y =   (self.opcode & 0x00F0) >> 4;
        let kk =  (self.opcode & 0x00FF) as u8;

        match self.opcode{
            0x00E0 =>{
                self.display = [0; SCREEN_SIZE];
            },
            0x00EE =>{ // []
                self.stack -= 1;
                self.pc = self.stack[self.sp];
            },
            0x1000..=0x1FFF =>{ // []
                self.pc = nnn;
                self.pc -= 2;
            },
            0x2000..=0x2FFF =>{ // []
                self.stack[self.sp] = self.pc;
                self.stack += 1;
                self.pc = nnn;
                self.pc -= 2;
            },
            0x3000..=0x3FFF => { // [SE Vx, byte] Skip next instruction if Vx = kk.
                if self.v[x as usize] == kk { self.pc += 2 }
            },
            0x4000..=0x4FFF =>{ // [SNE Vx, byte] Skip next instruction if Vx != kk.
                if self.v[x as usize] != kk { self.pc += 2 }
            },
            0x5000..=0x5FFF =>{ // [SE Vx, Vy] Skip next instruction if Vx = Vy.
                if self.v[x as usize] == self.v[y as usize] { self.pc += 2 }
            },
            0x6000..=0x6FFF =>{ // [LD Vx, byte] Set Vx = kk.
                self.v[x as usize] = kk;
            },
            0x7000..=0x7FFF =>{ // [ADD Vx, byte] Set Vx = Vx + kk.
                self.v[x as usize] += kk;
            },
            0x8000..=0x8FF0 =>{ // [LD Vx, Vy] Set Vx = Vy.
                self.v[x as usize] = self.v[y as usize];
            },
            0x8000..=0x8FF1 =>{ // [OR Vx, Vy] Set Vx = Vx OR Vy.
                self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
            },
            0x8000..=0x8FF2 =>{ // [AND Vx, Vy] Set Vx = Vx AND Vy.
                self.v[x as usize] = self.v[x as usize] & self.v[y as usize];
            },
            0x8000..=0x8FF3 =>{ // [XOR Vx, Vy] Set Vx = Vx XOR Vy.
                self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];
            },
            0x8000..=0x8FF4 =>{ // [ADD Vx, Vy] Set Vx = Vx + Vy, set VF = carry.
                let mut value = self.v[x as usize] as u16 + self.v[y as usize] as u16;
                if value > 255 {
                    value = 255;
                    self.v[0xF] = 1; // set the carry
                }else{
                    self.v[0xF] = 0;
                }

                self.v[x as usize] = value as u8;
            },
            0x8000..=0x8FF5 =>{ // [SUB Vx, Vy] Set Vx = Vx - Vy, set VF = NOT borrow.
                if self.v[x as usize] > self.v[y as usize] { self.v[0xF] = 1 } else { self.v[0xF] = 0 }
                self.v[x as usize] -= self.v[y as usize];
            },
            0x8000..=0x8FF6 =>{ // [SHR Vx {, Vy}] Set Vx = Vx SHR 1.
                if n == 0x1 { self.v[0xF] = 1 } else { self.v[0xF] = 0}
                self.v[x as usize] /= 2;
            },
            0x8000..=0x8FF7 =>{ // [SUBN Vx, Vy] Set Vx = Vy - Vx, set VF = NOT borrow.
                if self.v[y as usize] > self.v[x as usize] { self.v[0xF] = 1 } else { self.v[0xF] = 0 }
                self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
            },
            0x8000..=0x8FFE =>{ // [SHL Vx {, Vy}] Set Vx = Vx SHL 1.
                let most = (self.v[x as usize] & 0xF0) >> 4;
                if most == 1 { self.v[0xF] = 1 } else { self.v[0xF] = 0 }
                self.v[x as usize] *= 2;
            },
            0x9000..=0x9FF0 =>{ // [SNE Vx, Vy] Skip next instruction if Vx != Vy.
                if self.v[x as usize] != self.v[y as usize] { self.pc += 2 }
            },
            0xA000..=0xAFFF =>{ // [LD I, addr] Set I = nnn.
                self.i = nnn;
            },
            0xB000..=0xBFFF =>{ // [JP V0, addr] Jump to location nnn + V0.
                self.pc = nnn + self.v[0x0] as u16;
            },
            0xC000..=0xCFFF =>{ // [RND Vx, byte] Set Vx = random byte AND kk.
                let random = rand::thread_rng().gen::<u8>();
                self.v[x as usize] = kk & random;
            },
            0xD000..=0xDFFF => panic!("Drawing not implemented!"),
            0xE09E..=0xEF9E =>{ // [SKP Vx] Skip next instruction if key with the value of Vx is pressed.
                let input = 0u8; // TODO: Fetch keydown
                if input == self.v[x as usize] { self.pc += 2 }
            },
            0xE0A1..=0xEFA1 =>{
                let input = 0u8; // TODO: Fetch keydown
                if input != self.v[x as usize] { self.pc += 2 }
            },
            0xF007..=0xFF07 =>{ // []
                self.v[x as usize] = self.delay_timer;
            },
            0xF00A..=0xFF0A =>{ // []
                let input = 0u8; // TODO: wait for key press
                self.v[x as usize] = input;
            },
            0xF015..=0xFF15 =>{ // []
                self.delay_timer = self.v[x as usize];
            },
            0xF018..=0xFF18 =>{ // []
                self.sound_timer = self.v[x as usize];
            },
            0xF01E..=0xFF1E =>{ // []
                self.i += self.v[x as usize] as u16;
            },
            0xF029..=0xFF29 =>{ // []
                self.i = (self.v[x as usize] as u16) * 5; //sprites are 5-byte long
            },
            0xF033..=0xFF33 =>{ // []
                self.memory[self.i + 0] = self.v[x as usize] / 100;
                self.memory[self.i + 1] =(self.v[x as usize] / 10) % 10;
                self.memory[self.i + 2] = self.v[x as usize] % 10;
            },
            0xF055..=0xFF55 =>{ // []
                for i in 0..0xF{
                    self.memory[self.i + i] = self.v[i];
                }
            },
            0xF065..=0xFF65 =>{ // []
                for i in 0..0xF{
                    self.v[i] = self.memory[self.i + i];
                }
            },
            _ => println!("Unknown opcode: {:x?}", self.opcode),
        }

        self.pc += 2;
    }

    pub fn emulate_cycle(&mut self){
        self.opcode = self.decode_opcode();
        self.execute_opcode();
        //update timers
    }
}
