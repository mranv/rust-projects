use std::process::Command;

fn main() {
    let brew_installed = check_brew_installed();
    if brew_installed {
        println!("Homebrew is installed.");
    } else {
        println!("Homebrew is not installed.");
    }
}

fn check_brew_installed() -> bool {
    let output = Command::new("brew")
        .arg("--version")
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute 'brew' command"));

    output.status.success()
}