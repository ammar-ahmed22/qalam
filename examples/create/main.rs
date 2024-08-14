use std::env;
use std::io;
use std::path::Path;
use std::fs::File;
use std::io::Write;

fn main () -> io::Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    eprintln!("Usage: cargo run --example create <example_name>");
    std::process::exit(1);
  }

  let example_name = &args[1];
  let examples_dir_path = Path::new("./examples");

  // Check if example already exists
  for entry in std::fs::read_dir(&examples_dir_path)? {
    let entry = entry?;
    let path = entry.path();

    if let Some(last_part) = path.file_stem() {
      if last_part.to_string_lossy() == *example_name {
        eprintln!("Example '{}' already exists!", example_name);
        std::process::exit(1);
      }
    }
  }

  // println!("Creating example '{}'...", example_name);
  let new_dir_path = examples_dir_path.join(example_name);
  std::fs::create_dir(new_dir_path.clone())?;

  let template_file = std::fs::read_to_string("./examples/create/template.rs")?;
  let updated_file = template_file.replace("EXAMPLE_NAME_STR", &example_name);
  let main_path = new_dir_path.join("main.rs");

  let mut main_file = File::create(main_path)?;
  main_file.write_all(updated_file.as_bytes())?;
  File::create(new_dir_path.join("main.qlm"))?;

  println!("Created example '{}'", example_name);
  println!("Update '{}' with the example Qalam code.", new_dir_path.join("main.qlm").display());
  println!("Run the example with 'cargo run --example {}'", example_name);
  return Ok(());
}