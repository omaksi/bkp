pub fn run_script(script: &str) -> () {
    if script == "" {
        println!("No script to run");
        return;
    }

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .expect("failed to execute process");

    match output.status.success() {
        true => {
            println!("Script {} ran successfully", script);
        }
        false => {
            println!("Script {} failed", script);
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
