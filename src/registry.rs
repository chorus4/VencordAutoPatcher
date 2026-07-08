use windows_registry::{CURRENT_USER};
use std::{path::PathBuf};

pub fn add_autostart_registry(file: &PathBuf) {
    let key = CURRENT_USER.create("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run").expect("Failed to open registry");
    let command = format!("{} patch", file.display());

    key.set_string("VencordAutoPatcher", &command).expect("Failed to create registry value");
}

pub fn remove_autostart_registry() {
    let key = CURRENT_USER.create("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run").expect("Failed to open registry");

    key.remove_value("VencordAutoPatcher").expect("Failed to delet registry value");
}