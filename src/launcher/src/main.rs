mod ai;
mod antivirus;
mod diagnostics;
mod rescue;

use colored::*;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn open_gui_in_browser() {
    // Attempt 1: Relative to current working directory
    let mut web_path = PathBuf::from("web/index.html");

    // Attempt 2: Relative to executable path
    if !web_path.exists() {
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Try target/debug/../../../web/index.html
                let candidate = exe_dir.join("../../../web/index.html");
                if candidate.exists() {
                    web_path = candidate;
                } else {
                    // Try src/launcher/web/index.html
                    let candidate2 = exe_dir.join("../../web/index.html");
                    if candidate2.exists() {
                        web_path = candidate2;
                    }
                }
            }
        }
    }

    if web_path.exists() {
        println!("{}", "🌐 Opening Nantara Web GUI Dashboard in Browser...".green().bold());
        let _ = Command::new("cmd")
            .args(["/c", "start", "", web_path.to_str().unwrap_or("web/index.html")])
            .spawn();
    } else {
        println!("{}", "ℹ️ Web GUI assets (web/index.html) running in CLI mode.".yellow());
    }
}

fn main() {
    println!("{}", "==========================================================".cyan());
    println!("{}", "   🚀 Nantara-Boot System Rescue Engine (Rust Native)".cyan().bold());
    println!("{}", "   🤖 Integrated with Nantara AI Rescue Assistant".yellow().bold());
    println!("{}", "   100% Free & Open-Source Software (MIT License)".green());
    println!("{}", "==========================================================".cyan());

    // Automatically open Web GUI Dashboard in Browser
    open_gui_in_browser();

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

    // Keep console window open when double-clicked in File Explorer!
    print!("\n{}", "Tekan Enter untuk menutup jendela Nantara Launcher... ".yellow().bold());
    let _ = io::stdout().flush();
    let mut exit_input = String::new();
    let _ = io::stdin().read_line(&mut exit_input);
}
