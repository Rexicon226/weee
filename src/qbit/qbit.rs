#[derive(Debug, Copy, Clone)]
pub struct Qubit<const DIM: usize> {
    pub(crate) dim: usize,
    pub(crate) state: [f64; DIM],
}

impl<const DIM: usize> Qubit<DIM> {

    pub fn new() -> Self {
        let mut state = [0.0; DIM];
        state[0] = 1.0;
        Qubit {
            dim: DIM,
            state
        }
    }

    pub fn apply_matrix(&mut self, matrix: [[f64; DIM]; DIM]) {
        let mut new_state = [0.0; DIM];
        for i in 0..DIM {
            let mut sum = 0.0;
            for j in 0..DIM {
                sum += self.state[j] * matrix[j][i];
            }
            new_state[i] = sum;
        }
        self.state = new_state;
    }

    pub fn tensor_product(self, q2: &mut Qubit<DIM>) -> Qubit<{ DIM * 2 }> {
        let mut new_state = [0.0; DIM * 2];

        for i in 0..2 {
            for j in 0..2 {
                new_state[i * 2 + j] = self.state[i] * q2.state[j];
            }
        }

        Qubit {
            dim: DIM * 2,
            state: new_state
        }
    }

    pub fn split(self) -> (Qubit<{ DIM / 2 }>, Qubit<{ DIM / 2 }>) {
        let mut q1 = Qubit::<{ DIM / 2 }>::new();
        let mut q2 = Qubit::<{ DIM / 2 }>::new();

        for i in 0..DIM / 2 {
            q1.state[i] = self.state[i];
            q2.state[i] = self.state[i + DIM / 2];
        }

        (q1, q2)
    }
}

impl Qubit<2> {
    pub fn probability(&self) -> f64 {
        self.state[0].powi(2) + self.state[1].powi(2)
    }

    pub fn measure(&self) -> u8 {
        let p0 = self.probability();
        if rand::random::<f64>() < p0 {
            0
        } else {
            1
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::qbit::qbit::Qubit;

    #[test]
    fn test_qubit2() {
        let q = Qubit::new();
        println!("{:?}", q);
        assert_eq!(q.state, [1.0, 0.0]);
    }

    #[test]
    fn test_new_qubit4() {
        let q = Qubit::new();
        assert_eq!(q.state, [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_measure2() {
        let q =  Qubit::<2>::new();
        let m = q.measure();
        assert!(m == 0 || m == 1);
    }

    #[test]
    fn test_probability2() {
        let q = Qubit::<2> {
            dim: 2,
            state: [0.6, 0.8],
        };
        let prob = q.probability();
        assert_eq!(prob, 1.0);
    }

    #[test]
    fn test_apply_matrix2() {
        let mut q =  Qubit::<2>::new();
        let matrix = [
            [0.0, 1.0], [1.0, 0.0]
        ];
        q.apply_matrix(matrix);
        assert_eq!(q.state, [0.0, 1.0]);
    }

    #[test]
    fn test_apply_matrix4() {
        let mut q =  Qubit::<4>::new();
        let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        q.apply_matrix(matrix);
        assert_eq!(q.state, [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_split() {
        let q = Qubit::<4>::new();
        let (q0, q1) = q.split();
        assert_eq!(q0.state, [1.0, 0.0]);
        assert_eq!(q1.state, [0.0, 0.0]);
    }
}