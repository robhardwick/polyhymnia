use core::fmt;

use rand::rngs::SmallRng;
use rand::Rng;

use crate::clock::Clock;
use crate::constants::{ATTACK, DECAY, RELEASE, SUSTAIN};

pub struct ADSR {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    value: f32,
    length: f32,
    delta: f32,
    clock: Clock,
    state: State,
}

impl ADSR {
    pub fn new(rng: &mut SmallRng) -> Self {
        let attack = rng.gen_range(ATTACK);
        let decay = rng.gen_range(DECAY);
        let sustain = rng.gen_range(SUSTAIN);
        let release = rng.gen_range(RELEASE);

        ADSR {
            attack,
            decay,
            sustain,
            release,
            ..Default::default()
        }
    }

    pub fn start(&mut self, length: usize) {
        self.state = State::Attack;
        self.value = 0.0;
        self.length = length as f32;

        let length = self.length * self.attack;
        self.delta = 1.0 / length;
        self.clock.reset(length as usize);
    }

    pub fn next(&mut self) -> f32 {
        if self.clock.tick() {
            self.state = self.state.next();

            match self.state {
                State::Decay => {
                    let length = self.length * self.decay;
                    self.delta = (self.sustain - self.value) / length;
                    self.clock.reset(length as usize);
                }
                State::Sustain => {
                    self.delta = 0.0;
                    self.clock.reset(
                        (self.length * (1.0 - self.attack - self.decay - self.release)) as usize,
                    );
                }
                State::Release => {
                    let length = self.length * self.release;
                    self.delta = (-self.value) / length;
                    self.clock.reset(length as usize);
                }
                _ => {
                    self.state = State::Off;
                    self.delta = 0.0;
                    self.value = 0.0;
                }
            }
        }

        self.value += self.delta;
        self.value
    }
}

impl Default for ADSR {
    fn default() -> Self {
        ADSR {
            attack: 0.0,
            decay: 0.0,
            sustain: 0.0,
            release: 0.0,
            value: 0.0,
            length: 0.0,
            delta: 0.0,
            clock: Clock::default(),
            state: State::Off,
        }
    }
}

impl fmt::Display for ADSR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:.2}, {:.2}, {:.2}, {:.2})",
            self.attack, self.decay, self.sustain, self.release
        )
    }
}

#[derive(PartialEq, Debug)]
enum State {
    Attack,
    Decay,
    Sustain,
    Release,
    Off,
}

impl State {
    fn next(&self) -> State {
        match self {
            State::Off => State::Attack,
            State::Attack => State::Decay,
            State::Decay => State::Sustain,
            State::Sustain => State::Release,
            _ => State::Off,
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use super::*;

    #[test]
    fn new() {
        let mut rng = SmallRng::seed_from_u64(0);

        let adsr = ADSR::new(&mut rng);
        assert_eq!(0.2739218, adsr.attack);
        assert_eq!(0.049522623, adsr.decay);
        assert_eq!(0.9389341, adsr.sustain);
        assert_eq!(0.028486688, adsr.release);
        assert_eq!(0.0, adsr.value);
        assert_eq!(0.0, adsr.length);
        assert_eq!(0.0, adsr.delta);
        assert_eq!(true, adsr.clock.ready());
        assert_eq!(State::Off, adsr.state);
    }

    #[test]
    fn attack() {
        let mut adsr = ADSR {
            attack: 0.5,
            decay: 0.5,
            ..Default::default()
        };
        adsr.start(8);

        assert_eq!(
            (0..3).map(|_| adsr.next()).collect::<Vec<f32>>(),
            vec![0.25, 0.5, 0.75]
        )
    }

    #[test]
    fn decay() {
        let mut adsr = ADSR {
            decay: 0.5,
            sustain: 0.5,
            value: 1.0,
            length: 8.0,
            state: State::Attack,
            ..Default::default()
        };
        assert_eq!(
            (0..4).map(|_| adsr.next()).collect::<Vec<f32>>(),
            vec![0.875, 0.75, 0.625, 0.5]
        )
    }

    #[test]
    fn sustain() {
        let mut adsr = ADSR {
            attack: 0.2,
            decay: 0.2,
            release: 0.2,
            value: 0.5,
            length: 10.0,
            state: State::Decay,
            ..Default::default()
        };
        assert_eq!(
            (0..4).map(|_| adsr.next()).collect::<Vec<f32>>(),
            vec![0.5, 0.5, 0.5, 0.5]
        )
    }

    #[test]
    fn release() {
        let mut adsr = ADSR {
            release: 0.5,
            value: 0.5,
            length: 8.0,
            state: State::Sustain,
            ..Default::default()
        };
        assert_eq!(
            (0..4).map(|_| adsr.next()).collect::<Vec<f32>>(),
            vec![0.375, 0.25, 0.125, 0.0]
        )
    }
}
