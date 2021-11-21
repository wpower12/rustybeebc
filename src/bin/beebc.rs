#![allow(dead_code)]
extern crate rustybeebc;
use rustybeebc::beebc;
use clap::{Arg, App};
use std::time;
use std::fs::File;
use std::io::{self, stdout, Read};
use termion::{
    event::{Key},
    input::{TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::{TermionBackend},
    Terminal
};

fn main() -> io::Result<()> {
    let ebc_cli = App::new("beebc emulator.")
            .version("0.1.0")
            .author("wpower12 <willpowe@gmail.com>")
            .about("Runs beebc machine code.")
            .arg(Arg::with_name("INPUT")
                .help("input machine code binary tile.")
                .required(true)
                .index(1))
            .arg(Arg::with_name("gui")
                .short("g")
                .long("gui")
                .help("Starts emulator in graphical mode."))
            .get_matches();

    let file_name = String::from(ebc_cli.value_of("INPUT").unwrap());
    println!("running: {:?}", file_name);


    let mut ebc = beebc::EBC::default();
    let mut ram: [u8; 16] = [0; 16];    
    let mut f = File::open(file_name)?; 
    let _ = f.read(&mut ram[..])?;
    
    if ebc_cli.occurrences_of("gui") > 0 {
        // Graphical Mode
        let stdout = stdout().into_raw_mode()?;
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        let mut stdin = termion::async_stdin().keys();
        
        let mut running = true;

        let mut now = time::Instant::now();
        let tick_period = time::Duration::from_millis(500);

        loop {
            //handle inputs
            if let Some(Ok(key)) = stdin.next() {
                match key {
                    Key::Char('q') => return Ok(()),
                    Key::Char('p') => running = !running,
                    Key::Char('s') => {
                        let cw = beebc::decode_instruction(&ebc);
                        beebc::update_modules(&mut ebc, cw, ram);
                        beebc::gui::render(&mut terminal, &ebc, ram, cw);
                    },
                    _ => {}
                }
            }

            // Handle Timers - To control cpu speed.
            if now.elapsed() > tick_period && running {
                let cw = beebc::decode_instruction(&ebc);
                beebc::update_modules(&mut ebc, cw, ram);
                beebc::gui::render(&mut terminal, &ebc, ram, cw);
                now = time::Instant::now();
            }
        }

    } else {
        // "Batch" mode
        while !ebc.hlt {
            let new_cw = beebc::decode_instruction(&ebc);
            beebc::update_modules(&mut ebc, new_cw, ram);
            if (new_cw & beebc::signal::OI) > 0 { 
                println!("{:?}", ebc.reg_out);
            }
        }
    }

    Ok(())
}