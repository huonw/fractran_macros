#![feature(plugin)]

#![plugin(fractran_macros)]

fn main() {
    // Conway's prime enumeration program. The subsequence of perfect
    // powers of two (other than the initial 2) is 4 = 2^2, 8 = 2^3,
    // 32 = 2^5, etc. specifically, it enumerates the primes.

    let mut primes = fractran!(17/91, 78/85, 19/51, 23/38, 29/33, 77/29, 95/23, 77/19, 1/17,
                               11/13, 13/11, 15/14, 15/2, 55/1)(&[1]);

    let mut count = 0;

    loop {
        match primes.next() {
            None => break,
            Some(_) => {}
        }

        if count == 1_000_000_000 { break }

        if primes.state()[1..].iter().all(|x| *x == 0) {
            println!("{} (step {})", primes.state()[0], count)
        }

        count += 1;
    }
}
