mod zip;
//mod unzip;

use std::fs;

fn main() -> std::io::Result<()> { 
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => print!("no arguments used, the arguments must be:\n<filename or path> <-c or -d>\n"),
        2 => print!("please especify an option:\n  -c to compress\n  -d to decompress\n"),
        _ => {
            let file = fs::File::open(&args[1])?;
            let option = &args[2];
            
            match &option[..] {
                "-c" => zip::compress(file)?,
                "-d" => println!("decompressing"),// unzip::decompress(file),
                e => print!("'{}' is not a valid option:\n  '-c' to compress\n  '-d' to decompress\n", e),
            }
        }
    }

    Ok(())
}
