fn main() {
    'outer: for c in 1..1000 {
        for b in 1..1000 {
            if b > c { continue }

            for a in 1..1000 {
                if a > b { continue }
                if a * a + b * b != c*c { continue }
                if a + b + c != 1000 { continue }

                println!("Result: {}", a * b * c);
                break 'outer
            }
        }
    }
}
