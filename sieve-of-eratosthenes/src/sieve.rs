// Sieve of Eratosthenes [calculate primes]

// input: an integer n > 1.
// output: all prime numbers from 2 through n.

// let A be an array of Boolean values, indexed by integers 2 to n,
// initially all set to true.

// for i = 2, 3, 4, ..., not exceeding √n do
//     if A[i] is true
//         for j = i2, i2+i, i2+2i, i2+3i, ..., not exceeding n do
//             A[j] := false
// return all i such that A[i] is true.

pub fn eratosthenes (range: usize) {
    let upper_limit = f64::sqrt(range as f64) as usize;
    let mut numbers = vec![true; range];

    if range < 2 {
        println!("invalid range, please try numbers larger than 1!!");
    }
    else {
        // 0 and 1 are not primes 
        numbers[0] = false;
        numbers[1] = false;

        // mark all composites as false
        for x in 2..upper_limit+1 {
            if numbers[x] == true {
                let mut mul = x;
                loop {
                    if x * mul < range {
                        numbers[x * mul] = false; 
                        mul += 1;
                    }
                    else {
                        break;
                    }
                }
            } else {
                continue;
            }
        }

        // now print primes 
        let mut total_primes = 0;
        println!();
        print!("[ ");
        // io::stdout().flush().unwrap();
        for x in 0..range {
            if numbers[x] {
                print!("{} ", x);
                total_primes += 1;
            }
        }
        println!("]"); println!();
        // will a /n/n work instead. on all systems?
        println!("there are {} primes between 0 and {}.", total_primes, range);
    }
}
