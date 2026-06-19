use std::thread;

pub struct Profiler {
    elapsed_time: u128,
    peak_ram_usage: u64,
}

impl Profiler {
    pub fn new() -> Profiler {
        Self {
            elapsed_time: 0_u128, 
            peak_ram_usage: 0_u64,
        }
    }
    pub fn run(&mut self, rx: std::sync::mpsc::Receiver<bool>) {
        let start_time = std::time::Instant::now();
        let mut mem_usage: u64;
        let pid = sysinfo::get_current_pid().unwrap();
        let mut system = sysinfo::System::new();
        system.refresh_all();
        let process = system.process(pid).unwrap();
        'main_loop: loop {
            // do the job here
            // ram_sampler.push(process.memory());
            mem_usage = process.memory();
            self.peak_ram_usage = if self.peak_ram_usage > process.memory() {
                self.peak_ram_usage
            } else {
                mem_usage
            };
            // end logic
            thread::sleep(std::time::Duration::from_micros(10_u64));
            if let Ok(_msg) = rx.try_recv() {
                break 'main_loop;
            }
        }
        self.elapsed_time = start_time.elapsed().as_millis();
        // self.average_ram_usage = rams; // ram_sampler.iter().sum();
        // let len: u64  = ram_sampler.iter().len() as u64;
        println!("\n[*] Elapsed Time: {} ms\n[*] Peak Memory Usage:\t{} KiB", self.elapsed_time, self.peak_ram_usage/1024);
    }
}