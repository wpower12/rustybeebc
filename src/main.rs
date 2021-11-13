mod beebc;

fn main() {
    let mut ebc = beebc::EBC::default();
    let ram = beebc::examples::Add42;

    while !ebc.hlt {
        let new_cw = beebc::decode_instruction(&mut ebc);
        beebc::update_modules(&mut ebc, new_cw, ram);
        
        if (new_cw & beebc::OI) > 0 { 
            println!("{:?}", ebc.reg_out);
        }
    }
}