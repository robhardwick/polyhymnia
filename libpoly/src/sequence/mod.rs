mod note;

use core::fmt;

use rand::{rngs::SmallRng, seq::SliceRandom, Rng};

use crate::clock::Clock;
use crate::constants::{Scale, LENGTH, METRES, SCALES, SEQUENCE_MUTATE, TEMPOS};
use crate::error::Error;
use crate::logger::{debug, error};
use note::Note;

pub struct Sequence {
    tempo: usize,
    scale: &'static Scale,
    notes: [Note; LENGTH],
    length: usize,
    mutate_clock: Clock,
    note_clock: Clock,
    note: usize,
}

impl Sequence {
    pub fn new(rng: &mut SmallRng, sample_rate: f32) -> Result<Self, Error> {
        let tempo = ((60.0 / rng.gen_range(TEMPOS)) * sample_rate) as usize;
        let scale = SCALES.choose(rng).ok_or(Error::Rng)?;

        let length = *METRES.choose(rng).ok_or(Error::Rng)?;

        let mut notes = [Note::default(); LENGTH];
        for i in 0..length {
            notes[i] = Note::new(rng, scale, length)?;
        }

        let mutate_clock = Clock::deadline(rng.gen_range(SEQUENCE_MUTATE));
        let note_clock = Clock::default();

        Ok(Sequence {
            tempo,
            scale,
            notes,
            length,
            mutate_clock,
            note_clock,
            note: length - 1,
        })
    }

    pub fn next(&mut self, rng: &mut SmallRng) -> Option<(usize, f32)> {
        if self.mutate_clock.tick() {
            self.mutate(rng);
        }

        if self.note_clock.tick() {
            Some(self.step())
        } else {
            None
        }
    }

    fn mutate(&mut self, rng: &mut SmallRng) {
        self.mutate_clock.reset(rng.gen_range(SEQUENCE_MUTATE));

        let mut index;
        loop {
            index = rng.gen_range(0..self.length);
            if index != self.note {
                break;
            }
        }

        if let Ok(note) = Note::new(rng, self.scale, self.length) {
            debug!("[NOTE {}] {}", index, note);
            self.notes[index] = note;
        } else {
            error!("Note mutate failed");
        }
    }

    fn step(&mut self) -> (usize, f32) {
        self.note += 1;
        if self.note >= self.length {
            self.note = 0;
        }

        let length = self.notes[self.note].length * self.tempo;

        self.note_clock.reset(length);

        (length, self.notes[self.note].frequency)
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[NOTES] ")?;
        for i in 0..self.length {
            write!(f, "{}", self.notes[i])?;
            if i != self.length - 1 {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::SCALES;

    use rand::SeedableRng;

    #[test]
    fn next() {
        let mut rng = SmallRng::seed_from_u64(0);

        let mut sequence = Sequence {
            tempo: 1,
            scale: &SCALES[0],
            notes: [
                Note {
                    length: 1,
                    ..Default::default()
                },
                Note {
                    length: 2,
                    ..Default::default()
                },
                Note {
                    length: 3,
                    ..Default::default()
                },
                Note {
                    length: 4,
                    ..Default::default()
                },
                Note::default(),
                Note::default(),
                Note::default(),
                Note::default(),
            ],
            length: 4,
            mutate_clock: Clock::deadline(1000),
            note_clock: Clock::default(),
            note: 3,
        };

        assert_eq!(
            (0..11)
                .map(|_| sequence.next(&mut rng))
                .collect::<Vec<Option<(usize, f32)>>>(),
            vec![
                Some((1, 0.0)),
                Some((2, 0.0)),
                None,
                Some((3, 0.0)),
                None,
                None,
                Some((4, 0.0)),
                None,
                None,
                None,
                Some((1, 0.0)),
            ]
        )
    }
}
