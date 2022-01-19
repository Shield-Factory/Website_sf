use std::process::{Command, Stdio};


fn generation() -> String {
    let mut generate_private = Command::new("wg")
    .current_dir("/home/kali/Desktop")
    .arg("genkey")
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to spawn child process");


    let mut private_key = Command::new("cat")
    .current_dir("/home/kali/Desktop")
    .stdin(generate_private.stdout.unwrap())
    .output()
    .expect("Failed to execute command");


    println!("Status: {}", private_key.status);
    println!("Stdout: {}", String::from_utf8_lossy(&private_key.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&private_key.stderr));

    assert!(private_key.status.success());

    let my_private_key = String::from_utf8_lossy(&private_key.stdout);

    return my_private_key.to_string();

}

fn main() {
    generation();
}