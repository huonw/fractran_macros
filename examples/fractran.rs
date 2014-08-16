#![feature(phase)]

extern crate test;

#[phase(plugin)] extern crate fractran_macros;
extern crate fractran_support;

use fractran_support::Fractran;
use std::num;


fn main() {
    // A fractran interpreter in fractran
    // http://stackoverflow.com/a/1802570/1256624
    //
    // Unfortunately, not even u64 is large enough to hold the encoded
    // form of the multiplication program. :(

    let interpreter = fractran!(
        197*103/((2^11)*101), 101/103, 103*127/(2*101), 101/103, 109/101,
        2*23/(197*109), 109/23, 29/109,197*41*47/(31*59), (11^10)*53/(127*197), 197/53,
        37/197, (7^10)*43/((11^10)*37), 37/43, 59/(37*47), 59/47, 41*61/59, 31*67/(41*61),
        61/67, 7*67/(127*61), 61/67,101/71, 73/((127^9)*29), 79/((127^2)*73),
        83/(127*73), 89/(2*29), 163/29, (127^11)*89/79, 337/83, 2*59/89, 71/61,
        7*173/(127*163), 163/173, 337*167/163, 347/(31*337), 337/347, 151/337,
        1/71,19*179/(3*7*193), 193/179, 157/(7*193), 17*181/193, 7*211/(19*181),
        181/211, 193/181, 157/193, 223/(7*157), 157/223, 281*283/239,
        3*257*269/(7*241), 241/269, 263/241, 7*271/(257*263), 263/271, 281/263,
        241/(17*281), 1/281, 307/(7*283), 283/307, 293/283, 71*131/107, 193/(131*151),
        227/(19*157), 71*311/227, 233/(151*167*311), 151*311/229, 7*317/(19*229),
        229/317, 239*331/217, 71*313/157, 239*251/(151*167*313), 239*251/(151*313),
        149/(251*293), 107/(293*331), 137/199, (2^100)*(13^100)*353/((5^100)*137),
        2*13*353/(5*137), 137/353, 349/137, 107/349, (5^100)*359/((13^100)*149),
        5*359/(13*149), 149/359, 199/149);

    let mut input = [0, .. 46];
    // 3^initial state
    input[2] = num::pow(2, 4) * num::pow(3, 5);
    // * 5^encoded program (this one is 3/2, addition)
    input[3] = 475;
    // * 199
    input[45] = 1;

    let mut addition = interpreter(input.as_slice());

    let mut count = 0u;
    for _ in addition {
        // avoid an infinite loop.
        if count > 1_000_000_000 { break }

        // the interpreter has stepped to the next (high-level) state
        // (stored as 3^state) when the raw state is divisible by 199.
        if addition.state()[45] == 1 {
            println!("{:4}: {}", count, addition.state()[1]);
        }

        count += 1
    }
}
