### cli calculator using rust
calculator with basic maths operations, add, subtract, multiply, divide and mod.

Sample uses clap for argument parsing. 
```rust
struct Args {
    #[clap(subcommand)]
    operation: Operations,
}
```

### usage
```bash 
cargo run -- mod 2 36 ##using cargo 
cargo run -- add 2 36 ##using cargo 
calculator mod 21398.1 1983 ## using release binary
calculator add 21398.1 1983 ## using release binary
```

