# 2021-11-14 - Organizing Module, Working on Assembler

Had to clean up the directory layout. Learned more about what the lib.rs file is for. It's almost like a `__init__.py` file.

The assembler and the emulator are now their own stand alone entry points in the `src/bin/` directory. Source files in this directory are automagically compiled during a cargo build. You can run them with `cargo run --bin <BIN NAME>`.

## Assembler
Really forcing me to learn about the borrow checker system. I have working stubs that read in a file and spit out the line that was read in. Right now the stub application for the emulator does nothing. 

Both are using the `clap` crate to handle the CLI stuff. 

## Working Assembler and Emulator
Ok looks like both of these are working now. They are both in need of a refactor to make them more "rusty". They do a lot of hacky stuff to basically ignore all the Err's. 

But they work! It can assemble the add sub program and run it.

Will now need a proper readme.