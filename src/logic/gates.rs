use std::f64::consts::FRAC_1_SQRT_2;
use crate::qbit::qbit::Qubit;

#[derive(Debug)]
pub struct Logic;

#[derive(Debug)]
pub struct Logic11;

#[derive(Debug)]
pub struct Logic22;

#[allow(dead_code)]
impl Logic11 {
    pub fn hadamard(q: &mut Qubit<2>) {
        let hadamard_matrix = [
            [FRAC_1_SQRT_2, FRAC_1_SQRT_2],
            [FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
        ];
        q.apply_matrix(hadamard_matrix);
    }

    pub fn pauli_x(q: &mut Qubit<2>) {
        let pauli_x_matrix = [
            [0.0, 1.0],
            [1.0, 0.0],
        ];
        q.apply_matrix(pauli_x_matrix);
    }

    pub fn pauli_y(q: &mut Qubit<2>) {
        let pauli_y_matrix = [
            [0.0, -1.0],
            [1.0, 0.0],
        ];
        q.apply_matrix(pauli_y_matrix);
    }

    pub fn pauli_z(q: &mut Qubit<2>) {
        let pauli_z_matrix = [
            [1.0, 0.0],
            [0.0, -1.0],
        ];
        q.apply_matrix(pauli_z_matrix);
    }

    pub fn phase(q: &mut Qubit<2>) {
        let phase_matrix = [
            [1.0, 0.0],
            [0.0, 0.0],
        ];
        q.apply_matrix(phase_matrix);
    }

    pub fn phase_inverse(q: &mut Qubit<2>) {
        let phase_inverse_matrix = [
            [1.0, 0.0],
            [0.0, 0.0],
        ];
        q.apply_matrix(phase_inverse_matrix);
    }
}

#[allow(dead_code)]
impl Logic22 {
    pub fn cnot(control: &mut Qubit<2>, target: &mut Qubit<2>) {
        let cnot_matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 0.0],
        ];

        let q = &mut control.tensor_product(target);
        q.apply_matrix(cnot_matrix);
        let (c, t) = q.split();
        *control = c;
        *target = t;
    }
}



#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_1_SQRT_2;
    use super::*;
    #[test]
    fn test_hadamard() {
        let mut q = Qubit::<2>::new();
        Logic11::hadamard(&mut q);
        assert_eq!(q.state[0], FRAC_1_SQRT_2);
        assert_eq!(q.state[1], FRAC_1_SQRT_2);
    }

    #[test]
    fn test_pauli_x() {
        let mut q = Qubit::<2>::new();
        Logic11::pauli_x(&mut q);
        assert_eq!(q.state[0], 0.0);
        assert_eq!(q.state[1], 1.0);
    }

    #[test]
    fn test_pauli_y() {
        let mut q = Qubit::<2>::new();
        Logic11::pauli_y(&mut q);
        assert_eq!(q.state[0], 0.0);
        assert_eq!(q.state[1], -1.0);
    }

    #[test]
    fn test_pauli_z() {
        let mut q = Qubit::<2>::new();
        Logic11::pauli_z(&mut q);
        assert_eq!(q.state[0], 1.0);
        assert_eq!(q.state[1], 0.0);
    }

    #[test]
    fn test_phase() {
        let mut q = Qubit::new();
        Logic11::phase(&mut q);
        assert_eq!((q.state[0] - 1.0).abs(), 0.0);
        assert_eq!((q.state[1] - 0.0).abs(), 0.0);
    }

    #[test]
    fn test_phase_inverse() {
        let mut q = Qubit::new();
        Logic11::phase_inverse(&mut q);
        Logic11::phase(&mut q);
        assert_eq!((q.state[0] - 1.0).abs(), 0.0);
        assert_eq!((q.state[1] - 0.0).abs(), 0.0);
    }



    #[test]
    fn test_tensor_product2() {
        let q1 =  Qubit::<2>::new();

        let  mut q2 = Qubit::<2> {
            dim: 2,
            state: [0.0, 1.0],
        };

        let q4 = q1.tensor_product(&mut q2);
        println!("{:?}", q4);
        assert_eq!(q4.dim, 4);
    }


    #[test]
    fn test_cnot() {
        let mut q0 =  Qubit::<2>::new();
        let  mut q1 = Qubit::<2>::new();

        // CNOT should leave the qubits in the same state if control is |0>
        Logic22::cnot(&mut q0, &mut q1);

        assert_eq!(q0.state, [1.0, 0.0]);
        assert_eq!(q1.state, [0.0, 0.0]);
    }
}

