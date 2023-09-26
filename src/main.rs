use std::io::prelude::*;
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

            let search_dirs = std::fs::read_to_string("~/.config/pyvenvselect/searchdirs")
                .unwrap_or(String::from("not found"));
            let mut env_number = 1;
            let mut all_environments = Vec::<String>::new();

            for search_dir in search_dirs.split("\n") {
                if !search_dir.is_empty() {
                    let mut environments = get_environments(search_dir);

                    println!("Environments at : {}", text_bold(search_dir));
                    for env in environments.clone() {
                        println!("[{env_number}] - {env}{suffix}");
                        env_number += 1;
                    }
                    all_environments.append(&mut environments);
                    println!();
                }
            }
            let stdin = std::io::stdin();
            println!("Choose the Virtual Environment you want to activate");
            println!("[1 to {}] :", all_environments.len());

            for line in stdin.lock().lines() {
                match line {
                    Ok(choice_string) => {
                        match choice_string.parse::<u32>() {
                            Ok(ch) => {
                                let choice = ch as usize;

                                if choice > 0 && choice <= all_environments.len() {
                                    let command = all_environments[(choice - 1) as usize].clone();

                                    println!("Use the following command to selected activate Virtual Env.");
                                    println!("source {}{suffix}", command);
                                } else {
                                    eprintln!(
                                        "Acceptable inputs : 1-{}. Please try again",
                                        all_environments.len()
                                    );
                                    println!("Choose the Virtual Environment you want to activate");
                                    println!("[1 to {}] :", all_environments.len());
                                    continue;
                                }
                                break;
                            }
                            Err(_) => {
                                eprintln!("Please provide appropriate input.");
                                println!("Choose the Virtual Environment you want to activate");
                                println!("[1 to {}] :", all_environments.len());
                                continue;
                            }
                        }
                    }
                    Err(_) => {
                        println!("Couldn't read input.");
                        break;
                    }
                }
            }
            // TODO: Using the shell, activate
            // the env file
        }
        "windows" => todo!(),
        _ => todo!(),
    };
}
