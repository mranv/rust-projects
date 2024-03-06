use std::process::Command;

fn main() {
    let brew_installed = check_brew_installed();
    let (gatekeeper_status, xprotect_status) = check_gatekeeper_xprotect();

    println!("Homebrew is {}installed.", if brew_installed { "" } else { "not " });
    println!("GateKeeper is {}.", gatekeeper_status);
    println!("XProtect is {}.", xprotect_status);
}

fn check_brew_installed() -> bool {
    let output = Command::new("brew")
        .arg("--version")
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute 'brew' command"));

    output.status.success()
}

fn check_gatekeeper_xprotect() -> (String, String) {
    let output = Command::new("spctl")
        .arg("--status")
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute 'spctl' command"));

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();

    let gatekeeper_status = if lines.contains(&"assessments enabled") {
        "enabled".to_string()
    } else {
        "disabled".to_string()
    };

    let xprotect_status = if lines.contains(&"File Protections: enabled") {
        "enabled".to_string()
    } else {
        "disabled".to_string()
    };

    (gatekeeper_status, xprotect_status)
}