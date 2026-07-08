use std::fmt::format;
use std::{io, path::PathBuf};
use std::io::Write;
use std::fs::File;
use std::fs;
use std::process::Command;
use windows_registry::{CURRENT_USER};

fn main() {
    let arg = std::env::args().nth(1);
    match arg {
        Some(arg) => {
            if arg == "patch" {
                patch_discord();
            }
        },
        None => {
            main_menu();
        },
    }
}

fn main_menu() {
    println!("VencordAutoPatcher");
    println!("");
    println!(">1 Add script to autostart");
    println!(">2 Delete script from autostart");
    print!("Select option: ");
    io::stdout().flush().expect("Some troubles");

    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Please type valid value");

    let option: u32 = option.trim().parse().expect("Please type valid value");

    if option == 1 {
        install();
    } else if option == 2 {
        uninstall();
    }
}

fn install() {
    println!("Downloading VencordInstallerCli.exe");

    let mut appdata = dirs::config_dir().expect("Failed to get appdata dir");
    appdata.push("VencordAutoPatcher");
    fs::create_dir(&appdata).expect("Failed to create folder VencordAutoPatcher");

    let url = "https://github.com/Vencord/Installer/releases/latest/download/VencordInstallerCli.exe";

    let mut cli_path = appdata.clone();
    cli_path.push("VencordInstallerCli.exe");

    let mut resp = reqwest::blocking::get(url).expect("request failed");
    let mut out = File::create(cli_path).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");

    println!("Cloning VencordAutoPatcher.exe");
    let file = std::env::current_exe().expect("Failed to get executable file");

    let mut installer_path = appdata.clone();
    installer_path.push("VencordAutoPatcher.exe");

    let mut installer_old_file = File::open(file).expect("failed to create file");
    let mut installer_file = File::create(&installer_path).expect("failed to create file");

    io::copy(&mut installer_old_file, &mut installer_file).expect("failed to copy content");

    add_autostart_registry(&installer_path);
}

fn uninstall() {
    println!("Deleting VencordAutoPatcher folder");

    let mut appdata = dirs::config_dir().expect("Failed to get appdata dir");
    appdata.push("VencordAutoPatcher");

    fs::remove_dir_all(appdata).expect("Failed to remove dir");
    remove_autostart_registry();
}

fn add_autostart_registry(file: &PathBuf) {
    let key = CURRENT_USER.create("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run").expect("Failed to open registry");
    let command = format!("{} patch", file.display());

    key.set_string("VencordAutoPatcher", &command).expect("Failed to create registry value");
}

fn remove_autostart_registry() {
    let key = CURRENT_USER.create("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run").expect("Failed to open registry");

    key.remove_value("VencordAutoPatcher").expect("Failed to delet registry value");
}

fn patch_discord() {
    let mut file = std::env::current_exe()
        .expect("Failed to get executable file")
        .parent()
        .expect("Failed to get executable folder")
        .to_path_buf();
    file.push("VencordInstallerCli.exe");

    let startup_str = format!("{} -repair -branch auto", file.display());

    Command::new("cmd")
        .args(["/C", &startup_str])
        .output()
        .expect("Failed to spawn command");

    Command::new("cmd")
        .args(["/C", "start", "", "discord://"])
        .output()
        .expect("Failed to start discord");
}