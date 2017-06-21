fn main() {
    let mut sum1 : i64 = 0;
    let mut sum2 = 0;

    for x in 1..101 {
        sum1 += x * x;
        sum2 += x;
    }

    sum2 *= sum2;

    println!("Sum of squares: {}", sum1);
    println!("Square of sums: {}", sum2);
    println!("Result: {}", sum2 - sum1)
}
