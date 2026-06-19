mod clitools;
mod commands;
mod iotools;
mod profiler;
use clap::Parser;
use std::thread;
use std::sync::mpsc;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let (tx, rx) = mpsc::channel();
    let mut prof: profiler::profiler::Profiler = profiler::profiler::Profiler::new();
    let handler: thread::JoinHandle<()> = thread::spawn(move || {prof.run(rx)});
    let cli: clitools::Cli = clitools::Cli::parse();
    cli.command.run().unwrap();
    tx.send(true).unwrap();
    handler.join().unwrap();
    Ok(())
}