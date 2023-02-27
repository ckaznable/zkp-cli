use clap::Parser;

#[derive(Parser)]
#[command(author, version)]
pub struct Args {
    /// make host
    #[arg(long)]
    pub host: bool,

    /// target host ip
    #[arg(short, long, default_value=&"")]
    pub connect: String,
}