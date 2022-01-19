#![allow(unused)]    
use std::process::{Command, Stdio};


fn main() {
    let mut generate_private = Command::new("wg")
    .current_dir("/home/kali/Desktop")
    .arg("genkey")
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to spawn child process");


    let mut private_key = Command::new("tee")
    .current_dir("/home/kali/Desktop")
    .arg("private.key")
    .stdin(generate_private.stdout.unwrap())
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to execute command");

    let mut generate_public = Command::new("wg")
    .current_dir("/home/kali/Desktop")
    .arg("pubkey")
    .stdin(private_key.stdout.unwrap())
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to spawn child process");


    let mut public_key = Command::new("tee")
    .current_dir("/home/kali/Desktop")
    .arg("public.key")
    .stdin(generate_public.stdout.unwrap())
    .output()
    .expect("Failed to execute command");

    let cli_private_key = Command::new("/bin/cat")
    .current_dir("/home/kali/Desktop")
    .arg("private.key")
    .output()
    .expect("Failed to execute command");

    println!("Status: {}", cli_private_key.status);
    println!("Stdout: {}", String::from_utf8_lossy(&cli_private_key.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&cli_private_key.stderr));

    assert!(cli_private_key.status.success());

    let cli_public_key = Command::new("/bin/cat")
    .current_dir("/home/kali/Desktop")
    .arg("public.key")
    .output()
    .expect("Failed to execute command");

    println!("Status: {}", cli_public_key.status);
    println!("Stdout: {}", String::from_utf8_lossy(&cli_public_key.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&cli_public_key.stderr));

    assert!(cli_public_key.status.success());


    let my_private_key = String::from_utf8_lossy(&cli_private_key.stdout);
    let my_public_key = String::from_utf8_lossy(&cli_public_key.stdout);

    println!("My private key is : {}", my_private_key);
    println!("My public key is : {}", my_public_key);

}