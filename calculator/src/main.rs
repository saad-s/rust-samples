use clap::{Parser, Subcommand};
/// basic calculator 
#[derive(Parser)]
#[clap(about, version, author)]
#[clap(propagate_version = true)]

struct Args {
    #[clap(subcommand)]
    operation: Operations,
}

#[derive(Subcommand)]
enum Operations {
    /// add two numbers 
    Add { number_one: f64, number_two: f64},
    /// subtract second number from first 
    Sub { number_one: f64, number_two: f64},
    /// multiply two numbers 
    Mul { number_one: f64, number_two: f64},
    /// divide first number by second 
    Div { number_one: f64, number_two: f64},
    /// calculate mod of first number by second 
    Mod { number_one: f64, number_two: f64},
}

fn main() {
    let args = Args::parse();
    match &args.operation {
        Operations::Add { number_one, number_two } => {
            println!("{} + {} = {}", number_one, number_two, (number_one + number_two) );
        }
        Operations::Sub { number_one, number_two } => {
            println!("{} - {} = {}", number_one, number_two, (number_one - number_two));
        }
        Operations::Mul { number_one, number_two } => {
            println!("{} x {} = {}", number_one, number_two, (number_one * number_two) );
        }
        Operations::Div { number_one, number_two } => {
            println!("{} / {} = {}", number_one, number_two, (number_one / number_two) );
        }
        Operations::Mod { number_one, number_two } => {
            println!("{} % {} = {}", number_one, number_two, (number_one % number_two));
        }
    }
}
