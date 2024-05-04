use elr_primes::Primes;

pub fn get_divisors(mut num: usize) -> Vec<usize> {
    if num == 0 { return vec![]; }

    let mut out: Vec<usize> = vec![1];

    let p = Primes::new(num);

    for prime in p.primes() {
        while num % prime == 0 {
            num /= prime;
            out.push(prime.clone());
        }
    }

    return out;
}