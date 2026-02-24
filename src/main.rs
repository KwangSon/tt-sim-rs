use clap::Parser;

#[derive(Parser)]
#[command()]
struct Cli {
    wormhole: u32,
    blackhole: u32,
}

fn main() {
    let cli = Cli::parse();
}
