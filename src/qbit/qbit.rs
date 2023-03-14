use std::ops::Index;

pub struct Qubit {
    pub(crate) state: [f64; 2],
}

pub struct Qubit4 {
    pub(crate) state: [f64; 4],
}

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

    pub(crate) fn tensor_product(&self, other: &Qubit) -> Qubit {
        let mut result = Qubit::new();
        for i in 0..2 {
            for j in 0..2 {
                result.state[2 * i + j] = self.state[i] * other.state[j];
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

    pub fn cnot(control: &mut Qubit, target: &mut Qubit) {
        let cx = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        let mut q= control.tensor_product(target);
        q.apply_matrix(&cx);
        let (c, t) = q.split();
        *control = c;
        *target = t;
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
}

