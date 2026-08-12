use colored::*;
use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn launch_native_desktop_window() {
    println!("{}", "\n🖥️ --- [NANTARA NATIVE DESKTOP WINDOW ENGINE v2.0] ---".cyan().bold());

    // Locate HTML UI entry point
    let mut web_path = PathBuf::from("web/index.html");

    if !web_path.exists() {
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let candidate = exe_dir.join("../../../web/index.html");
                if candidate.exists() {
                    web_path = candidate;
                } else {
                    let candidate2 = exe_dir.join("../../web/index.html");
                    if candidate2.exists() {
                        web_path = candidate2;
                    }
                }
            }
        }
    }

    if web_path.exists() {
        let abs_path = web_path.canonicalize().unwrap_or(web_path.clone());
        let file_url = format!("file:///{}", abs_path.to_str().unwrap_or("").replace("\\", "/"));

        println!("  -> Target UI Path : {}", file_url.green());
        println!("  -> Window Mode   : Standalone App Window (No Browser Bar)");

        // Launch in Standalone App Window mode via msedge --app or chrome --app
        let _status = Command::new("cmd")
            .args([
                "/c",
                "start",
                "msedge",
                &format!("--app={}", file_url),
                "--window-size=1280,800",
                "--title=Nantara-Boot Rescue Environment"
            ])
            .spawn();

        println!("{}", "  ✅ Native Desktop Window Launched Successfully!".green().bold());
    } else {
        println!("{}", "  ⚠️ Warning: Web UI assets (web/index.html) not found.".yellow());
    }

    println!("{}", "--------------------------------------------------".dimmed());
}
