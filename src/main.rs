use anyhow::Result;
use qalam::Qalam;
use clap::Parser;
use qalam::args::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut qalam = Qalam::init();
    return qalam.run(args);
}
