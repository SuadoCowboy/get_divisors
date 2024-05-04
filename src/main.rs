use std::io::Write;

mod get_divisors;

fn main() {
    let mut input = String::new();

    print!("Pick a number: ");
    std::io::stdout().flush().expect("Could not flush stdout for some reason.");

    std::io::stdin().read_line(&mut input).expect("Something went wrong while reading user input.");

    let num: usize = input.trim().parse().expect("Could not convert String to usize type.");
    
    let mut divisors = String::new();
    get_divisors::get_divisors(num).iter().for_each(|i| { divisors += i.to_string().as_str(); divisors += " "; });
    
    println!("The divisors of {num} are: {divisors}");
}
