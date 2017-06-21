// Reimplementation of C++ code
// From: https://stackoverflow.com/questions/44441627/how-to-optimize-this-haskell-code-summing-up-the-primes
// C++ code: https://gist.github.com/AdamStelmaszczyk/a6661e53e34bbf8b10af0fa5a5e97395

use std::collections::HashMap;

fn main() {
    let n = 2000000;
    let r = f64::sqrt(n as f64) as usize;
    let r2 = r * 2;
    let mut v : Vec<usize> = Vec::with_capacity(r2);
    for i in 1..r + 1 {
        v.push(n / i);
    }
    for i in (1.. r).rev() {
        v.push(i);
    }

    let mut s : HashMap<usize, usize> = HashMap::with_capacity(r2);
    for x in &v {
        s.insert(*x, x * (x + 1) / 2 - 1);
    }

    for p in 2 .. r + 1 {
        if s.get(&p).unwrap() > s.get(&(p - 1)).unwrap() {
            for i in 0 .. v.len() - 1 {
                let v = v[i];
                if v < p * p {
                    break;
                }
                let new = {
                    let old = s.get(&v).unwrap();
                    let vp = s.get(&(v / p)).unwrap();
                    let p1 = s.get(&(p - 1)).unwrap();
                    old - (p * (vp - p1))
                };
                s.insert(v, new);
            }
        }
    }
    println!("sum({}) = {}", n, s.get(&n).unwrap());
}
