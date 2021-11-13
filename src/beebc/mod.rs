pub mod examples;
pub mod signal;
pub mod opcode;

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

// Micro Code - Mapping [op_code][step_count] -> control word
// Skips the first 2 fetch steps, hard-coded in decode_instruction.
const UC: [[u16; 6]; 16] = [
    [0,0,0,0,0,0], // No-op
    [signal::IO | signal::MI, // LDA
     signal::RO | signal::AI,
     0,0,0,0], 
    [signal::IO | signal::MI, // ADD
     signal::RO | signal::BI,
     signal::EO | signal::AI | signal::FI,
     0,0,0],
    [signal::IO | signal::MI, // SUB
     signal::RO | signal::BI,
     signal::EO | signal::AI | signal::SU | signal::FI,
     0,0,0],
    [signal::IO | signal::MI, // STA
     signal::AO | signal::RI,
     0,0,0,0],
    [signal::IO | signal::AI, // LDI
     0,0,0,0,0],
    [signal::IO | signal::J_, // JMP
     0,0,0,0,0],
    [0,0,0,0,0,0],            // JC - Handled Later
    [0,0,0,0,0,0],            // JZ - ^
    [0,0,0,0,0,0], // NOP 
    [0,0,0,0,0,0], // NOP
    [0,0,0,0,0,0], // NOP
    [0,0,0,0,0,0], // NOP
    [0,0,0,0,0,0], // NOP
    [signal::AO | signal::OI, // OUT
     0,0,0,0,0], 
    [signal::HLT,             // HLT
     0,0,0,0,0]
];

// The current state of a machine maps to a new control word. 
pub fn decode_instruction(ebc: &mut EBC) -> u16 {
	// Hardcode Fetch steps.
    if ebc.sc == 0 { return signal::MI | signal::CO; }      
    if ebc.sc == 1 { return signal::RO | signal::II | signal::CE; }

    let op_code = ebc.ir >> 4; // op_code is in the top 4 bits of the IR
    if ebc.sc == 2 {
        // Handle Jump-Carry instructions.
        if(op_code == opcode::JC) && 
           (ebc.reg_flgs & signal::CF > 0) { return  signal::IO | signal::J_; }
        if(op_code == opcode::JZ) && 
           (ebc.reg_flgs & signal::ZF > 0) { return  signal::IO | signal::J_; }
    }
    // Otherwise, index into microcode to find the new CW
    // Note - offset the step counter by 2, b/c 0 & 1 are hardcoded
    return UC[op_code as usize][(ebc.sc-2) as usize];
}

// The control word is used to update the state of the machine.
pub fn update_modules(ebc: &mut EBC, cw: u16, mut ram: [u8; 16]){
    // Bus-Write Operations First.
    if (cw & signal::RO) > 0 { // Ram Out. 
        ebc.bus = ram[ebc.mar as usize];
    }
    if (cw & signal::IO) > 0 { // Instruction Out.
        ebc.bus = ebc.ir & 0b00001111;
    }
    if (cw & signal::AO) > 0 { // A Register Out.
        ebc.bus = ebc.reg_a;
    }
    if (cw & signal::CO) > 0 { // PC Register Out.
        ebc.bus = ebc.pc;
    }

    // ALU Update - Save the optional result, so you can manage flags
    let result: Option<u8>; // Will set CF on over or underflow.
    if (cw & signal::SU) > 0 {
        result = ebc.reg_a.checked_sub(ebc.reg_b);
    } else {
        result = ebc.reg_a.checked_add(ebc.reg_b);
    }
    match result {
        Some(val) => {
            ebc.reg_alu = val;
            if (cw & signal::FI) > 0 && val == 0 {
                ebc.reg_flgs = signal::ZF;
            }
        },
        None => {
            if (cw & signal::FI) > 0 {
                ebc.reg_flgs = signal::CF;
            }
        }
    }

    if (cw & signal::EO) > 0 {
        ebc.bus = ebc.reg_alu;
    }

    // Bus-Read Operations Second.
    if (cw & signal::MI) > 0 { // Memory Address Register In.
        ebc.mar = ebc.bus & 0b00001111;
    }
    if (cw & signal::RI) > 0 { // RAM In. To location in MAR from bus.
        ram[ebc.mar as usize] = ebc.bus;
    }
    if (cw & signal::II) > 0 { // Instruction Register In.
        ebc.ir = ebc.bus;
    }
    if (cw & signal::AI) > 0 { // A Register In.
        ebc.reg_a = ebc.bus;
    }
    if (cw & signal::BI) > 0 { // B Register In.
        ebc.reg_b = ebc.bus;
    }
    if (cw & signal::OI) > 0 { // OUT Register In.
        ebc.reg_out = ebc.bus;
    }
    if (cw & signal::J_) > 0 { // Program Counter In. Jump.
        ebc.pc = ebc.bus & 0b00001111;
    }
    
    ebc.sc = (ebc.sc + 1) & 0b111; // Micro step counter

    if (cw & signal::CE) > 0 { // Increment PC on 'Counter Enable' signal. 
        ebc.pc = ebc.pc + 1;
    }

    if (cw & signal::HLT) > 0 {
        ebc.hlt = true;
    }
}