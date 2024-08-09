use std::env;
use anyhow::Result;
use qalam::Qalam;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut qalam = Qalam::init();
    return qalam.run(args);
}
