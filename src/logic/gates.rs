use std::f64::consts::FRAC_1_SQRT_2;
use crate::qbit::qbit::Qubit;

#[derive(Debug)]
pub struct Logic;

impl Logic {
    pub fn hadamard(q: &mut Qubit) {
        let h = [
            [FRAC_1_SQRT_2, FRAC_1_SQRT_2],
            [FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
        ];
        q.apply_matrix(&h);
    }

    pub fn pauli_x(q: &mut Qubit) {
        let x = [[0.0, 1.0], [1.0, 0.0]];
        q.apply_matrix(&x);
    }

    pub fn pauli_y(q: &mut Qubit) {
        let y = [[0.0, -1.0], [1.0, 0.0]];
        q.apply_matrix(&y);
    }

    pub fn pauli_z(q: &mut Qubit) {
        let z = [[1.0, 0.0], [0.0, -1.0]];
        q.apply_matrix(&z);
    }

    pub(crate) fn phase(q: &mut Qubit) {
        let s = [[1.0, 0.0], [0.0, FRAC_1_SQRT_2]];
        q.apply_matrix(&s);
    }

    pub fn phase_inverse(q: &mut Qubit) {
        let s = [[1.0, 0.0], [0.0, -FRAC_1_SQRT_2]];
        q.apply_matrix(&s);
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
        assert!((q.state[0] - 1.0).abs() < 0.0001);
        assert!((q.state[1] - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_phase_inverse() {
        let mut q = Qubit::new();
        Logic::phase(&mut q);
        Logic::phase_inverse(&mut q);
        assert!((q[0] - 1.0).abs() < 0.0001);
        assert!((q[1] - 0.0).abs() < 0.0001);
    }
}

