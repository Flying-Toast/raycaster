#[derive(Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }

    /// Adds `v` to this vector, modifying it in-place.
    pub fn plus(&mut self, v: &Self) {
        self.x += v.x;
        self.y += v.y;
    }

    /// Adds `v` to this vector, returning the result as a new vector.
    pub fn add(&self, v: &Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }

    /// Multiplies this vector by `v`, modifying it in-place.
    pub fn times(&mut self, v: &Self) {
        self.x *= v.x;
        self.y *= v.y;
    }

    /// Multiplies this vector by `v`, returning the result as a new vector.
    pub fn mult(&self, v: &Self) -> Self {
        Self {
            x: self.x * v.x,
            y: self.y * v.y,
        }
    }

    /// Creates a vector that is perpendicular to this one.
    pub fn perp(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}
