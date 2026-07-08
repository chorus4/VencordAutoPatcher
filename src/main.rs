use std::{io};
use std::io::Write;

mod installer;
mod patcher;
mod registry;

fn main() {
    let arg = std::env::args().nth(1);
    match arg {
        Some(arg) => {
            if arg == "patch" {
                patcher::patch_discord();
            }
        },
        None => {
            main_menu();
        },
    }
}

fn main_menu() {
    let mut appdata = dirs::config_dir().expect("Failed to get appdata dir");
    appdata.push("VencordAutoPatcher");

    println!("VencordAutoPatcher");
    println!("");
    if appdata.is_dir() {
        println!("Script is already in autostart");
    }
    println!(">1 Add script to autostart");
    println!(">2 Delete script from autostart");
    print!("Select option: ");
    io::stdout().flush().expect("Some troubles");

    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Please type valid value");

    let option: u32 = option.trim().parse().expect("Please type valid value");

    println!("");
    if option == 1 {
        if appdata.is_dir() {
            installer::uninstall();
            println!("");
        }
        installer::install();
    } else if option == 2 {
        installer::uninstall();
    }

    println!("");
    println!("Press Enter to exit...");

    let mut qqq = String::new();
    io::stdin()
        .read_line(&mut qqq)
        .expect("Please type valid value");
}