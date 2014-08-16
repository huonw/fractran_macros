#![feature(phase)]

#[phase(plugin)] extern crate fractran_macros;
extern crate fractran_support;

use fractran_support::Fractran;

fn main() {
    let add = fractran!(3 / 2);
    println!("123 + 45 = {}", add(&[123, 45]).run());

    let mult = fractran!(455 / 33, 11/13, 1/11, 3/7, 11/2, 1/3);
    println!("123 * 45 = {}", mult(&[123, 456]).run());
}
