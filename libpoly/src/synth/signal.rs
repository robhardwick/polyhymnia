use core::f32::consts::{FRAC_2_PI, PI, TAU};
use core::fmt;

use libm::{atanf, sinf, tanf};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Signal {
    Sine,
    Square,
    Saw,
}

impl Signal {
    pub fn generate(&self, sample_rate: f32, frequency: f32, clock: f32) -> f32 {
        match self {
            Signal::Sine => sinf(TAU * frequency * (clock / sample_rate)),
            Signal::Square => {
                if (clock / sample_rate) as usize % frequency as usize == 0 {
                    1.0
                } else {
                    -1.0
                }
            }
            Signal::Saw => atanf(1.0 / tanf(FRAC_2_PI * (frequency * (PI * clock) / sample_rate))),
        }
    }
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Signal::Sine => "Sine",
                Signal::Square => "Square",
                Signal::Saw => "Saw",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sine() {
        let signal = Signal::Sine;
        assert_eq!(
            (0..4)
                .map(|index| signal.generate(8.0, 1.0, index as f32))
                .collect::<Vec<f32>>(),
            vec![0.0, 0.70710677, 1.0, 0.70710677]
        )
    }

    #[test]
    fn square() {
        let signal = Signal::Square;
        assert_eq!(
            (0..9)
                .map(|index| signal.generate(4.0, 2.0, index as f32))
                .collect::<Vec<f32>>(),
            vec![1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0]
        )
    }

    #[test]
    fn saw() {
        let signal = Signal::Saw;
        assert_eq!(
            (0..5)
                .map(|index| signal.generate(8.0, 1.0, index as f32))
                .collect::<Vec<f32>>(),
            vec![1.5707963, 1.3207964, 1.0707964, 0.8207963, 0.5707963]
        )
    }

    #[test]
    fn display() {
        assert_eq!("Sine", format!("{}", Signal::Sine));
        assert_eq!("Saw", format!("{}", Signal::Saw));
    }
}
