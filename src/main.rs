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

fn directory_walk(line: &str) -> Vec<String> {
    let mut retvec = Vec::<String>::new();
    let iter = std::fs::read_dir(line).unwrap();
    for direntry in iter {
        let entry = direntry.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file() {
            if entry.file_name() == "pyvenv.cfg" {
                retvec.push(String::from(entry.path().to_str().unwrap()));
            }
        }

        if file_type.is_dir() {
            retvec.extend(directory_walk(entry.path().to_str().unwrap().clone()));
        }
    }
    retvec
}

fn main() {
    let _ = match std::env::consts::FAMILY {
        "unix" => {
            let home_dir = std::env::var("HOME").unwrap();
            let shell = get_shell();
            println!("Current Shell : {}", text_bold(&shell));
            println!("");

            let suffix = if shell.contains("fish") { ".fish" } else { "" };

            let search_dirs =
                std::fs::read_to_string(home_dir.clone() + "/.config/pyvenvselect/searchdirs")
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
                    Ok(choice_string) => match choice_string.parse::<u32>() {
                        Ok(ch) => {
                            let choice = ch as usize;

                            if choice > 0 && choice <= all_environments.len() {
                                let command = all_environments[(choice - 1) as usize].clone();

                                println!(
                                    "Use the command {} to selected activate Virtual Env.",
                                    text_bold("source $PYVENV_CURRENT")
                                );
                                let activation_file = format!("{}{suffix}", command);
                                // let contents = std::fs::read_to_string(activation_file)
                                //      .unwrap_or(String::new());
                                let contents = format!("source {activation_file}");
                                std::fs::write(
                                    home_dir + "/.config/pyvenvselect/current",
                                    contents,
                                )
                                .expect("Unable to write command to config directory.");
                                println!("");
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
                    },
                    Err(_) => {
                        println!("Couldn't read input.");
                        break;
                    }
                }
            }
            // TODO: Using the shell, activate
            // the env file
        }
        "windows" => {
            let home_dir =
                std::env::var("HOMEDRIVE").unwrap() + &std::env::var("HOMEPATH").unwrap();
            let file_name = format!("{home_dir}\\.config\\pyvenvselect\\searchdirs");
            let fl = std::fs::read_to_string(file_name).unwrap();

            for line in fl.lines() {
                let env_dirs = directory_walk(line);
                for env_dir in env_dirs {
                    println!(
                        "{}Scripts\\activate",
                        &env_dir[0..env_dir.len() - "pyvenv.cfg".len()]
                    );
                }
            }
        }
        _ => todo!(),
    };
}
