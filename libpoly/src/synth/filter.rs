use core::{f32::consts::FRAC_PI_2, fmt};

use libm::{sinf, tanhf};
use rand::{rngs::SmallRng, Rng};

use crate::constants::{CUTOFF, Q};

pub struct Filter {
    cutoff: f32,
    q: f32,
    resonance: f32,
    p: f32,
    k: f32,
    px: f32,
    s: [f32; 4],
    ps: [f32; 4],
}

impl Filter {
    pub fn new(rng: &mut SmallRng, sample_rate: f32) -> Self {
        let cutoff = rng.gen_range(CUTOFF);
        let q = rng.gen_range(Q);

        let c = 2.0 * cutoff / sample_rate;
        let p = c * (1.8 - 0.8 * c);
        let k = 2.0 * sinf(c * FRAC_PI_2) - 1.0;
        let t1 = (1.0 - p) * 1.386249;
        let t2 = 12.0 + t1 * t1;
        let resonance = q * (t2 + 6.0 * t1) / (t2 - 6.0 * t1);

        Filter {
            cutoff,
            q,
            resonance,
            p,
            k,
            px: 0.0,
            s: [0.0; 4],
            ps: [0.0; 4],
        }
    }

    pub fn generate(&mut self, input: f32) -> f32 {
        let x = -self.resonance * self.s[3] + input;

        self.s[0] = (x + self.px) * self.p - self.k * self.s[0];
        self.s[1] = (self.s[0] + self.ps[0]) * self.p - self.k * self.s[1];
        self.s[2] = (self.s[1] + self.ps[1]) * self.p - self.k * self.s[2];
        self.s[3] = tanhf((self.s[2] + self.ps[2]) * self.p - self.k * self.s[3]);

        self.px = x;
        self.ps[0] = self.s[0];
        self.ps[1] = self.s[1];
        self.ps[2] = self.s[2];

        self.s[3]
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}Hz, {:.2})", self.cutoff, self.q)
    }
}
