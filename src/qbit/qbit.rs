use std::ops::Index;

pub struct Qubit {
    pub(crate) state: [f64; 2],
}

pub struct Qubit4 {
    pub(crate) state: [f64; 4],
}

pub struct Qubit8 {
    pub(crate) state: [f64; 8],
}

#[allow(dead_code)]
impl Qubit {
    pub(crate) fn new() -> Qubit {
        Qubit {
            state: [1.0, 0.0],
        }
    }

    pub(crate) fn apply_matrix(&mut self, matrix: &[[f64; 2]; 2]) {
        let q0 = self.state[0] * matrix[0][0] + self.state[1] * matrix[1][0];
        let q1 = self.state[0] * matrix[0][1] + self.state[1] * matrix[1][1];
        self.state[0] = q0;
        self.state[1] = q1;
    }

    pub(crate) fn split(&self) -> (Qubit, Qubit) {
        let q0 = Qubit {
            state: [self.state[0], 0.0],
        };
        let q1 = Qubit {
            state: [0.0, self.state[1]],
        };
        (q0, q1)
    }

    pub(crate) fn tensor_product(&self, other: &Qubit) -> Qubit4 {
        let mut result = Qubit4::new();
        for i in 0..2 {
            for j in 0..2 {
                result.state[2 * i + j] = self.state[i] * other.state[j];
            }
        }
        result
    }

    pub(crate) fn probability(&self) -> f64 {
        self.state[0].powi(2)
    }

    pub(crate) fn measure(&self) -> u8 {
        let p0 = self.probability();
        if rand::random::<f64>() < p0 {
            0
        } else {
            1
        }
    }
}

#[allow(dead_code)]
impl Qubit4 {
    pub(crate) fn new() -> Qubit4 {
        Qubit4 {
            state: [1.0, 0.0, 0.0, 0.0],
        }
    }

    pub(crate) fn split(&self) -> (Qubit, Qubit) {
        let q0 = Qubit {
            state: [self.state[0], self.state[1]],
        };
        let q1 = Qubit {
            state: [self.state[2], self.state[3]],
        };
        (q0, q1)
    }

    pub(crate) fn tensor_product(&self, other: &Qubit4) -> Qubit8 {
        let mut result = Qubit8::new();
        for i in 0..4 {
            for j in 0..4 {
                result.state[4 * i + j] = self.state[i] * other.state[j];
            }
        }
        result
    }

    pub(crate) fn apply_matrix(&mut self, matrix: &[[f64; 4]; 4]) {
        let q0 = self.state[0] * matrix[0][0] + self.state[1] * matrix[1][0]
            + self.state[2] * matrix[2][0] + self.state[3] * matrix[3][0];
        let q1 = self.state[0] * matrix[0][1] + self.state[1] * matrix[1][1]
            + self.state[2] * matrix[2][1] + self.state[3] * matrix[3][1];
        let q2 = self.state[0] * matrix[0][2] + self.state[1] * matrix[1][2]
            + self.state[2] * matrix[2][2] + self.state[3] * matrix[3][2];
        let q3 = self.state[0] * matrix[0][3] + self.state[1] * matrix[1][3]
            + self.state[2] * matrix[2][3] + self.state[3] * matrix[3][3];
        self.state[0] = q0;
        self.state[1] = q1;
        self.state[2] = q2;
        self.state[3] = q3;
    }
}

impl Qubit8 {
    pub fn new() -> Qubit8 {
        Qubit8 {
            state: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn split(&self) -> (Qubit4, Qubit4) {
        let q0 = Qubit4 {
            state: [self.state[0], self.state[1], self.state[2], self.state[3]],
        };
        let q1 = Qubit4 {
            state: [self.state[4], self.state[5], self.state[6], self.state[7]],
        };
        (q0, q1)
    }

    pub fn tensor_product(&self, other: &Qubit4) -> Qubit8 {
        let mut result = Qubit8::new();
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    for l in 0..2 {
                        let index = 4 * i + 2 * j + k;
                        result.state[index * 2 + l] = self.state[index] * other.state[2 * k + l];
                    }
                }
            }
        }
        result
    }

    pub fn apply_matrix(&mut self, matrix: &[[f64; 8]; 8]) {
        let mut new_state = [0.0; 8];
        for i in 0..8 {
            for j in 0..8 {
                new_state[i] += self.state[j] * matrix[j][i];
            }
        }
        self.state = new_state;
    }
}


impl Index<usize> for Qubit {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.state[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let q = Qubit::new();
        assert_eq!(q[0], 1.0);
        assert_eq!(q[1], 0.0);
    }

    #[test]
    fn test_measure() {
        let q = Qubit::new();
        let m = q.measure();
        assert!(m == 0 || m == 1);
    }

    #[test]
    fn test_apply_matrix() {
        let mut q = Qubit::new();
        let matrix = [[0.0, 1.0], [1.0, 0.0]];
        q.apply_matrix(&matrix);
        assert_eq!(q.state, [0.0, 1.0]);
    }

    #[test]
    fn test_split() {
        let q = Qubit {
            state: [0.0, 1.0],
        };
        let (q0, q1) = q.split();
        assert_eq!(q0.state, [0.0, 0.0]);
        assert_eq!(q1.state, [0.0, 1.0]);
    }

    #[test]
    fn test_tensor_product() {
        let q1 = Qubit {
            state: [1.0, 0.0],
        };
        let q2 = Qubit {
            state: [0.0, 1.0],
        };
        let q4 = q1.tensor_product(&q2);
        assert_eq!(q4.state, [0.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_probability() {
        let q = Qubit {
            state: [0.6, 0.8],
        };
        let prob = q.probability();
        assert_eq!(prob, 0.36);
    }

    #[test]
    fn test_new_qubit4() {
        let q = Qubit4::new();
        assert_eq!(q.state, [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_split_qubit4() {
        let q = Qubit4::new();
        let (q0, q1) = q.split();
        assert_eq!(q0.state, [1.0, 0.0]);
        assert_eq!(q1.state, [0.0, 0.0]);
    }

    #[test]
    fn test_apply_matrix_qubit4() {
        let mut q = Qubit4::new();
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

