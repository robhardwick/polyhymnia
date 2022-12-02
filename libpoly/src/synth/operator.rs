use core::fmt;

use rand::{rngs::SmallRng, seq::SliceRandom, Rng};

use super::signal::Signal;
use crate::constants::OPERATORS;
use crate::error::Error;

pub struct Operator {
    sample_rate: f32,
    frequency: f32,
    ratio: f32,
    signal: Signal,
    clock: usize,
}

impl Operator {
    pub fn new(sample_rate: f32, ratio: f32, signal: Signal) -> Self {
        Operator {
            sample_rate,
            frequency: 0.0,
            ratio,
            signal,
            clock: 0,
        }
    }

    pub fn random(rng: &mut SmallRng, sample_rate: f32) -> Result<Self, Error> {
        let (signal, ratio_range) = OPERATORS.choose(rng).ok_or(Error::Rng)?;
        let ratio = rng.gen_range(ratio_range.clone());

        Ok(Self::new(sample_rate, ratio, *signal))
    }

    pub fn set_frequency(&mut self, new_frequency: f32) {
        self.frequency = new_frequency * self.ratio;
    }

    pub fn next(&mut self) -> f32 {
        self.clock = self.clock.wrapping_add(1);
        self.signal
            .generate(self.sample_rate, self.frequency, self.clock as f32)
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.signal, self.ratio)
    }
}

#[cfg(test)]
mod tests {
    use core::f32::consts::PI;

    use rand::SeedableRng;

    use super::*;

    #[test]
    fn random() {
        let mut rng = SmallRng::seed_from_u64(0);

        let result = Operator::random(&mut rng, 1.0);
        assert!(result.is_ok());

        let operator = result.unwrap();
        assert_eq!(1.0, operator.sample_rate);
        assert_eq!(2.8782806, operator.ratio);
        assert_eq!(Signal::Square, operator.signal);
    }

    #[test]
    fn set_frequency() {
        let mut operator = Operator::new(1.0, 0.2, Signal::Sine);
        operator.set_frequency(440.0);
        assert_eq!(88.0, operator.frequency);
    }

    #[test]
    fn next() {
        let mut operator = Operator::new(1.0, 1.0, Signal::Sine);
        operator.set_frequency(PI);
        assert_eq!(
            (0..8).map(|_| operator.next()).collect::<Vec<f32>>(),
            vec![
                0.77685404,
                0.97834,
                0.45523128,
                -0.40504146,
                -0.9653236,
                -0.8106515,
                -0.05557911,
                0.74065745
            ]
        );
    }
}
