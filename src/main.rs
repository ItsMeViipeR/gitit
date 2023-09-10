use std::io::Write;


fn get_user_branch() -> String {
    let mut branch: String = String::new();

    print!("Enter branch name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut branch).unwrap();

    branch = branch.trim().to_string();

    if branch.is_empty() {
        branch = "master".to_string();
    }

    branch
}

fn create_branch(branch: &str) {
    let output = std::process::Command::new("git")
        .args(&["branch", "-M", branch])
        .output()
        .expect("Failed to create branch");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn create_commit() {
    let mut commit_msg: String = String::new();

    print!("Enter commit message: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut commit_msg).unwrap();

    let output = std::process::Command::new("git")
        .args(&["commit", "-m", commit_msg.as_str()])
        .output()
        .expect("Failed to create commit");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn remote_add(repo: &str) {
    let output = std::process::Command::new("git")
        .args(&["remote", "add", "origin", repo])
        .output()
        .expect("Failed to add remote");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn push(branch: &str) {
    let output = std::process::Command::new("git")
        .args(&["push", "-u", "origin", branch])
        .output()
        .expect("Failed to push");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn add_all() {
    let output = std::process::Command::new("git")
        .args(&["add", "."])
        .output()
        .expect("Failed to add all");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn commit_and_push() {
    let branch: String = get_user_branch();
    
    add_all();

    create_commit();

    create_branch(&branch);

    let mut repo: String = String::new();

    print!("Enter repo: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut repo).unwrap();

    if !repo.contains(".git") {
        repo = format!("{}.git", repo.trim());
    }

    remote_add(&repo);

    push(&branch);
}

fn main() {
    commit_and_push();
}