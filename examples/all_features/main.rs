use std::process::Command;

static EXAMPLE_NAME: &str = "all_features";

fn main () {
  let status = Command::new("cargo")
    .arg("run")
    .arg("--")
    .arg(format!("./examples/{}/main.qlm", EXAMPLE_NAME))
    .status()
    .expect("Failed to execute process");

  if !status.success() {
    eprintln!("Command failed with status: {:?}", status)
  }
}