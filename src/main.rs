use std::process::Command;

fn main() {
    let status = Command::new("ping").arg("-c 3").arg("8.8.8.8").status().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    println!("process exited with: {}", status);
    }

fn get_addr() {
    }
