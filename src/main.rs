use std::process::Command;
use chrono::{Local, TimeZone};
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    println!("Process Name | PID | CPU% | Memory | Status | Start Time | User | Command");
    println!("-------------|-----|------|--------|--------|------------|------|--------");

    for (pid, process) in system.processes() {
        let cpu_usage = process.cpu_usage();
        let memory = process.memory() / 1024 / 1024; // Convert to MB
        let status = process.status().to_string();
        let start_time = Local.timestamp_opt(process.start_time() as i64, 0)
       .unwrap()
       .format("%Y-%m-%d %H:%M:%S");
        let user = get_process_user(pid);
        let command = process.cmd().join(" ");

        println!(
            "{:<12} | {:>4} | {:>4.1}% | {:>6} MB | {:<7} | {} | {:<10} | {}",
            process.name(), pid, cpu_usage, memory, status, start_time, user, command
        );
    }
}

fn get_process_user(pid: &sysinfo::Pid) -> String {
    let output = Command::new("ps")
        .args(&["-o", "user=", "-p", &pid.to_string()])
        .output()
        .expect("Failed to execute ps command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
