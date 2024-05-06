pub struct Interval {
    min: f64, 
    max: f64
}

impl Interval {
    pub fn new() -> Interval {
        Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY
        }
    }

    pub fn build(min: f64, max: f64) -> Interval {
        Interval {
            min: min,
            max: max
        }
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn contains(&self, x: f64) -> bool{
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool{
        self.min < x && x < self.max
    }
}

const EMPTY: Interval = Interval{min: f64::INFINITY, max: f64::NEG_INFINITY};
const UNIVERSE: Interval = Interval{min: f64::NEG_INFINITY, max: f64::INFINITY};