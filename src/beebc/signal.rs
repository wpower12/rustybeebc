// Control Word Signals
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

// Flag Signals
pub const CF: u8 = 0b01;
pub const ZF: u8 = 0b10;
