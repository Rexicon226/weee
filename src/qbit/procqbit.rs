#![allow(dead_code)]
macro_rules! qubit_type {
    ($dim:expr) => {
        #[derive(Debug)]
        pub struct Qubit {
            state: [f64; $dim],
        }

        impl Qubit {
            pub fn new() -> Self {
                let mut state = [0.0; $dim];
                state[0] = 1.0;
                Qubit { state }
            }

            pub fn apply_matrix(&mut self, matrix: &[[f64; $dim]; $dim]) {
                let mut new_state = [0.0; $dim];
                for i in 0..$dim {
                    let mut sum = 0.0;
                    for j in 0..$dim {
                        sum += self.state[j] * matrix[j][i];
                    }
                    new_state[i] = sum;
                }
                self.state = new_state;
            }

            pub fn probability(&self) -> f64 {
                self.state[0].powi(2)
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
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_qubit() {
        qubit_type!(2);
        let q1 = Qubit::new();
        println!("{:?}", q1);
        assert_eq!(q1.state, [1.0, 0.0]);
    }

    #[test]
    fn test_apply_matrix_qubit4() {
        qubit_type!(4);
        let mut q = Qubit::new();
        let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        q.apply_matrix(&matrix);
        assert_eq!(q.state, [1.0, 0.0, 0.0, 0.0]);
    }

}