fn main() {
    let mut num : u64 = 600851475143;
    let mut x = 2;
    loop {
        if x == num {
            break
        } else if num % x == 0 {
            num /= x
        } else {
            x += 1
        }
    }
    println!("Result: {}", num)
}
