use std::process::Command;

fn main() {
    let command = Command::new("bash")
        .arg("-c")
        .arg("echo Hello from Rust")
        .output()
        .expect("Failed to execute command");

    if command.status.success() {
        let output = String::from_utf8_lossy(&command.stdout);
        println!("Command output: {}", output);
    } else {
        let error = String::from_utf8_lossy(&command.stderr);
        eprintln!("Command failed: {}", error);
    }
}