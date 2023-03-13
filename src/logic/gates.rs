use std::f64::consts::FRAC_1_SQRT_2;
use crate::qbit::qbit::Qubit;

#[derive(Debug)]
pub struct Logic;

impl Logic {
    // Apply the Hadamard gate to a qubit
    pub fn hadamard(q: &mut Qubit) {
        let h = [
            [FRAC_1_SQRT_2, FRAC_1_SQRT_2],
            [FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
        ];
        q.apply_matrix(&h);
    }

    // Apply the Pauli X gate to a qubit
    pub fn pauli_x(q: &mut Qubit) {
        let x = [[0.0, 1.0], [1.0, 0.0]];
        q.apply_matrix(&x);
    }

    // Apply the Pauli Y gate to a qubit
    pub fn pauli_y(q: &mut Qubit) {
        let y = [[0.0, -1.0], [1.0, 0.0]];
        q.apply_matrix(&y);
    }

    // Apply the Pauli Z gate to a qubit
    pub fn pauli_z(q: &mut Qubit) {
        let z = [[1.0, 0.0], [0.0, -1.0]];
        q.apply_matrix(&z);
    }

    // Apply the phase gate to a qubit
    pub fn phase(q: &mut Qubit) {
        let s = [[1.0, 0.0], [0.0, std::f64::consts::SQRT_2]];
        q.apply_matrix(&s);
    }

    // Apply the inverse phase gate to a qubit
    pub fn phase_inverse(q: &mut Qubit) {
        let s_inv = [[1.0, 0.0], [0.0, -std::f64::consts::SQRT_2]];
        q.apply_matrix(&s_inv);
    }

    // Apply the T gate to a qubit
    pub fn t(q: &mut Qubit) {
        let t = [[1.0, 0.0], [0.0, FRAC_1_SQRT_2]];
        q.apply_matrix(&t);
    }

    // Apply the inverse T gate to a qubit
    pub fn t_inverse(q: &mut Qubit) {
        let t_inv = [[1.0, 0.0], [0.0, -FRAC_1_SQRT_2]];
        q.apply_matrix(&t_inv);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadamard() {
        let mut q = Qubit::new();
        Logic::hadamard(&mut q);
        assert_eq!(q[0], FRAC_1_SQRT_2);
        assert_eq!(q[1], FRAC_1_SQRT_2);
    }

    #[test]
    fn test_pauli_x() {
        let mut q = Qubit::new();
        Logic::pauli_x(&mut q);
        assert_eq!(q[0], 0.0);
        assert_eq!(q[1], 1.0);
    }

    #[test]
    fn test_pauli_y() {
        let mut q = Qubit::new();
        Logic::pauli_y(&mut q);
        assert_eq!(q[0], 0.0);
        assert_eq!(q[1], -1.0);
    }

    #[test]
    fn test_pauli_z() {
        let mut q = Qubit::new();
        Logic::pauli_z(&mut q);
        assert_eq!(q[0], 1.0);
        assert_eq!(q[1], 0.0);
    }

    #[test]
    fn test_phase() {
        let mut q = Qubit::new();
        Logic::phase(&mut q);
        assert_eq!(q[0], 1.0);
        assert_eq!(q[1], FRAC_1_SQRT_2);
    }

    #[test]
    fn test_phase_inverse() {
        let mut q = Qubit::new();
        Logic::phase_inverse(&mut q);
        assert_eq!(q[0], 1.0);
        assert_eq!(q[1], -std::f64::consts::SQRT_2);
    }

    #[test]
    fn test_t() {
        let mut q = Qubit::new();
        Logic::t(&mut q);
        assert_eq!(q[0], 1.0);
        assert_eq!(q[1], FRAC_1_SQRT_2);
    }

    #[test]
    fn test_t_inverse() {
        let mut q = Qubit::new();
        Logic::t_inverse(&mut q);
        assert_eq!(q[0], 1.0);
   }
}

