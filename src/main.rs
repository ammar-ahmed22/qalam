use std::env;
use anyhow::{Result, Context};
use std::io::{self, Write};

fn run_file(path: &String) -> Result<()> {
    let file_content = std::fs::read_to_string(path)
        .with_context(|| format!("Cannot read file"))?;
    print!("{}", file_content);
    return Ok(());
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
                if input == "exit()" {
                    break;
                }
                println!("{}",input);
            },
            Err(err) => {
                eprintln!("Error reading input: {}", err);
            }
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: qalam [script]");
        std::process::exit(1);
    } else if args.len() == 2 {
        run_file(&args[1])?
    } else {
        run_prompt();
    }

    return Ok(())
}
