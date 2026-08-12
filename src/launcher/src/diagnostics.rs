use colored::*;
use sysinfo::{Disks, System};

pub struct DiagnosticReport {
    pub total_ram_mb: u64,
    pub used_ram_mb: u64,
    pub cpu_cores: usize,
    pub cpu_brand: String,
    pub disk_count: usize,
}

pub fn run_diagnostics() -> DiagnosticReport {
    println!("{}", "\n🩺 --- [NANTARA SMART HARDWARE DIAGNOSTICS] ---".yellow().bold());
    
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_ram_mb = sys.total_memory() / 1024 / 1024;
    let used_ram_mb = sys.used_memory() / 1024 / 1024;
    let cpu_cores = sys.cpus().len();
    let cpu_brand = sys.cpus().first().map(|c| c.brand().trim().to_string()).unwrap_or_else(|| "Unknown CPU".to_string());

    println!("  🧠 RAM Status   : {} MB / {} MB used", used_ram_mb.to_string().cyan(), total_ram_mb.to_string().green());
    println!("  ⚡ Processor    : {} ({} Cores)", cpu_brand.cyan(), cpu_cores.to_string().green());

    let disks = Disks::new_with_refreshed_list();
    println!("  💾 Storage Drives Detected: {}", disks.len().to_string().green());

    for disk in &disks {
        let name = disk.name().to_string_lossy();
        let mount = disk.mount_point().to_string_lossy();
        let total_gb = disk.total_space() / 1024 / 1024 / 1024;
        let available_gb = disk.available_space() / 1024 / 1024 / 1024;
        let fs_type = disk.file_system().to_string_lossy();

        println!(
            "     - [{}] Drive '{}' ({}) - {} GB free / {} GB total",
            mount.yellow().bold(),
            name.cyan(),
            fs_type.dimmed(),
            available_gb.to_string().green(),
            total_gb.to_string().white()
        );
    }

    println!("{}", "--------------------------------------------------".dimmed());

    DiagnosticReport {
        total_ram_mb,
        used_ram_mb,
        cpu_cores,
        cpu_brand,
        disk_count: disks.len(),
    }
}
