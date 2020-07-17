#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    /// Adds `v` to this vector, modifying it in-place.
    pub fn add(&mut self, v: &Self) {
        self.x += v.x;
        self.y += v.y;
    }

    /// Adds `v` to this vector, returning the result as a new vector.
    pub fn plus(&self, v: &Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }

    /// Multiplies this vector by the given scalar, modifying it in-place.
    pub fn smult(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }

    /// Multiplies this vector by the given scalar, returning the result as a new vector.
    pub fn stimes(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    /// Creates a vector that is perpendicular to this one.
    pub fn perp(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Creates a vector pointing in the opposite direction as this one.
    pub fn opp(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
