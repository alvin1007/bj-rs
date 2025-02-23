struct Complex {
    x: f64,
    y: f64,
}

impl Complex {
    fn new(x: f64, y: f64) -> Self {
        Self {
            x: x,
            y: y,
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }

    fn div(&self, other: &Self) -> Self {
        let d = other.x.powi(2) + other.y.powi(2);
        Self {
            x: (self.x * other.x + self.y * other.y) / d,
            y: (self.y * other.x - self.x * other.y) / d,
        }
    }

    fn sqrt(&self) -> Self {
        let magnitude = (self.x * self.x + self.y * self.y).sqrt();
        let real_part = ((magnitude + self.x) / 2.0).sqrt();
        let imag_part = self.y.signum() * ((magnitude - self.x) / 2.0).sqrt();
        Self {
            x: real_part,
            y: imag_part,
        }
    }
}