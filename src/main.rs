use anyhow::Result;
use clap::Parser;
use qalam::args::Args;
use qalam::Qalam;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut qalam = Qalam::init();
    return qalam.run(args);
}
