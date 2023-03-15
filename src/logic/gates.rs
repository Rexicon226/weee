//use std::f64::consts::FRAC_1_SQRT_2;
use crate::qbit::qbit::Qubit;

#[derive(Debug)]
pub struct Logic;

#[allow(dead_code)]
impl Logic {
}

#[cfg(test)]
mod tests {
    use crate::logic::gates::Logic;
    use crate::qbit::qbit::Qubit;
    /*
    #[test]
    fn test_tensor_product2() {
        let q1 =  Qubit::<2>::new();
        let q2 = Qubit::<2> {
            dim: 2,
            state: [0.0, 1.0],
        };

        let q4 = Logic::tensor_product(&q1, &q2);
        println!("{:?}", q4);
        assert_eq!(q4, [0.0, 1.0, 0.0, 0.0]);
    }

     */
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hadamard() {
        qubit_type!(2);
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
 */
