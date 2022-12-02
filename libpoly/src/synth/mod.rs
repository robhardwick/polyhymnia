mod adsr;
mod filter;
mod operator;
mod signal;

use core::fmt;
use rand::{rngs::SmallRng, Rng};

pub use signal::Signal;

use crate::clock::Clock;
use crate::constants::SYNTH_MUTATE;
use crate::error::Error;
use crate::logger::{debug, error};
use adsr::ADSR;
use filter::Filter;
use operator::Operator;

pub struct Synth {
    sample_rate: f32,
    operators: [Operator; 3],
    adsr: ADSR,
    filter: Filter,
    mutate_clock: Clock,
}

impl Synth {
    pub fn new(rng: &mut SmallRng, sample_rate: f32) -> Result<Self, Error> {
        let operators = [
            Operator::new(sample_rate, 1.0, Signal::Sine),
            Operator::random(rng, sample_rate)?,
            Operator::random(rng, sample_rate)?,
        ];

        let adsr = ADSR::new(rng);

        let filter = Filter::new(rng, sample_rate);

        let mutate_clock = Clock::deadline(rng.gen_range(SYNTH_MUTATE));

        Ok(Synth {
            sample_rate,
            operators,
            adsr,
            filter,
            mutate_clock,
        })
    }

    pub fn play(&mut self, rng: &mut SmallRng, length: usize, frequency: f32) {
        if self.mutate_clock.ready() {
            self.mutate(rng);
        }

        for operator in &mut self.operators {
            operator.set_frequency(frequency);
        }

        self.adsr.start(length);
    }

    pub fn next(&mut self) -> f32 {
        self.mutate_clock.tick();

        self.filter.generate(
            self.operators
                .iter_mut()
                .fold(1.0, |sample, operator| sample * operator.next())
                * self.adsr.next(),
        )
    }

    fn mutate(&mut self, rng: &mut SmallRng) {
        self.mutate_clock.reset(rng.gen_range(SYNTH_MUTATE));

        let index = rng.gen_range(1..=2);
        if let Ok(operator) = Operator::random(rng, self.sample_rate) {
            debug!("[OPERATOR {}] {}", index, operator);
            self.operators[index] = operator;
        } else {
            error!("Operator mutate failed")
        }
    }
}

impl fmt::Display for Synth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[FILTER] {} [ADSR] {} [OPERATORS] {}, {}, {}",
            self.filter, self.adsr, self.operators[0], self.operators[1], self.operators[2]
        )
    }
}

#[cfg(test)]
mod tests {}
