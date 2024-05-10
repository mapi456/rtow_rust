use statrs::distribution::Normal;
use rand::distributions::{Distribution, Uniform};

pub enum Distributions {
    NORMAL(f64, f64),
    UNIFORM(f64, f64),
    UNIFORM_INCL(f64, f64)
}

// Sampling.

pub fn sample(dist_type: Distributions) -> f64 {
    let mut rng = rand::rngs::OsRng;
    match dist_type {
        Distributions::NORMAL(mu, std_dev) => {
            Normal::new(mu, std_dev).expect("Normal distribution failed to initialize.").sample(&mut rng)
        }
        Distributions::UNIFORM (loc, width) => {
            Uniform::new(loc, width).sample(&mut rng)
        }
        Distributions::UNIFORM_INCL (loc, width) => {
            Uniform::new_inclusive(loc, width).sample(&mut rng)
        }
    }
}


pub fn random_f64(min: f64, max: f64) -> f64 {
    // Returns a random f64 in the interval [0.0, 1.0)
    sample(Distributions::UNIFORM(min, max))
}

pub fn random_f64_standard() -> f64 {
    // Returns a random f64 in the interval [0.0, 1.0)
    sample(Distributions::UNIFORM(0.0, 1.0))
}


// Distributions.

pub fn standard_normal() -> Normal {
    Normal::new(0.0, 1.0).expect("Standard normal distribution failed to initialize.")
}

pub fn normal(mu: f64, std_dev: f64) -> Normal  {
    Normal::new(mu, std_dev).expect("Normal distribution failed to initialize. Maybe invalid parameters")
}

pub fn standard_uniform() -> Uniform<f64> {
    Uniform::new(0.0, 1.0)
}

pub fn standard_uniform_incl() -> Uniform<f64> {
    Uniform::new_inclusive(0.0, 1.0)
}