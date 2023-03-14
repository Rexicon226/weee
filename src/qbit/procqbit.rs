macro_rules! qubit_type {
    ($dim:expr) => {
        #[derive(Debug)]
        pub struct Qubit {
            state: [f64; $dim],
        }

        impl Qubit {
            pub fn new() -> Self {
                Qubit {
                    state: [1.0, 0.0],
                }
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

            pub fn split(&self) -> (Self, Self) {
                let mut q0 = Qubit {
                    state: [0.0; $dim],
                };
                let mut q1 = Qubit {
                    state: [0.0; $dim],
                };
                for i in 0..$dim/2 {
                    q0.state[i] = self.state[i];
                    q1.state[i] = self.state[i + $dim/2];
                }
                (q0, q1)
            }

            pub fn tensor_product(&self, other: &Qubit) -> Qubit {
                let mut result = Qubit {
                    state: [0.0; $dim * $dim],
                };
                for i in 0..$dim {
                    for j in 0..$dim {
                        result.state[i * $dim + j] = self.state[i] * other.state[j];
                    }
                }
                result
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