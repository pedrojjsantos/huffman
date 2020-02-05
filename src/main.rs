mod zip;
use std::fs;

fn main() -> std::io::Result<()> { 
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        panic!("no arguments used");
    }
    let mut file = fs::File::open(&args[1])?;
    let option = &args[2];
    
    match &option[..] {
        "-c" => zip::compress(&mut file),
        //"-d" => {}
        _ => panic!( "not a valid option:\n
                     '-c' to compress\n
                     '-d' to decompress"),
    }

}
