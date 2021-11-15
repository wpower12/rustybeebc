#![allow(dead_code)]
extern crate rustybeebc;
use rustybeebc::beebc;

use clap::{Arg, App};

fn main() {
    let ebc_cli = App::new("beebc machine code assembler.")
            .version("0.1.0")
            .author("wpower12 <willpowe@gmail.com>")
            .about("Assembles beebc machine code to a binary file.")
            .arg(Arg::with_name("INPUT")
                .help("input assembly language file.")
                .required(true)
                .index(1))
            .arg(Arg::with_name("OUTPUT")
                .help("output binary machine code file.")
                .required(true)
                .index(2))
            .get_matches();

    let file_name = String::from(ebc_cli.value_of("INPUT").unwrap());
    let out_name  = String::from(ebc_cli.value_of("OUTPUT").unwrap());
    beebc::asm::assemble(&file_name, &out_name);
}
