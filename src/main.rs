#![feature(generic_const_exprs)]

#![allow(dead_code)]
#![allow(incomplete_features)]

use crate::logic::gates::Logic11;
use crate::qbit::qbit::Qubit;

pub mod logic;
pub mod qbit;


fn main() {
    let mut q1 = Qubit::<2>::new();
    let q2 = Qubit::<2>::new();

    Logic11::hadamard(&mut q1);

    let m1 = q1.measure();
    let m2 = q2.measure();

    println!("{:?}", q1);
    println!("{:?}", q2);
    println!("{} {}", m1, m2);
    return;
}

