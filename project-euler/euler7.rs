fn main() {
    let mut primes = std::vec::Vec::new();
    let mut x = 1;

    loop {
        x += 1;
        let mut is_prime = true;
        for prime in &primes {
            if x % prime == 0 {
                is_prime = false;
                break
            }
        }
        if is_prime {
            if primes.len() == 10000 {
                println!("prime: {}", x);
                break
            } else {
                primes.push(x)
            }
        }
    }
}
