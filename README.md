# rustybeebc
Rust implementation of an emulator for the Simple-As-Possible 8 bit computer, as made famous by [You-Tuber Ben Eater](https://eater.net/8bit/).

## Install
The crate contains two binaries; `beebasm` and `beebc`. The first is an assembler that converts a very simple assembly language into machine code, which can be run by the second program, the actual emulator. To 'install' clone, and then build the project with cargo. 

## Run

```bash
// Running with cargo
> cargo run --bin beebasm addsub.ebc addsub
> cargo run --bin beebc addsub.o
```

The example script should already be in the project root directory, so the above should run. 

Note; I don't understand enough about the rust file io to know whats going wrong, but there's an issue with passing output file names to the assembler that contain file extensions. So for now, just don't use one? It's fine reading them in, but not writing to them? More to learn!


## BEEB Assembly Language
The assembly language is very simple. It only recognizes the set of opcodes and integer literals. Any other encountered strings are treated as symbols to be resolved by the symbol table.

The following is an example assembly file for the add-sub-loop program. This is also found in the e `examples` submodule of the `rustybeebd` crate.

```
// addsub.ebc
ADD_LOOP
  OUT
  ADD VAL
  JC SUB_LOOP
  JMP ADD_LOOP
SUB_LOOP
  SUB VAL
  OUT
  JZ ADD_LOOP
  JMP SUB_LOOP
VAL
  1
```

One thing to note is that the tokens should be separated by single spaces, and that any spaces at the start of lines are stripped. The indents are for human use only. 

The following table lists the **reserved keywords** for the assembly language.

| opcode| value | # operands | Desc. | Note |
| --- | --- | --- | --- | --- |
| NOP | 0b0000 | 0 | No Op | |
| LDA | 0b0001 | 1 | Ram\[Operand\] to RegA | | 
| ADD | 0b0010 | 1 | Ram\[Operand\] to RegB, A+B to regALU | Can Set Carry Flags |
| SUB | 0b0011 | 1 | Ram\[Operand\] to RegB, A-B to regALU | Can Set Carry Flags |
| STA | 0b0100 | 1 | RegA to Ram\[Operand\] 
| LDI | 0b0101 | 1 | Operand to RegA | | 
| JMP | 0b0110 | 1 | Operand to PC | | 
| JC  | 0b0111 | 1 | Operand to PC | If carry flag is set.| 
| JZ  | 0b1000 | 1 | Operand to PC | If zero flag is set.|
| OUT | 0b1110 | 0 | RegA to regOut | Will print regOut to stdout at end of tick.| 
| HLT | 0b1111 | 0 | Stop Computation | Read by emulator to halt ticks. |

Any string that starts a line, and is not one of these keywords, will be treated as a symbol for the symbol table. 