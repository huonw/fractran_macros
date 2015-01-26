use std::cmp;
use std::iter::repeat;
use num::integer;

use slow_primes::Primes;

#[derive(Show)]
pub struct Fract<T> {
    pub numer: T,
    pub denom: T,
}

pub fn factorise(fracs: &[Fract<Vec<usize>>]) -> Vec<(Fract<usize>, Fract<Vec<usize>>)> {
    let largest = fracs.iter().map(|f| {
        f.numer.iter().map(|x| *x)
            .chain(f.denom.iter().map(|x| *x)).max().unwrap_or(1)
    }).max().unwrap_or(1);
    let primes = Primes::sieve(largest + 1);
    let mut prime_idx = repeat(0).take(primes.upper_bound() + 1).collect::<Vec<_>>();
    for (i, p) in primes.primes().enumerate() {
        prime_idx[p] = i;
    }

    let combine = |&: a: &mut Vec<usize>, b: &[(usize, usize)]| {
        for &(prime, count) in b.iter() {
            let l = a.len();
            let idx = prime_idx[prime];

            if idx >= l {
                a.extend(repeat(0).take(idx - l + 1));
            }
            a[idx] += count
        }
    };

    let fac = |&: nums: &[usize]| -> (Vec<usize>, usize) {
        let mut ret = vec![];
        let mut prod = 1;
        for n in nums.iter() {
            prod *= *n as usize;
            // by construction, large enough to factor
            combine(&mut ret, &*primes.factor(*n as usize).ok().unwrap());
        }
        (ret, prod)
    };
    let cancel = |&: a: &mut [usize], b: &mut [usize]| {
        for (x, y) in a.iter_mut().zip(b.iter_mut()) {
            let m = cmp::min(*x, *y);
            *x -= m;
            *y -= m;
        }
    };

    fracs.iter().map(|frac| {
        let (mut n, n_prod) = fac(&*frac.numer);
        let (mut d, d_prod) = fac(&*frac.denom);
        cancel(&mut *n, &mut *d);

        let gcd = integer::gcd(n_prod, d_prod);

        (Fract { numer: n_prod / gcd, denom: d_prod / gcd }, Fract { numer: n, denom: d })
    }).collect()
}
