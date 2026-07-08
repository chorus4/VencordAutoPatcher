use std::process::Command;

pub fn patch_discord() {
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