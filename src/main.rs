use clap::Parser;
use gefat::args::Args;

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hey {}", args.name);
    }
}
