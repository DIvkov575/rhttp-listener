use clap::Parser;
mod lib;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Receiving url or ip
    // Include Port with ip and format *.*.*.*:*
    #[arg(short, long)]
    url: String,
    #[arg(short, long)]
    file: String,

}

fn main() {
    let args = Args::parse();
    lib::send(args.url, args.file).unwrap();
}