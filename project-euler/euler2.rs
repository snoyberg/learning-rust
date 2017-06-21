fn main() {
    let mut total = 0;
    let mut fib = 1;
    let mut next_fib = 2;

    while fib <= 4000000 {
        if fib % 2 == 0 {
            total += fib
        }
        let tmp = fib + next_fib;
        fib = next_fib;
        next_fib = tmp;
    }
    println!("Total: {}", total)
}
