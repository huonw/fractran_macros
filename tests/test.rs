#![feature(plugin)]

#![plugin(fractran_macros)]

macro_rules! go {
    ($($program: expr),*;
     $($input: expr => $expected: expr),*) => {{
         let prog = fractran!($($program),*);
         $(
             {
                 let mut out = prog(&$input[..]);
                 let expected = $expected;
                 let expected = &expected[..];
                 let (matching, zeros) = out.run().split_at(expected.len());
                 assert_eq!(matching, expected);
                 assert!(zeros.iter().all(|x| *x == 0));
             }
             )*
    }}
}

#[test]
fn test_addition() {
    // 2^a 3^b => 3^(a+b)
    go! {
        3/2;
        [0, 0] => [0, 0],
        [1, 0] => [0, 1],
        [0, 1] => [0, 1],
        [1, 1] => [0, 2],
        [12, 34] => [0, 12 + 34]
    }
}

#[test]
fn test_multiplication() {
    // 2^a 3^b => 5^(ab)
    go! {
        5 * 7 * 13/ (11 * 3), 11/13, 1/11, 3/7, 11/2, 1/3;
        &[0, 0] => &[0, 0, 0],
        &[1, 0] => &[0, 0, 0],
        &[0, 1] => &[0, 0, 0],
        &[1, 1] => &[0, 0, 1],
        &[1, 2] => &[0, 0, 2],
        &[2, 1] => &[0, 0, 2],
        &[12, 34] => &[0, 0, 12 * 34]
    }
}

#[test]
fn test_division() {
    // 2^n 3^d 11 => 5^q 7^r
    // n = qd + r
    go! {
        91/66, 11/13, 1/33, 85/11, 57/119, 17/19, 11/17, 1/3;

        &[0, 1, 0, 0, 1] => &[0, 0, 0, 0],
        &[1, 1, 0, 0, 1] => &[0, 0, 1, 0],
        &[1, 2, 0, 0, 1] => &[0, 0, 0, 1],
        &[2, 2, 0, 0, 1] => &[0, 0, 1, 0],
        &[10 * 3 + 2, 3, 0, 0, 1] => &[0, 0, 10, 2]
    }
}

#[test]
fn test_hamming() {
    // https://en.wikipedia.org/wiki/FRACTRAN#Other_examples
    // 2^a => 13^HammingWeight(a)
    go! {
        (2 + 1) * 11 / ((2^2) * 5), 5/11, 13/(2*5), 1/5, 2/3, 2*5/7, 7/2;
        &[0] => &[0, 0, 0, 0, 0, 0],
        &[1] => &[0, 0, 0, 0, 0, 1],
        &[2] => &[0, 0, 0, 0, 0, 1],
        &[3] => &[0, 0, 0, 0, 0, 2],
        &[4] => &[0, 0, 0, 0, 0, 1],
        &[5] => &[0, 0, 0, 0, 0, 2],
        &[6] => &[0, 0, 0, 0, 0, 2],
        &[7] => &[0, 0, 0, 0, 0, 3]
    }
}
