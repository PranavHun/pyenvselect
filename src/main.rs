use std::process::Command;
fn main() {
    let _ = match std::env::consts::FAMILY {
        "unix" => {
            let currently_running_processes = String::from_utf8(
                Command::new("ps")
                    .args(["-o", "cmd"])
                    .output()
                    .expect("ps command didn't run")
                    .stdout,
            )
            .expect("ps command didn't produce and output.");
            let mut shell = "";
            for line in currently_running_processes.split("\n") {
                if line.contains("ps -o cmd") {
                    break;
                } else {
                    if line.contains("pyenvselect") {
                        continue;
                    }
                    shell = line;
                }
            }

            println!("Shell is : {shell}");
            // TODO: Using the shell, activate
            // the env file
        }
        "windows" => todo!(),
        _ => todo!(),
    };
}
