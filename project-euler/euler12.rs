fn divisors(x: u64) -> u64 {
    let mut count = 2;
    let mut i = 1;
    loop {
        i += 1;
        let i2 = i * i;

        if i2 > x {
            return count
        } else if i2 == x {
            return count + 1
        } else if x % i == 0 {
            count += 2
        }
    }
}

fn main() {
    let mut num = 0;
    let mut i = 0;
    loop {
        i += 1;
        num += i;
        println!("i: {}, num: {}, divisors: {}", i, num, divisors(num));
        if divisors(num) > 500 {
            println!("{}", num);
            break
        }
    }
}
