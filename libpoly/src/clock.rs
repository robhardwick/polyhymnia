pub struct Clock {
    tick: usize,
    deadline: usize,
}

impl Clock {
    pub fn deadline(deadline: usize) -> Self {
        Clock { tick: 0, deadline }
    }

    pub fn ready(&self) -> bool {
        self.tick >= self.deadline
    }

    pub fn tick(&mut self) -> bool {
        self.tick = self.tick.wrapping_add(1);
        self.ready()
    }

    pub fn reset(&mut self, deadline: usize) {
        self.tick = 0;
        self.deadline = deadline;
    }
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            tick: 0,
            deadline: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let clock = Clock::default();
        assert_eq!(0, clock.tick);
        assert_eq!(0, clock.deadline);
    }

    #[test]
    fn deadline() {
        let clock = Clock::deadline(99);
        assert_eq!(0, clock.tick);
        assert_eq!(99, clock.deadline);
    }

    #[test]
    fn tick() {
        let mut clock = Clock::deadline(5);

        assert_eq!(
            vec![false, false, false, false, true],
            (0..5).map(|_| clock.tick()).collect::<Vec<bool>>()
        );
    }

    #[test]
    fn reset() {
        let mut clock = Clock::deadline(10);
        clock.tick();
        assert_eq!(1, clock.tick);
        assert_eq!(10, clock.deadline);

        clock.reset(5);
        assert_eq!(0, clock.tick);
        assert_eq!(5, clock.deadline);
    }
}
