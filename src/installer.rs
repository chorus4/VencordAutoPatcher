use std::{io};
use std::fs;
use std::fs::File;

use crate::registry;

pub fn install() {
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

    println!("Adding autostart to registry");
    registry::add_autostart_registry(&installer_path);
}

pub fn uninstall() {
    println!("Deleting VencordAutoPatcher folder");

    let mut appdata = dirs::config_dir().expect("Failed to get appdata dir");
    appdata.push("VencordAutoPatcher");

    fs::remove_dir_all(appdata).expect("Failed to remove dir");
    
    println!("Deleting autostart from registry");
    registry::remove_autostart_registry();
}