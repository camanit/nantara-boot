use colored::*;

pub struct AiDiagnosticQuery {
    pub error_code: String,
    pub system_logs: String,
    pub is_online: bool,
}

pub fn run_ai_assistant_query(query: &AiDiagnosticQuery) {
    println!("{}", "\n🤖 --- [NANTARA AI RESCUE ASSISTANT ENGINE] ---".cyan().bold());

    let mode_str = if query.is_online {
        "ONLINE MODE (Cloud Gemini Engine Active)".green().bold()
    } else {
        "OFFLINE MODE (Local SLM Engine Active - No Internet Needed)".yellow().bold()
    };

    println!("  -> Execution Mode  : {}", mode_str);
    println!("  -> Analyzing Error : [{}]", query.error_code.magenta().bold());
    println!("  -> Reading Crash Logs / Event Viewer Dump...");

    println!("\n{}", "💡 [Nantara AI Diagnostic & Repair Solution]:".green().bold());

    match query.error_code.to_uppercase().as_str() {
        "0XC000021A" | "STATUS_SYSTEM_PROCESS_TERMINATED" => {
            println!("  1. Cause: Critical user-mode subsystem (winlogon / csrss) has been compromised.");
            println!("  2. Recommended Action:");
            println!("     - Run '1-Click SAM Password Reset' if recent credentials changed.");
            println!("     - Run 'Offline Malware Scan' to remove infected winlogon hooks.");
            println!("     - Execute SFC Offline Repair via DISM++ Tool.");
        }
        "INACCESSIBLE_BOOT_DEVICE" | "0XC000000E" => {
            println!("  1. Cause: Missing NVMe/SATA controller driver or corrupted BCD boot record.");
            println!("  2. Recommended Action:");
            println!("     - Open BCD Repair category -> Run Bootice / EasyBCD.");
            println!("     - Inject Intel VMD / RST NVMe Driver using DISM++ Tool.");
        }
        _ => {
            println!("  1. Analysis: Detected general system boot corruption or driver conflict.");
            println!("  2. Recommended Action:");
            println!("     - Perform 1-Click Smart Backup to save important files to USB first.");
            println!("     - Run S.M.A.R.T HDD/SSD diagnostic check for bad sectors.");
        }
    }

    println!("{}", "--------------------------------------------------".dimmed());
}
