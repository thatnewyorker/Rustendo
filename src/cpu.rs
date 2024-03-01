// Define the CPU module
struct Cpu6502 {
    // Registers
    a: u8, // Accumulator
    x: u8, // X Index Register
    y: u8, // Y Index Register
    sp: u8, // Stack Pointer
    pc: u16, // Program Counter
    status: u8, // Status Register

    // Memory (64KB)
    memory: [u8; 65536],
}

// Implementation of the CPU
impl Cpu6502 {
    fn new() -> Self {
        Cpu6502 {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD, // Initialized to 0xFD as per 6502's power-up state
            pc: 0x8000, // Commonly used starting address for programs
            status: 0x24, // Default status flags
            memory: [0; 65536],
        }
    }

    // Load the accumulator with a value
    fn lda_immediate(&mut self, value: u8) {
        self.a = value;
        self.update_zero_and_negative_flags(self.a);
    }

    // Update the zero and negative flags based on the result
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status |= 0b0000_0010; // Set zero flag
        } else {
            self.status &= !0b0000_0010; // Clear zero flag
        }

        if result & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000; // Set negative flag
        } else {
            self.status &= !0b1000_0000; // Clear negative flag
        }
    }

    // Memory access functions
    // These functions are used to read and write to memory
    
    // Read a byte from memory
    fn read(&self, addr: u16) -> u8 { 
        self.memory[addr as usize] 
    }
    
    // Write a byte to memory
    fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    // Read a 16-bit word from memory
    fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr + 1) as u16;
        (hi << 8) | lo
    }

    // Write a 16-bit word to memory
    fn write_word(&mut self, addr: u16, data: u16) {
        let lo = data as u8;
        let hi = (data >> 8) as u8;
        self.write(addr, lo);
        self.write(addr + 1, hi);
    }

    // Stack operations 
    fn push(&mut self, data: u8) {
        self.write(0x0100 + self.sp as u16, data);
        self.sp = self.sp.wrapping_sub(1);
    }

    // Pop a byte from the stack
    fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.read(0x0100 + self.sp as u16)
    }

    // Push a 16-bit word to the stack
    fn push_word(&mut self, data: u16) {
        self.push((data >> 8) as u8);
        self.push(data as u8);
    }

    // Pop a 16-bit word from the stack
    fn pop_word(&mut self) -> u16 {
        let lo = self.pop() as u16;
        let hi = self.pop() as u16;
        (hi << 8) | lo
    }

    // Status register operations
    fn pull_status(&mut self) {
        self.status = self.pop();
    }

}

// Path: main.rs
// Import the CPU module
// This will allow us to use the Cpu6502 struct and its methods
fn main() {
    let mut cpu = Cpu6502::new();
    // Example usage: Load the value 0x10 into the accumulator
    cpu.lda_immediate(0x10);
    println!("Accumulator: {}", cpu.a);
}

