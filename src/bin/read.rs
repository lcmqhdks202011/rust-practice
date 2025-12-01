use std::env;
use std::io::{self, Read, Write, Result, ErrorKind, BufReader, BufWriter};
use clap::{App, Arg};
use std::fs::File;

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()>{



    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(io::stdout())
    };

    let mut total_bytes = 0;

    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes)
        }
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            // if e.kind() == ErrorKind::BrokenPipe {
            //     break;
            // }
            // return Err(e);
            eprintln!("Oh no! Something went wrong: {}", e);
        }
    }
    if !silent {
        eprintln!("num_read: {}", total_bytes);
    }

    Ok(())
}
