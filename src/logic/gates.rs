use std::f64::consts::FRAC_1_SQRT_2;
use crate::qbit::qbit::{Qubit, Qubit8};

#[derive(Debug)]
pub struct Logic;

#[allow(dead_code)]
impl Logic {
    pub fn hadamard(q: &mut Qubit) {
        let h = [
            [FRAC_1_SQRT_2, FRAC_1_SQRT_2],
            [FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
        ];
        q.apply_matrix(&h);
    }

    pub fn cnot(control: &mut Qubit, target: &mut Qubit) {
        let cx = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        let mut q = control.tensor_product(target);
        q.apply_matrix(&cx);
        let (c, t) = q.split();
        *control = c;
        *target = t;
    }

    /*

    pub fn toffoli(control1: &mut Qubit, control2: &mut Qubit, target: &mut Qubit) {
        let ccx = [
            [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        ];
        let mut q: Qubit8 = control1.tensor_product(control2).tensor_product(target);
        q.apply_matrix(&ccx);
        let (c1, c2, t) = q.split();
        *control1 = c1;
        *control2 = c2;
        *target = t;
    }

     */

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

    #[test]
    fn test_cnot() {
        let mut q0 = Qubit::new();
        let mut q1 = Qubit::new();

        // CNOT should leave the qubits in the same state if control is |0>
        Logic::cnot(&mut q0, &mut q1);
        assert_eq!(q0.state, [1.0, 0.0]);
        assert_eq!(q1.state, [0.0, 0.0]);
    }
}

