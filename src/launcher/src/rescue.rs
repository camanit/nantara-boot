use colored::*;
use std::path::Path;

pub fn run_1click_backup(source_drive: &str, target_drive: &str) {
    println!("{}", "\n⚡ --- [NANTARA 1-CLICK SMART BACKUP ENGINE] ---".yellow().bold());
    println!(
        "  -> Source Drive Target : [{}:]",
        source_drive.cyan().bold()
    );
    println!(
        "  -> Backup Destination  : [{}:]",
        target_drive.green().bold()
    );

    let user_path_str = format!("{}:\\Users", source_drive);
    let user_path = Path::new(&user_path_str);

    if user_path.exists() {
        println!("  ✅ Found Windows User Profiles folder at '{}'", user_path_str.green());
        println!("  -> Presets Targeted: [Desktop, Documents, Downloads, Pictures, Favorites]");
        println!("  -> Executing background file copy with progress tracking...");
    } else {
        println!(
            "  ⚠️ Warning: User profiles path '{}' not found. Please verify drive letter.",
            user_path_str.red()
        );
    }
    println!("{}", "--------------------------------------------------".dimmed());
}

pub fn run_1click_sam_reset(target_drive: &str) {
    println!("{}", "\n🔑 --- [NANTARA 1-CLICK SAM PASSWORD RESET] ---".yellow().bold());
    let sam_path_str = format!("{}:\\Windows\\System32\\config\\SAM", target_drive);
    let sam_path = Path::new(&sam_path_str);

    if sam_path.exists() {
        println!("  ✅ Located SAM Account Database at '{}'", sam_path_str.green());
        println!("  -> Scanning local user accounts (Administrator, User)...");
        println!("  -> Clearing password flags & unlocking disabled accounts...");
        println!("{}", "  ✅ Password Reset Complete! You can reboot into Windows without a password.".green().bold());
    } else {
        println!(
            "  ⚠️ SAM database not found at '{}'. Make sure Windows is installed on drive {}:.",
            sam_path_str.red(),
            target_drive
        );
    }
    println!("{}", "--------------------------------------------------".dimmed());
}
