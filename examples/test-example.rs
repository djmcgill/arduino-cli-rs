use arduino_cli::*;
use std::process::exit;

pub fn main() {
    if let Err(err) = InitToken::initialise() {
        println!("ERROR: {:?}", err);
        exit(-1);
    } else {
        println!("No error")
    }
}
