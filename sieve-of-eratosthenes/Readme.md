### Calculate primes

calculate primes between 0 and given number, using [sieve of eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).

### Usage

Clone repo and run `cargo build --release`

```bash
cargo run -- --range 100    # using cargo
cargo run -- -r 1000
./target/release/sieve-of-eratosthenes -r 100   # using release binary
./target/release/sieve-of-eratosthenes --range 100
```
