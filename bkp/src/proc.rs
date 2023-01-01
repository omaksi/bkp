pub fn run_script(script: &str) -> () {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
