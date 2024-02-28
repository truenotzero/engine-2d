use std::time::{Duration, Instant};

/// Calculates the time between ticks
pub struct Ticker(Instant);

impl Default for Ticker {
    fn default() -> Self {
        Self::new()
    }
}

impl Ticker {
    pub fn new() -> Self {
        Self(Instant::now())
    }

    pub fn tick(&mut self) -> Duration {
        let now = Instant::now();
        let dt = now - self.0;
        self.0 = now;
        dt
    }
}

pub trait Tickable {
    fn tick(&mut self, dt: Duration);
}

/// Ticks at a regular interval
pub struct Timer {
    accumulator: Duration,
    interval: Duration,
}

impl Timer {
    pub fn new(interval: Duration) -> Self {
        Self {
            accumulator: Duration::ZERO,
            interval,
        }
    }

    pub fn reset(&mut self) {
        self.accumulator = Duration::from_secs(0);
    }

    pub fn tick(&mut self, dt: Duration) -> bool {
        self.accumulator += dt;
        let ret = self.accumulator >= self.interval;

        if ret {
            self.accumulator -= self.interval;
        }

        ret
    }
}

/// Measures a cooldown
pub struct Cooldown {
    accumulator: Duration,
    duration: Duration,
}

impl Cooldown {
    pub fn new(duration: Duration) -> Self {
        Self {
            accumulator: Duration::ZERO,
            duration,
        }
    }

    /// reset the cooldown
    pub fn reset(&mut self) {
        self.accumulator = Duration::from_secs(0);
    }

    /// start the cooldown
    pub fn enable(&mut self) {
        self.accumulator = self.duration;
    }

    /// advances the cooldown
    /// returns false if the cooldown is still active
    ///         true if the cooldown is over
    pub fn tick(&mut self, dt: Duration) -> bool {
        self.accumulator = self.accumulator.saturating_sub(dt);
        self.accumulator == Duration::ZERO
    }
}
