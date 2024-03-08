use std::process::Command;
use std::env;
use std::process::ExitStatus;

fn main() {
    dotenv::dotenv().ok();
    
    let mode = match env::var("MODE") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("MODE is not set");
            return;
        }
    };

    // Print a message to stderr
    Command::new("echo")
        .arg("db_url.sh: script started")
        .status()
        .expect("failed to run echo");

    let status :ExitStatus;

    if mode == "dev" {
        status = Command::new("bash")
            .arg("scripts/db_url.sh")  
            .status()
            .expect("Failed to execute Bash script");

        // Check if the script executed successfully
        if !status.success() {
            panic!("Failed to run db_url.sh");
        }

        if status.success() {
            eprintln!("retrieved db url");
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}
