pub struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn plus(&mut self, v: &Self) {
        self.x += v.x;
        self.y += v.y;
    }

    pub fn add(a: &Self, b: &Self) -> Self {
        Self {
            x: a.x + b.x,
            y: a.y + b.y,
        }
    }

    pub fn times(&mut self, v: &Self) {
        self.x *= v.x;
        self.y *= v.y;
    }

    pub fn mult(a: &Self, b: &Self) -> Self {
        Self {
            x: a.x * b.x,
            y: a.y * b.y,
        }
    }
}
