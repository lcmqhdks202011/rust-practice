use crate::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};
// use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;

pub fn read_loop(infile: &str, stats_tx: Sender<Vec<u8>>) -> Result<()> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut buffer = [0; CHUNK_SIZE];
    // let mut total_bytes = 0;
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        // todo: send this buffer to the stats thread
        if stats_tx.send(Vec::from(&buffer[..num_read])).is_err() {
            break;
        }
    }

    let _ = stats_tx.send(Vec::new());
    // // todo: send an empty buffer to the stats thread
    // let mut quit = quit.lock().unwrap();
    //
    // *quit = true;

    // // let num_read = match io::stdin().read(&mut buffer) {
    // //     Ok(0) => break,
    // //     Ok(x) => x,
    // //     Err(_) => break,
    // // };
    //
    // let num_read = match reader.read(&mut buffer) {
    //     Ok(x) => x,
    //     Err(e) => return Err(e),
    // };

    // Ok(Vec::from(&buffer[..num_read]))
    Ok(())
}
