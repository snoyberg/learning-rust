fn main() {
    let mut largest = 0;

    for x in 100..999 {
        for y in 100..999 {
            let product = x * y;
            if product > largest && is_palindrome(product) {
                largest = product
            }
        }
    }
    println!("Result: {}", largest)
}

fn is_palindrome(x : u32) -> bool {
    let str = format!("{}", x);
    let rev = str.chars().rev().collect::<String>();
    str == rev
}
