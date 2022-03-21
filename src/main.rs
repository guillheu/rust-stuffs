mod cryptopals;

extern crate clap;

use clap::Parser;

/// Implementation of the Cryptopals sets of challenges.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Set to use
    #[clap(short, long)]
    set: u8,

    /// Challenge to use
    #[clap(short, long)]
    challenge: u8,

    /// Input of the function
    input: String,
}

fn main() {
    let args = Args::parse();
    match (args.set, args.challenge){
        (1, 1)  => cryptopals::set1::challenge_1(&args.input),
        _       => eprintln!("set/challenge not found")
    }
}