use std::process::Command;

fn main () {
  let status = Command::new("cargo")
    .arg("run")
    .arg("--")
    .arg("./examples/variable_assignment/main.qlm")
    .status()
    .expect("Failed to execute process");

  if !status.success() {
    eprintln!("Command failed with status: {:?}", status)
  }
}