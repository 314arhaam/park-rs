mod clitools;
mod commands;
mod iotools;
mod profiler;
use clap::Parser;
use core::time;
use std::thread;
use std::sync::mpsc;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let pid = sysinfo::get_current_pid().unwrap();
    println!("{}", pid);
    thread::sleep(std::time::Duration::from_secs(10));
    let (tx, rx) = mpsc::channel();
    let mut prof = profiler::profiler::Profiler::new();
    let handler = thread::spawn(move || {prof.run(rx)});
    let cli = clitools::Cli::parse();
    cli.command.run().unwrap();
    tx.send(true).unwrap();
    handler.join().unwrap();
    thread::sleep(std::time::Duration::from_secs(10));
    Ok(())
}