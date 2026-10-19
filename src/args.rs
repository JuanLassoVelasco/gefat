use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub count: u8,
}
