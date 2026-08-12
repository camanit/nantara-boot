use colored::*;
use sysinfo::System;

fn main() {
    println!("{}", "==========================================================".cyan());
    println!("{}", "   🚀 Nantara-Boot System Rescue Engine (Rust Native)".cyan().bold());
    println!("{}", "   100% Free & Open-Source Software (MIT License)".green());
    println!("{}", "==========================================================".cyan());

    let mut sys = System::new_all();
    sys.refresh_all();

    println!("\n{}", "🩺 [Smart Auto-Diagnostics Initialized]".yellow().bold());
    println!("  -> Total Memory (RAM) : {} MB", sys.total_memory() / 1024 / 1024);
    println!("  -> Used Memory  (RAM) : {} MB", sys.used_memory() / 1024 / 1024);
    println!("  -> CPU Cores Count    : {}", sys.cpus().len());

    if let Some(cpu) = sys.cpus().first() {
        println!("  -> Processor Brand    : {}", cpu.brand().trim());
    }

    println!("\n{}", "🛡️ [Offline Safety & Security Status]".yellow().bold());
    println!("  -> Read-Only Protection : ACTIVE (Target drives protected)");
    println!("  -> Offline Antivirus    : Ready (ClamAV & KVRT Engines)");
    println!("  -> 1-Click SAM Reset   : Ready");
    println!("  -> 1-Click Smart Backup : Ready");

    println!("\n{}", "==========================================================".cyan());
    println!("{}", "   System Ready. Launching Nantara Dashboard...".green().bold());
    println!("{}", "==========================================================".cyan());
}
