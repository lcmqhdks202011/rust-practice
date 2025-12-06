use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
// use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;

pub fn write_loop(outfile: &str, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        // //todo receive vector from stats thread
        // let buffer: Vec<u8> = Vec::new();
        let buffer = write_rx.recv().unwrap();
        if buffer.is_empty() {
            break;
        }
        // {
        //     let quit = quit.lock().unwrap();
        //     if *quit {
        //         break;
        //     }
        // }
        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                // break;
                //Stop Cleanly
                return Ok(());
            }
            return Err(e);
        }
    }

    // keep going
    Ok(())
}
