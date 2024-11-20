use clap::Parser;

#[derive(Parser)]
#[command(
    name = "qalam", 
    about="An interpreter for the Qalam language",
    version=env!("CARGO_PKG_VERSION")
)]
pub struct Args {
    /// File path to interpret and execute
    #[arg(value_name="FILE_PATH")]
    pub file_path: Option<String>,

    /// Raw string input to interpret and execute
    #[arg(long="raw", value_name="SOURCE")]
    pub raw: Option<String>
}