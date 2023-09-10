pub mod git_commands;
pub mod user_menu;

use crate::user_menu::user_menu;

fn main() {
    // clear the screen
    print!("\x1B[2J\x1B[1;1H");

    loop {
        let selected_mode: Result<i32, std::num::ParseIntError> = user_menu().trim().parse();

        match selected_mode {
            Ok(mode) => {
                match mode {
                    1 => {
                        print!("\x1B[2J\x1B[1;1H");
                        git_commands::commit_and_push();
                        break;
                    }
                    2.. => {
                        println!("Not implemented yet");
                    }
                    _ => {
                        println!("Invalid option");
                    }
                }
            }
            Err(_) => {
                println!("Invalid option. You must enter a number.");
            }
        };

        std::thread::sleep(std::time::Duration::from_secs(1));
        print!("\x1B[2J\x1B[1;1H");
    }
}