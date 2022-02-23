use clap::Parser;

mod sieve;
/// calculate prime numbers up to a given number
#[derive(Parser)]
#[clap(version, about, author)]

struct Args {
    /// max number for prime range, should be greater than 1
    #[clap(short, long)]
    range: usize,
} 

fn main () {
    let args = Args::parse();
    println!("\ncalculating primes up to {}.
using sieve of eratosthenes...", args.range);
    sieve::eratosthenes(args.range);
}