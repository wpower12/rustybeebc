#![allow(dead_code)]
extern crate rustybeebc;
use rustybeebc::beebc;
use clap::{Arg, App};

fn main() {

    let ebc_cli = App::new("beebc emulator.")
            .version("0.1.0")
            .author("wpower12 <willpowe@gmail.com>")
            .about("Runs beebc machine code.")
            .arg(Arg::with_name("INPUT")
                .help("input machine code binary tile.")
                .required(true)
                .index(1))
            .get_matches();

    println!("{:?}", ebc_cli.value_of("INPUT").unwrap());

    let mut ebc = beebc::EBC::default();
    let ram = beebc::examples::ADD42;

    while !ebc.hlt {
        let new_cw = beebc::decode_instruction(&mut ebc);
        beebc::update_modules(&mut ebc, new_cw, ram);
        if (new_cw & beebc::signal::OI) > 0 { 
            println!("{:?}", ebc.reg_out);
        }
    }
}