fn main() {
    let mut num : u32 = 0;

    'outer: loop {
        num += 1;
        for x in 1..20 {
            if num % x != 0 {
                continue 'outer
            }
        }
        break
    }

    println!("Result: {}", num)
}
