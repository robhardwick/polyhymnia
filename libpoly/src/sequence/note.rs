use core::fmt;

use rand::{rngs::SmallRng, seq::SliceRandom, Rng};

use crate::constants::Scale;
use crate::error::Error;

#[derive(Clone, Copy)]
pub struct Note {
    pub length: usize,
    pub frequency: f32,
}

impl Note {
    pub fn new(rng: &mut SmallRng, scale: &Scale, length: usize) -> Result<Self, Error> {
        let length = rng.gen_range(1..=length);
        let frequency = *scale.choose(rng).ok_or(Error::Rng)?;

        Ok(Note { length, frequency })
    }
}

impl Default for Note {
    fn default() -> Self {
        Note {
            length: 0,
            frequency: 0.0,
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {:.2}Hz)", self.length, self.frequency)
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use super::*;

    #[test]
    fn new() {
        let mut rng = SmallRng::seed_from_u64(0);

        let result = Note::new(&mut rng, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], 4);
        assert!(result.is_ok());

        let note = result.unwrap();
        assert_eq!(4.0, note.frequency);
        assert_eq!(3, note.length);
    }

    #[test]
    fn default() {
        let note = Note::default();
        assert_eq!(0.0, note.frequency);
        assert_eq!(0, note.length);
    }

    #[test]
    fn display() {
        let note = Note::default();
        assert_eq!("(0, 0.00Hz)", format!("{}", note));
    }
}
