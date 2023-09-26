use std::process::Command;

fn text_bold<T: std::fmt::Display>(text: T) -> String {
    format!("\x1b[37;1m{text}\x1b[0m")
}

fn get_environments(search_dir: &str) -> Vec<String> {
    let py_env_directories = String::from_utf8(
        Command::new("find")
            .args([search_dir, "-iname", "pyvenv.cfg"])
            .output()
            .expect("fd command didn't run")
            .stdout,
    )
    .expect("fd command didn't produce and output.");
    let mut environment_directories = Vec::<String>::new();
    for path in py_env_directories.split("\n") {
        if !path.is_empty() {
            let mut path_arr: Vec<&str> = path.split("/").collect();
            path_arr.pop();
            path_arr.push("bin");
            path_arr.push("activate");
            let environment_directory = path_arr.join("/");
            environment_directories.push(environment_directory);
        }
    }
    environment_directories
}

fn get_shell() -> String {
    let current_exe_path = std::env::current_exe().expect("Cannot read current executable name.");
    let current_exe = current_exe_path
        .components()
        .last()
        .expect("Cannot find executable name.")
        .as_os_str()
        .to_str()
        .expect("Cannot convert executable name to str.");

    let currently_running_processes = String::from_utf8(
        Command::new("ps")
            .args(["-o", "cmd"])
            .output()
            .expect("ps command didn't run")
            .stdout,
    )
    .expect("ps command didn't produce and output.");
    let mut shell = String::new();
    for line in currently_running_processes.split("\n") {
        if line.contains("ps -o cmd") {
            break;
        } else {
            if line.contains(current_exe) {
                continue;
            }
            shell = String::from(line);
        }
    }
    shell
}

fn main() {
    let _ = match std::env::consts::FAMILY {
        "unix" => {
            let shell = get_shell();
            println!("Current Shell : {}", text_bold(&shell));
            println!("");

            let suffix = if shell.contains("fish") { ".fish" } else { "" };

            let search_dirs = std::fs::read_to_string(".config/pyvenvselect/searchdirs")
                .unwrap_or(String::from("not found"));
            let mut env_number = 1;

            for search_dir in search_dirs.split("\n") {
                if !search_dir.is_empty() {
                    let environments = get_environments(search_dir);
                    println!("Environments at : {}", text_bold(search_dir));
                    for env in environments {
                        println!("[{env_number}] - {env}{suffix}");
                        env_number += 1;
                    }
                    println!();
                }
            }
            // TODO: Using the shell, activate
            // the env file
        }
        "windows" => todo!(),
        _ => todo!(),
    };
}
