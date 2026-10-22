/// Initial state only; gravity and time evolution are future exercises.
pub struct Star {
    /// Radius divided by the initial radius, R / R0. Not a pixel distance.
    pub radius: f64,
}

impl Default for Star {
    fn default() -> Self {
        Self { radius: 1.0 }
    }
}

pub struct Simulation {
    pub star: Star,
    pub elapsed: f64,
    pub paused: bool,
}

impl Default for Simulation {
    fn default() -> Self {
        Self {
            star: Star::default(),
            elapsed: 0.0,
            paused: true,
        }
    }
}

impl Simulation {
    pub fn step(&mut self, dt: f64) {
        self.elapsed += dt;
    }
}
