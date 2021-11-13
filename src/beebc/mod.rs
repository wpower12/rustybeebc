pub mod examples;

#[derive(Default, Debug)]
pub struct EBC { // Full state of the machine, besides ram.
    pub pc:       u8, // Program Counter
    pub sc:       u8, // Micro-[s]tep counter
    pub mar:      u8, // Memory Address Register
    pub ir:       u8, // Instruction Register
    pub reg_a:    u8,
    pub reg_b:    u8,
    pub reg_alu:  u8, // ALU Output register
    pub reg_out:  u8, // Readout screen register
    pub reg_flgs: u8, // Flags / Friendly local game store.
    pub bus:      u8, // The Bus / Jerome Bettis
    pub hlt:      bool, // Signal computation has halted
}

// Signals, used to compose cw's.
pub const HLT: u16  = 0b1000000000000000;  // Halt clock
pub const MI:  u16  = 0b0100000000000000;  // Memory address register in
pub const RI:  u16  = 0b0010000000000000;  // RAM data in
pub const RO:  u16  = 0b0001000000000000;  // RAM data out
pub const IO:  u16  = 0b0000100000000000;  // Instruction register out
pub const II:  u16  = 0b0000010000000000;  // Instruction register in
pub const AI:  u16  = 0b0000001000000000;  // A register in
pub const AO:  u16  = 0b0000000100000000;  // A register out
pub const EO:  u16  = 0b0000000010000000;  // ALU out
pub const SU:  u16  = 0b0000000001000000;  // ALU subtract
pub const BI:  u16  = 0b0000000000100000;  // B register in
pub const OI:  u16  = 0b0000000000010000;  // Output register in
pub const CE:  u16  = 0b0000000000001000;  // Program counter enable
pub const CO:  u16  = 0b0000000000000100;  // Program counter out
pub const J_:  u16  = 0b0000000000000010;  // Jump (program counter in)
pub const FI:  u16  = 0b0000000000000001;

// ALU Flags.
const CF: u8 = 0b01;
const ZF: u8 = 0b10;

// Machine Codes - Would eventually be used by an assembler? I think?
// const MC_NOP: u8 = 0b0000; 
// const MC_LDA: u8 = 0b0001; 
// const MC_ADD: u8 = 0b0010;
// const MC_SUB: u8 = 0b0011;
// const MC_STA: u8 = 0b0100;
// const MC_LDI: u8 = 0b0101;
// const MC_JMP: u8 = 0b0110;
const MC_JC : u8 = 0b0111; 
const MC_JZ : u8 = 0b1000;
// const MC_OUT: u8 = 0b1110;
// const MC_HLT: u8 = 0b1111;   


// Micro Code - Mapping [op_code][step_count] -> control word
// Skips the first 2 fetch steps, hard-coded in decode_instruction.
const UC: [[u16; 6]; 16] = [
    [0,0,0,0,0,0], // No-op
    [IO | MI,      // LDA
     RO | AI,
     0,0,0,0], 
    [IO | MI,      // ADD
     RO | BI,
     EO | AI | FI,
     0,0,0],
    [IO | MI,      // SUB
     RO | BI,
     EO | AI | SU | FI,
     0,0,0],
    [IO | MI,      // STA
     AO | RI,
     0,0,0,0],
    [IO | AI,      // LDI
     0,0,0,0,0],
    [IO | J_,      // JMP
     0,0,0,0,0],
    [0,0,0,0,0,0], // JC - Handled Later
    [0,0,0,0,0,0], // JZ - ^
    [0,0,0,0,0,0], // NOP 
    [0,0,0,0,0,0], // NOP
    [0,0,0,0,0,0], // NOP
    [0,0,0,0,0,0], // NOP
    [0,0,0,0,0,0], // NOP
    [AO | OI,      // OUT
     0,0,0,0,0], 
    [HLT,          // HLT
     0,0,0,0,0]
];

// The current state of a machine maps to a new control word. 
pub fn decode_instruction(ebc: &mut EBC) -> u16 {
    if ebc.sc == 0 { return MI | CO; }      // Fetch
    if ebc.sc == 1 { return RO | II | CE; }

    let op_code = ebc.ir >> 4; // op_code is in the top 4 bits of the IR
    if ebc.sc == 2 {
        // Handle Jump-Carry instructions.
        if(op_code == MC_JC) && 
           (ebc.reg_flgs & CF > 0) { return  IO | J_; }
        if(op_code == MC_JZ) && 
           (ebc.reg_flgs & ZF > 0) { return  IO | J_; }
    }
    // Otherwise, index into microcode to find the new CW
    // Note - offset the step counter by 2, b/c 0 & 1 are hardcoded
    return UC[op_code as usize][(ebc.sc-2) as usize];
}

// The control word is used to update the state of the machine.
pub fn update_modules(ebc: &mut EBC, cw: u16, mut ram: [u8; 16]){
    // Bus-Write Operations First.
    if (cw & RO) > 0 { // Ram Out. 
        ebc.bus = ram[ebc.mar as usize];
    }
    if (cw & IO) > 0 { // Instruction Out.
        ebc.bus = ebc.ir & 0b00001111;
    }
    if (cw & AO) > 0 { // A Register Out.
        ebc.bus = ebc.reg_a;
    }
    if (cw & CO) > 0 { // A Register Out.
        ebc.bus = ebc.pc;
    }

    // ALU Update - Save the optional result, so you can manage flags
    let result: Option<u8>; // Will set CF on over or underflow.
    if (cw & SU) > 0 {
        result = ebc.reg_a.checked_sub(ebc.reg_b);
    } else {
        result = ebc.reg_a.checked_add(ebc.reg_b);
    }
    let mut over_flowed = false;
    match result {
        Some(val) => ebc.reg_alu = val,
        None      => over_flowed = true
    }

    if (cw & FI) > 0  { // ALU Flags In
        ebc.reg_flgs = 0;
        if over_flowed { ebc.reg_flgs = CF; }
        if ebc.reg_alu == 0 { ebc.reg_flgs = ZF; }
    }

    if (cw & EO) > 0 {
        ebc.bus = ebc.reg_alu;
    }

    // Bus-Read Operations Second.
    if (cw & MI) > 0 { // Memory Address Register In.
        ebc.mar = ebc.bus & 0b00001111;
    }
    if (cw & RI) > 0 { // RAM In. To location in MAR from bus.
        ram[ebc.mar as usize] = ebc.bus;
    }
    if (cw & II) > 0 { // Instruction Register In.
        ebc.ir = ebc.bus;
    }
    if (cw & AI) > 0 { // A Register In.
        ebc.reg_a = ebc.bus;
    }
    if (cw & BI) > 0 { // B Register In.
        ebc.reg_b = ebc.bus;
    }
    if (cw & OI) > 0 { // OUT Register In.
        ebc.reg_out = ebc.bus;
    }
    if (cw & J_) > 0 { // Program Counter In. Jump.
        ebc.pc = ebc.bus & 0b00001111;
    }
    
    ebc.sc = (ebc.sc + 1) & 0b111; // Micro step counter

    if (cw & CE) > 0 { // Increment PC on 'Counter Enable' signal. 
        ebc.pc = ebc.pc + 1;
    }

    if (cw & HLT) > 0 {
        ebc.hlt = true;
    }

}