#![cfg_attr(not(test), no_std)]

mod clock;
mod constants;
mod error;
mod logger;
mod sequence;
mod synth;

use rand::{rngs::SmallRng, SeedableRng};

pub use error::Error;
use logger::debug;
use sequence::Sequence;
use synth::Synth;

pub struct Poly {
    rng: SmallRng,
    sequence: Sequence,
    synth: Synth,
}

impl Poly {
    pub fn new(seed: u64, sample_rate: u32) -> Result<Self, Error> {
        let mut rng = SmallRng::seed_from_u64(seed);
        debug!("[SEED] {}", seed);

        let sequence = Sequence::new(&mut rng, sample_rate as f32)?;
        debug!("{}", sequence);

        let synth = Synth::new(&mut rng, sample_rate as f32)?;
        debug!("{}", synth);

        Ok(Poly {
            rng,
            sequence,
            synth,
        })
    }

    pub fn next(&mut self) -> f32 {
        if let Some((length, frequency)) = self.sequence.next(&mut self.rng) {
            self.synth.play(&mut self.rng, length, frequency);
        }
        self.synth.next()
    }
}

#[cfg(test)]
mod tests {}
