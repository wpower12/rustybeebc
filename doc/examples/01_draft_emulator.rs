#[derive(Default, Debug)]
struct EBC { // Full state of the machine, besides ram.
    pc:       u8, // Program Counter
    sc:       u8, // Micro-[s]tep counter
    mar:      u8, // Memory Address Register
    ir:       u8, // Instruction Register
    reg_a:    u8,
    reg_b:    u8,
    reg_alu:  u8, // ALU Output register
    reg_out:  u8, // Readout screen register
    reg_flgs: u8, // Flags / Friendly local game store.
    bus:      u8, // The Bus / Jerome Bettis
    hlt:      bool, // Signal computation has halted
}

// Signals, used to compose cw's.
const HLT: u16  = 0b1000000000000000;  // Halt clock
const MI:  u16  = 0b0100000000000000;  // Memory address register in
const RI:  u16  = 0b0010000000000000;  // RAM data in
const RO:  u16  = 0b0001000000000000;  // RAM data out
const IO:  u16  = 0b0000100000000000;  // Instruction register out
const II:  u16  = 0b0000010000000000;  // Instruction register in
const AI:  u16  = 0b0000001000000000;  // A register in
const AO:  u16  = 0b0000000100000000;  // A register out
const EO:  u16  = 0b0000000010000000;  // ALU out
const SU:  u16  = 0b0000000001000000;  // ALU subtract
const BI:  u16  = 0b0000000000100000;  // B register in
const OI:  u16  = 0b0000000000010000;  // Output register in
const CE:  u16  = 0b0000000000001000;  // Program counter enable
const CO:  u16  = 0b0000000000000100;  // Program counter out
const J_:  u16  = 0b0000000000000010;  // Jump (program counter in)
const FI:  u16  = 0b0000000000000001;  // Flags In. 

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
fn decode_instruction(ebc: &mut EBC) -> u16 {
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
fn update_modules(ebc: &mut EBC, cw: u16, mut ram: [u8; 16]){
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

fn tick(ebc: &mut EBC, ram: [u8; 16]){
    let new_cw = decode_instruction(ebc);
    update_modules(ebc, new_cw, ram);

    // If there's an out signal, print the contents of the
    // output register. Where to rerender?
    if (new_cw & OI) > 0 { // its 'old' at this point, tbh.
        println!("{:?}", ebc.reg_out);
    }
}

fn main() {
    let mut ebc = EBC::default();

    // Test Program - Add To 42 
    // let ram: [u8; 16] = [
    //     0b00010100, // LDA [4] - Load from memory at 4
    //     0b00100101, // ADD [5] - Load from memory at 5 into B, put A+B in A
    //     0b11100000, // OUT     - Put A in OR, and Display
    //     0b11110000, // HLT     - Halt        
    //     0b00001110, // [4]     - 14
    //     0b00011100, // [5]     - 28
    //     0b00000000, //
    //     0b00000000, //       
    //     0b00000000, //
    //     0b00000000, //
    //     0b00000000, //
    //     0b00000000, //               
    //     0b00000000, //
    //     0b00000000, //
    //     0b00000000, //
    //     0b00000000];//

    // Test Program - Subtract 2 values.
    // let ram: [u8; 16] = [ 
    //     0b00010101, // LDA [4] - Load from memory at 5
    //     0b00110100, // SUB [5] - Load from memory at 4 into B, put A-B in A
    //     0b11100000, // OUT     - Put A in OR, and Display
    //     0b11110000, // HLT     - Halt        
    //     0b00001110, // [4]     - 14
    //     0b00011100, // [5]     - 28
    //     0b00000000, //
    //     0b00000000, //       
    //     0b00000000, //
    //     0b00000000, //
    //     0b00000000, //
    //     0b00000000, //               
    //     0b00000000, //
    //     0b00000000, //
    //     0b00000000, //
    //     0b00000000];//

    // Test Program - Repeat: Add up till CF, Sub down till CF.
    let ram: [u8; 16] = [ 
        0b11100000, // [00] OUT
        0b00101000, // [01] ADD 8
        0b01110100, // [02] JC  4
        0b01100000, // [03] JMP 0
        0b00111000, // [04] SUB 8
        0b11100000, // [05] OUT
        0b10000000, // [06] JZ  0
        0b01100100, // [07] JMP 4 
        0b00000001, // [08] 1
        0b00000000, // [09]
        0b00000000, // [10]              
        0b00000000, // [11]
        0b00000000, // [12]
        0b00000000, // [13]
        0b00000000, // [14] 
        0b00000000];// [15] 

    while !ebc.hlt {
        tick(&mut ebc, ram);
    }
}
