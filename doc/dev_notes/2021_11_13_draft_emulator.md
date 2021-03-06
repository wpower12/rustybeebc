# 2021-11-13 - Draft Emulator

Ok so I think I have a working mvp. The guts of the js one are all in rust now, and I have two simple methods acting on a state struct, a ram object, and a control word. Emulation occurs 'tick by tick' of the clock, with each tick being an update to all modules, based on a control word pulled from the micro code. 

I have some examples in the code now. I'll make this the first 'examples' script in the docs directory. 

Fun side note; I think the auto-doc generator for crates uses an example directory to automagically produce code snippets for uesrs when they look at a crates repo.

I want to get this to a place where I can share it, so I should be specific about the boxes I want to check before I do that.

 * organize current code into a module
 * make a cli that uses that core module
 
 ## Core Module
 I tested a few of the example programs and hit some exceptions during the sub one. Looks like i need to do a better job of handling adding and subtracting and handling the possible overflows. 

 I'm making a temporary Option variable that holds the result of checked adds and subs for the two registers. One nice thing about this is it gets rid of an explicit cast. That feels nice. 

 Also its more rust-ish? I think? What still feels gross is the flag variable I used. I just wanted to keep the logic for "process a result" and "do things when the flag in signal is seen" separate. 

 This seems to be working. All the programs are running.

 ## Quick Refactor
 I decided to take the control word out of the ebc. The current state of an ebc should map to a control word. A current state, a control word, and a ram then map to a new state for the ebc. 

 This draft is now saved to `doc/examples/01_draft_emulator.rs`.

 Now to work on wrapping it in a module? Get to learn that?

 ## Module Working
 I have a module made up. The imports were super easy to figure out. Rust is awesome. 

 ## Code Smells
 I would like to make enums or structs that actually can be called like Sig.HLT, Sig.CE etc... Not sure what the rustiest way to do that is. 

 Also, I have a feeling a control word itself should be a struct, that is composed out of signals. I should be able to make a struct like that? idk. 

 That's def the next big step. For now it can stay until I learn more rust while building out a ui. 

 ## Broke Out Signals
 I put the signals and opcodes in their own files to clean things up. Now i can keep them straight by their namespace. Probably still a better way, but this feels cleaner than before. 

The example usage, with those changes, is now in `doc/examples/02_draft_module_test.rs`.