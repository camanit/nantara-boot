mod ai;
mod antivirus;
mod diagnostics;
mod rescue;
mod window;

use colored::*;
use std::io::{self, Write};

fn main() {
    println!("{}", "==========================================================".cyan());
    println!("{}", "   🚀 Nantara-Boot System Rescue Engine v2.0".cyan().bold());
    println!("{}", "   🖥️ Native Standalone Desktop Window Container Active".magenta().bold());
    println!("{}", "   🤖 Integrated with Nantara AI Rescue Assistant".yellow().bold());
    println!("{}", "   100% Free & Open-Source Software (MIT License)".green());
    println!("{}", "==========================================================".cyan());

    // 1. Launch Native Standalone Desktop App Window (v2.0 Engine - No Browser Bar)
    window::launch_native_desktop_window();

    // 2. Run Initial Smart Hardware Auto-Diagnostics
    let _report = diagnostics::run_diagnostics();

    // 3. Check Antivirus Status
    let _av_engines = antivirus::check_antivirus_engines();

    println!("\n{}", "📋 --- [NANTARA RESCUE CONTROL PANEL v2.0] ---".yellow().bold());
    println!("  1. Run Full Hardware & Storage Scan");
    println!("  2. Execute Offline Malware & Virus Scan");
    println!("  3. 1-Click Smart File Backup (Desktop, Docs, Downloads)");
    println!("  4. 1-Click Windows Password Reset (SAM Database)");
    println!("  5. Ask Nantara AI Assistant (BSOD & Crash Diagnostic)");
    println!("  6. Exit Nantara Launcher");
    println!("{}", "--------------------------------------------------".dimmed());

    // Test sample 1-click rescue & AI calls
    rescue::run_1click_sam_reset("C");
    rescue::run_1click_backup("C", "D");
    antivirus::scan_target_drive("C");

    // Test Nantara AI Assistant query
    let ai_sample = ai::AiDiagnosticQuery {
        error_code: "INACCESSIBLE_BOOT_DEVICE".to_string(),
        system_logs: "SATA / NVMe controller driver missing".to_string(),
        is_online: false, // Local SLM mode
    };
    ai::run_ai_assistant_query(&ai_sample);

    println!("\n{}", "==========================================================".cyan());
    println!("{}", "   System Ready. Nantara v2.0 Rescue Kernel Active.".green().bold());
    println!("{}", "==========================================================".cyan());

    // Keep console window open when double-clicked in File Explorer
    print!("\n{}", "Tekan Enter untuk menutup jendela Nantara Launcher... ".yellow().bold());
    let _ = io::stdout().flush();
    let mut exit_input = String::new();
    let _ = io::stdin().read_line(&mut exit_input);
}
