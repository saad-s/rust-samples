use clap::Parser;
mod fizzbuzz;

/// print fizz buzz up to a user input number  
#[derive(Parser)]
#[clap(version, about, author)]

struct Args {
    /// number range 
    #[clap(short, long)]
    range: u32,
} 

fn main () {
    let args = Args::parse();
    fizzbuzz::fizzbuzz(args.range);
}