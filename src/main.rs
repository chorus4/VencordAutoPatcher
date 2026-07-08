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