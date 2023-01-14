use std::process::exit;

use log::{error, info};

pub fn run_script(script: &str) -> () {
    if script == "" {
        error!("No script to run");
        return;
    }

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .expect("failed to execute process");

    match output.status.success() {
        true => {
            info!("Script {} ran successfully", script);
        }
        false => {
            error!("Script {} failed", script);
            error!("{}", String::from_utf8_lossy(&output.stderr));
            exit(1);
        }
    }
}
