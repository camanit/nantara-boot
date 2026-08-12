mod ai;
mod antivirus;
mod diagnostics;
mod rescue;

use colored::*;
use std::io::{self, Write};

fn main() {
    println!("{}", "==========================================================".cyan());
    println!("{}", "   🚀 Nantara-Boot System Rescue Engine (Rust Native)".cyan().bold());
    println!("{}", "   🤖 Integrated with Nantara AI Rescue Assistant".yellow().bold());
    println!("{}", "   100% Free & Open-Source Software (MIT License)".green());
    println!("{}", "==========================================================".cyan());

    // 1. Run Initial Smart Hardware Auto-Diagnostics
    let _report = diagnostics::run_diagnostics();

    // 2. Check Antivirus Status
    let _av_engines = antivirus::check_antivirus_engines();

    println!("\n{}", "📋 --- [NANTARA RESCUE CONTROL PANEL] ---".yellow().bold());
    println!("  1. Run Full Hardware & Storage Scan");
    println!("  2. Execute Offline Malware & Virus Scan");
    println!("  3. 1-Click Smart File Backup (Desktop, Docs, Downloads)");
    println!("  4. 1-Click Windows Password Reset (SAM Database)");
    println!("  5. Ask Nantara AI Assistant (BSOD & Crash Diagnostic)");
    println!("  6. Exit Nantara Launcher");
    println!("{}", "--------------------------------------------------".dimmed());

    print!("{}", "Select option (1-6) [Auto-Exit in PE mode]: ".cyan().bold());
    let _ = io::stdout().flush();

    // Demonstrate interactive CLI / diagnostic preview
    println!("\n{}", "✨ Nantara Engine is running in Live Diagnostic mode.".green().bold());
    
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
    println!("{}", "   System Ready. Nantara Rescue Kernel Active.".green().bold());
    println!("{}", "==========================================================".cyan());
}
