use crossbeam::channel::{bounded, unbounded};
use rust_myproject::{args::Args, read, stats, write};
use std::io::Result;
// use std::sync::mpsc;
// use std::sync::{Arc, Mutex};

use std::thread;

fn main() -> Result<()> {
    let args = Args::parse();

    let Args {
        infile,
        outfile,
        silent,
    } = args;

    // let (stat_tx, stats_rx) = mpsc::channel();
    // let (write_tx, write_rx) = mpsc::channel();
    let (stat_tx, stats_rx) = unbounded();
    let (write_tx, write_rx) = bounded(1024);

    // let quit = Arc::new(Mutex::new(false));

    // let (quit1, quit2, quit3) = (quit.clone(), quit.clone(), quit.clone());

    // let read_handle = thread::spawn(move || read::read_loop(&infile, quit1));
    // let stats_handle = thread::spawn(move || stats::stats_loop(silent, quit2));
    // let write_handle = thread::spawn(move || write::write_loop(&outfile, quit3));
    let read_handle = thread::spawn(move || read::read_loop(&infile, stat_tx, write_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    // crash if any threads have crashed

    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}
