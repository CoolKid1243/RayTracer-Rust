#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    // Default constructor empty interval
    pub fn new_empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    // Custom constructor
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    // Returns the size of the interval
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    // Check if the interval contains a value (inclusive)
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    // Check if the interval strictly surrounds a value (exclusive)
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    // Predefined empty and universe intervals
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };
}
