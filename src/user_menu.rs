use std::io::Write;

pub fn user_menu() -> String {

    let mut mode: String = String::new();

    print!("Select mode:\n  1. Commit and push\n\nMode: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut mode).unwrap();

    mode
}