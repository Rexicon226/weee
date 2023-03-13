use std::ops::Index;

pub struct Qubit {
    state: [f64; 2],
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
    use std::f32::consts::FRAC_1_SQRT_2;
    use super::*;

    #[test]
    fn test_new() {
        let q = Qubit::new();
        assert_eq!(q[0], 1.0);
        assert_eq!(q[1], 0.0);
    }

    #[test]
    fn test_measure() {
        let mut q = Qubit::new();
        let m = q.measure();
        assert!(m == 0 || m == 1);
    }
}

