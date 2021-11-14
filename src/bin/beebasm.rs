#![allow(dead_code)]
extern crate rustybeebc;
use rustybeebc::beebc;

use clap::{Arg, App};

fn main() {
    let ebc_cli = App::new("beebc machine code assembler.")
            .version("0.1.0")
            .author("wpower12 <willpowe@gmail.com>")
            .about("Assembles beebc machine code to actual binary file.")
            .arg(Arg::with_name("INPUT")
                .help("input machine code file.")
                .required(true)
                .index(1))
            .get_matches();

    let file_name = String::from(ebc_cli.value_of("INPUT").unwrap());

    beebc::asm::assemble(&file_name);
}
