use std::{
    fmt::Display,
    io::{self, Read},
};

#[derive(Debug)]
struct Stats {
    pub n: u64,

    pub min: u32,
    pub max: u32,

    pub mean: f64,
    pub var: f64,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            n: Default::default(),
            min: u32::MAX,
            max: u32::MIN,
            mean: Default::default(),
            var: Default::default(),
        }
    }
}

impl Stats {
    pub fn update(&mut self, d: u32) {
        self.n += 1;
        self.min = std::cmp::min(self.min, d);
        self.max = std::cmp::max(self.max, d);

        let n = self.n as f64;
        self.var = match self.n {
            1 => 0.0,
            2 => 4.5,
            _ => (n - 2f64) / (n - 1f64) * self.var + 1f64 / n * f64::powi(d as f64 - self.mean, 2),
        };
        self.mean = 1f64 / n * (d as f64 + (n - 1f64) * self.mean)
    }

    pub fn stddev(&self) -> f64 {
        f64::sqrt(self.var)
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "count: {}, min: {}, max: {}, mean: {:.3} (σ={:0.3})",
            self.n,
            self.min,
            self.max,
            self.mean,
            self.stddev()
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Level {
    Low,
    High,
}

impl From<u8> for Level {
    fn from(n: u8) -> Self {
        if n > 0 {
            return Level::High;
        }
        return Level::Low;
    }
}

#[derive(Debug, Default)]
struct Pulse {
    pub duration: Stats,
    pub period: Stats,

    pub start: Option<Level>,
    pub last: Option<Level>,
    pub duty_start: u32,
    pub n: u32,
}

impl Pulse {
    pub fn tick(&mut self, level: Level) {
        use Level::*;
        if self.start.is_none() {
            self.n = 1;
            self.start = Some(level);
            self.last = Some(level);
            if level == High {
                self.duty_start = 0;
            }
            return;
        }

        self.n += 1;

        let (start, last) = (self.start.unwrap(), self.last.unwrap());
        self.last = Some(level);

        match (last, level) {
            (Low, High) => self.duty_start = self.n - 1,
            (High, Low) => self.duration.update(self.n - self.duty_start - 1),
            _ => return,
        };

        if level != start {
            return;
        }

        self.period.update(self.n - 1);
        self.n = 1;
        self.duty_start = 0;
    }

    /* TODO: incremental update instead */
    fn finish(&mut self) {
        if self.start.is_none() {
            return;
        }

        self.tick(self.start.unwrap());
    }
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "duration: {}, period: {}", self.duration, self.period)
    }
}

fn main() -> Result<(), io::Error> {
    let signals: &mut [(_, _, &mut Pulse)] = &mut [
        ("hsync", 0x01, &mut Default::default()),
        ("vsync", 0x04, &mut Default::default()),
        ("end", 0x20, &mut Default::default()),
    ];

    // io::stdin().read_to_end(buf)

    for b in io::stdin().bytes() {
        let b = b?;

        for (_, ref mask, ref mut signal) in signals.iter_mut() {
            signal.tick((b & mask).into());
        }

        // if b & 0x20 > 0 {
        //     break;
        // }
    }

    for (name, _, signal) in signals {
        signal.finish();

        println!("{}: {}", name, signal);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{Level, Pulse, Stats};

    #[test]
    fn test_stats() {
        let mut s = Stats::default();

        s.update(5);
        s.update(8);
        s.update(12);
        s.update(7);

        assert_eq!(s.n, 4);
        assert_eq!(s.min, 5);
        assert_eq!(s.max, 12);

        assert!(
            f64::abs(s.mean - 8f64) < 1e-4,
            "epsilon too large for mean: {:?}",
            s.mean
        );

        assert!(
            f64::abs(s.var - 8.66666667f64) < 1e-4,
            "epsilon too large for variance: {:?}",
            s.var
        )
    }

    #[test]
    #[ignore = "floating point math, yey"]
    fn test_stats_stability() {
        let mut hsync_period = Stats::default();

        for _ in 0..525 {
            hsync_period.update(50);
        }

        assert_eq!(hsync_period.n, 525);
        assert_eq!(hsync_period.min, 50);
        assert_eq!(hsync_period.max, 50);
        assert_eq!(hsync_period.mean, 50.0); // 50.00000000000008
        assert_eq!(hsync_period.var, 0.0); // 0.008587786259541983
    }

    #[test]
    fn test_pulse_sq() {
        let mut p = Pulse::default();

        p.tick(Level::High);
        p.tick(Level::High);
        p.tick(Level::Low);
        p.tick(Level::Low);

        p.finish();

        assert_eq!(p.period.n, 1);
        assert_eq!(p.period.mean, 4f64);

        assert_eq!(p.duration.n, 1);
        assert_eq!(p.duration.mean, 2f64);
    }

    #[test]
    fn test_pulse() {
        let mut p = Pulse::default();

        p.tick(Level::Low);
        p.tick(Level::Low);
        p.tick(Level::Low);
        p.tick(Level::High);
        p.tick(Level::Low);
        p.tick(Level::Low);
        p.tick(Level::High);
        p.tick(Level::High);

        p.finish();

        assert_eq!(p.period.n, 2);
        assert_eq!(p.period.mean, 4f64);

        assert_eq!(p.duration.n, 2);
        assert_eq!(p.duration.mean, 1.5f64);
    }
}
