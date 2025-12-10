use std::fs::read;
use std::env;

fn main() {
    let args: Vec<String> =  env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: network_target <file>");
        return;
    }
    let data = read(&args[1]).unwrap();
    match network_target::parse_packet(&data) {
        Ok(msg) => println!("Parsed: {}", msg),
        Err(e) => eprintln!("Error: {}", e),
    }
}
