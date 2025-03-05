pub struct Kalman {
    q: f32,
    r: f32,
    value: Option<f32>,
    covariance: f32,
}

impl Kalman {
    pub fn new(q: f32, r: f32, covariance: f32) -> Kalman {
        Kalman {
            q,
            r,
            value: None,
            covariance,
        }
    }
    pub fn process(&mut self, input: f32) -> f32 {
        match self.value {
            None => {
                self.value = Some(input);
                input
            }
            Some(x0) => {
                let p0 = self.covariance + self.q;
                let k = p0 / (p0 + self.r);
                let x1 = x0 + k * (input - x0);
                let cov = (1.0 - k) * p0;
                self.value = Some(x1);
                self.covariance = cov;
                x1
            }
        }
    }
}
