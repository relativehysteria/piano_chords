use std::env::args;
use core::arch::x86_64::_rdtsc;
use pchords::*;

fn rdtsc() -> usize {
    (unsafe { _rdtsc() }) as usize
}

fn main() {
    let mut rng = Rng::new(rdtsc());
    println!("{}", random_chord(&mut rng, args().count() > 1));
}
