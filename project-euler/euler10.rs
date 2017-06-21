fn main() {
    let mut sum: u64 = 0;
    let mut primes = Vec::new();
    for x in 2..2000000 {
        let mut is_prime = true;

        for prime in &primes {
            if x % prime == 0 {
                is_prime = false;
                break
            } else if prime * prime > x {
                break
            }
        }

        if is_prime {
            println!("Prime: {}", x);
            primes.push(x);
            sum += x
        }
    }
    println!("Sum: {}", sum)
}
