use crate::logic::gates::Logic;
use crate::qbit::qbit::Qubit;

pub mod logic;
pub mod qbit;

fn test_tensor_product2() {
    let q1 = Qubit::<2>::new();
    let q2 = Qubit::<2> {
        dim: 2,
        state: [0.0, 1.0],
    };

    let q4 = Logic::tensor_product(&q1, &q2);
    println!("{:?}", q4);
}

fn main() {
    test_tensor_product2();
    return;
}

