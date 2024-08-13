use std::process::Command;

fn main () {
  let status = Command::new("cargo")
    .arg("run")
    .arg("--")
    .arg("./examples/conditionals/main.qlm")
    .status()
    .expect("Failed to execute process");

  if !status.success() {
    eprintln!("Command failed with status: {:?}", status)
  }
}