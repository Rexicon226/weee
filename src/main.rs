use crate::logic::gates::Logic;
use crate::qbit::qbit::Qubit;

pub mod qbit;
pub mod logic;

fn bell_state_example() {
    let mut q1 = Qubit::new();
    let mut q2 = Qubit::new();

    Logic::hadamard(&mut q1);

    Logic::cnot(&mut q1, &mut q2);

    let m1 = q1.measure();
    let m2 = q2.measure();

    match (m1, m2) {
        (0, 0) => println!("Bell state |Φ⁺> detected"),
        (0, 1) => println!("Bell state |Ψ⁺> detected"),
        (1, 0) => println!("Bell state |Ψ⁻> detected"),
        (1, 1) => println!("Bell state |Φ⁻> detected"),
        _ => unreachable!(),
    }
}

fn main() {
    print!("Bell state example: ");
    bell_state_example();
}
