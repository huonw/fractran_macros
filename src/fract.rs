use std::cmp;
use num::integer;

use slow_primes::Primes;

#[deriving(Show)]
pub struct Fract<T> {
    pub numer: T,
    pub denom: T,
}

pub fn factorise(fracs: &[Fract<Vec<uint>>]) -> Vec<(Fract<u64>, Fract<Vec<u32>>)> {
    let largest = fracs.iter().map(|f| {
        f.numer.iter().map(|x| *x)
            .chain(f.denom.iter().map(|x| *x)).max().unwrap_or(1)
    }).max().unwrap_or(1);
    let primes = Primes::sieve(largest + 1);
    let mut prime_idx = Vec::from_elem(primes.upper_bound() + 1, 0);
    for (i, p) in primes.primes().enumerate() {
        *prime_idx.get_mut(p) = i;
    }

    let combine = |a: &mut Vec<u32>, b: &[(uint, u32)]| {
        for &(prime, count) in b.iter() {
            let l = a.len();
            let idx = prime_idx[prime];

            if idx >= l {
                a.grow(idx - l + 1, &0);
            }
            *a.get_mut(idx) += count
        }
    };

    let fac = |nums: &[uint]| -> (Vec<u32>, u64) {
        let mut ret = vec![];
        let mut prod = 1;
        for n in nums.iter() {
            prod *= *n as u64;
            combine(&mut ret, primes.factor(*n).as_slice());
        }
        (ret, prod)
    };
    let cancel = |a: &mut [u32], b: &mut [u32]| {
        for (x, y) in a.mut_iter().zip(b.mut_iter()) {
            let m = cmp::min(*x, *y);
            *x -= m;
            *y -= m;
        }
    };

    fracs.iter().map(|frac| {
        let (mut n, n_prod) = fac(frac.numer.as_slice());
        let (mut d, d_prod) = fac(frac.denom.as_slice());
        cancel(n.as_mut_slice(), d.as_mut_slice());

        let gcd = integer::gcd(n_prod, d_prod);

        (Fract { numer: n_prod / gcd, denom: d_prod / gcd }, Fract { numer: n, denom: d })
    }).collect()
}
