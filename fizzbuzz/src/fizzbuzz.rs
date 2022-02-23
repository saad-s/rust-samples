
pub fn fizzbuzz(max_number: u32) {
    let mut number = 0;
    while number < max_number  {
        if number % 15 == 0 {
            println!("fizzbuzz");
        } else if number % 5 == 0 {
            println!("buzz");
        } else if number % 3 == 0 {
            println!("fizz");
        }
        else {
            println!("{}", number)
        }
        number += 1;
    }
}