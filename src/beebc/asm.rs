use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::collections::HashMap;

pub fn assemble(file_name: &String, out_name: &String){
	let op_map: HashMap<&str, [u8; 2]> = HashMap::from([
		("NOP", [0b0000, 0]), // "<OP_NAME>", [BIN_VAL, #_OPERANDS]
		("LDA", [0b0001, 1]), 
		("ADD", [0b0010, 1]), 
		("SUB", [0b0011, 1]), 
		("STA", [0b0100, 1]), 
		("LDI", [0b0101, 1]), 
		("JMP", [0b0110, 1]), 
		("JC" , [0b0111, 1]), 
		("JZ" , [0b1000, 1]),
		("OUT", [0b1110, 0]), 
		("HLT", [0b1111, 0])
	]);

	// First pass over lines to fill in symbol_table.
	let mut symbol_table = HashMap::new();
    if let Ok(lines) = read_lines(file_name){
    	let mut lc = 0;
    	for line in lines {
    		if let Ok(l) = line {
    			let mut words: Vec<&str> = l.split(" ").collect();
    			words.retain(|s| *s != "");
				if is_symbol(&words[0], &op_map){
					symbol_table.insert(String::from(words[0]), lc);
				} else {
					lc = lc+1;
				}
    		}
    	}
    }

    // Second pass to actually build machine code.
    let mut mc: [u8; 16] = [0; 16];
    if let Ok(lines) = read_lines(file_name){
    	let mut lc = 0;
    	for line in lines {
    		if let Ok(l) = line {
    			let mut words: Vec<&str> = l.split(" ").collect();
    			words.retain(|s| *s != "");
    			let token_0 = String::from(words[0]);
    			if !is_symbol(&token_0, &op_map){
    				if is_u8_literal(&token_0) {
    					mc[lc] = token_0.parse::<u8>().unwrap();
    				} else {
    					let op = op_map.get(words[0]).unwrap();
    					if op[1] == 0 { // 0 Operand Op Code.
    						mc[lc] = op[0] << 4;
    					} else { // Single Operand Op Code.
    						let operand = String::from(words[1]);
    						if is_symbol(&operand, &op_map){
    							mc[lc] = (op[0] << 4) | symbol_table[&operand];
    						} else {
    							let literal = operand.parse::<u8>().unwrap();
    							mc[lc] = (op[0] << 4) | literal;
    						}
    					}
    				}
    				lc = lc+1;
    			}
    		}
    	}
    }
    let mut file = File::create(&out_name).unwrap();
    file.write_all(&mc).unwrap();
	println!("assembled {:?} to {:?}", file_name, out_name);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn is_symbol(v: &str, op_map: &HashMap<&str, [u8; 2]>) -> bool {
	!is_op_code(v, op_map) & !is_u8_literal(v) 
}

fn is_op_code(v: &str, op_map: &HashMap<&str, [u8; 2]>) -> bool {
	op_map.contains_key(v)
}

fn is_u8_literal(v: &str) -> bool {
	match String::from(v).parse::<u8>() {
		Ok(_) => return true,
		Err(_) => return false
	}
}