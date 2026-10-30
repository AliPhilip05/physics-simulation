/// Number of material boundaries used to represent the uniform sphere.
pub const SHELL_COUNT: usize = 16;
/// Numerical stopping radius of the surface, in units of the initial radius.
/// This is an integration cutoff, not a relativistic horizon.
pub const SURFACE_CUTOFF: f64 = 0.25;

/// A spherical material boundary enclosing a fixed fraction of the total mass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shell {
    /// Initial radius in units of the sphere's initial surface radius.
    initial_radius: f64,
    /// Fixed enclosed mass in units of the total mass.
    enclosed_mass: f64,
    /// Current radius in units of the initial surface radius.
    radius: f64,
    /// Current radial velocity in units R0 / t0; negative is inward.
    radial_velocity: f64,
}

impl Shell {
    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn radial_velocity(&self) -> f64 {
        self.radial_velocity
    }

    /// Newtonian acceleration from the fixed enclosed mass, with G M_total = 1.
    pub fn radial_acceleration(&self) -> f64 {
        -self.enclosed_mass / (self.radius * self.radius)
    }

    /// Specific orbital energy for this material boundary in normalized units.
    pub fn specific_energy(&self) -> f64 {
        0.5 * self.radial_velocity * self.radial_velocity - self.enclosed_mass / self.radius
    }
}

/// Sample of the surface trajectory, kept in model units for the plot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrajectoryPoint {
    pub time: f64,
    pub radius: f64,
}

#[derive(Debug)]
pub struct Simulation {
    /// Ordered material boundaries, from innermost to outer surface.
    shells: Vec<Shell>,
    /// Time divided by t0 = sqrt(R0^3 / (G M_total)).
    pub elapsed: f64,
    pub paused: bool,
    /// Terminal rejection reason; reset clears it.
    pub status: Option<&'static str>,
    /// Accepted surface states used by the validation plot.
    trajectory: Vec<TrajectoryPoint>,
}

impl Default for Simulation {
    fn default() -> Self {
        let shells = (1..=SHELL_COUNT)
            .map(|index| {
                let initial_radius = index as f64 / SHELL_COUNT as f64;
                // Uniform density implies M(<r) / M_total = (r / R0)^3.
                let enclosed_mass = initial_radius.powi(3);
                Shell {
                    initial_radius,
                    enclosed_mass,
                    radius: initial_radius,
                    radial_velocity: 0.0,
                }
            })
            .collect();
        Self {
            shells,
            elapsed: 0.0,
            paused: true,
            status: None,
            trajectory: vec![TrajectoryPoint {
                time: 0.0,
                radius: 1.0,
            }],
        }
    }
}

impl Simulation {
    pub fn shells(&self) -> &[Shell] {
        &self.shells
    }

    pub fn trajectory(&self) -> &[TrajectoryPoint] {
        &self.trajectory
    }

    pub fn surface(&self) -> &Shell {
        self.shells
            .last()
            .expect("the default sphere has a surface shell")
    }

    /// Maximum absolute drift of the per-boundary specific energies.
    pub fn max_specific_energy_error(&self) -> f64 {
        self.shells
            .iter()
            .map(|shell| (shell.specific_energy() + shell.initial_radius.powi(2)).abs())
            .fold(0.0, f64::max)
    }

    /// Accept all boundary candidates and time together, or preserve the state.
    pub fn step(&mut self, dt: f64) -> Result<(), &'static str> {
        if let Some(reason) = self.status {
            self.paused = true;
            return Err(reason);
        }
        if !dt.is_finite() || dt <= 0.0 {
            return self.stop("Timestep must be finite and positive");
        }
        if !self.elapsed.is_finite() || self.elapsed < 0.0 {
            return self.stop("Simulation time is invalid");
        }
        if self.shells.is_empty() {
            return self.stop("Sphere has no material boundaries");
        }
        if !self.valid_current_state() {
            return self.stop("Current shell state is invalid or crossed");
        }

        let new_elapsed = self.elapsed + dt;
        if !new_elapsed.is_finite() {
            return self.stop("Proposed simulation time is not finite");
        }

        let mut candidates = Vec::with_capacity(self.shells.len());
        for shell in &self.shells {
            let acceleration = shell.radial_acceleration();
            let new_radius =
                shell.radius + shell.radial_velocity * dt + 0.5 * acceleration * dt * dt;
            if !new_radius.is_finite() || new_radius <= 0.0 {
                return self.stop("Proposed shell radius is invalid");
            }
            let new_acceleration = -shell.enclosed_mass / (new_radius * new_radius);
            let new_velocity = shell.radial_velocity + 0.5 * (acceleration + new_acceleration) * dt;
            if !new_velocity.is_finite() {
                return self.stop("Proposed shell velocity is not finite");
            }
            candidates.push(Shell {
                radius: new_radius,
                radial_velocity: new_velocity,
                ..*shell
            });
        }

        if candidates
            .last()
            .is_some_and(|surface| surface.radius <= SURFACE_CUTOFF)
        {
            return self.stop("Surface numerical cutoff reached");
        }
        if candidates
            .windows(2)
            .any(|pair| pair[0].radius >= pair[1].radius)
        {
            return self.stop("Material shell crossing detected");
        }

        let surface_radius = candidates.last().expect("nonempty candidates").radius;
        self.shells = candidates;
        self.elapsed = new_elapsed;
        self.trajectory.push(TrajectoryPoint {
            time: new_elapsed,
            radius: surface_radius,
        });
        Ok(())
    }

    fn valid_current_state(&self) -> bool {
        self.shells.iter().all(|shell| {
            shell.initial_radius.is_finite()
                && shell.initial_radius > 0.0
                && shell.enclosed_mass.is_finite()
                && shell.enclosed_mass > 0.0
                && shell.radius.is_finite()
                && shell.radius > 0.0
                && shell.radial_velocity.is_finite()
        }) && self.shells.windows(2).all(|pair| {
            pair[0].initial_radius < pair[1].initial_radius
                && pair[0].enclosed_mass < pair[1].enclosed_mass
                && pair[0].radius < pair[1].radius
        }) && self.surface().radius > SURFACE_CUTOFF
    }

    fn stop(&mut self, reason: &'static str) -> Result<(), &'static str> {
        self.status = Some(reason);
        self.paused = true;
        Err(reason)
    }
}

/// Analytic surface radius for a uniform, pressureless sphere released from rest.
/// The normalized free-fall time is pi/sqrt(8). Returns `None` outside the
/// collapse interval. The parameter eta obeys t/t0 = (eta + sin eta)/(2 sqrt(2)).
pub fn analytic_surface_radius(time: f64) -> Option<f64> {
    let free_fall_time = std::f64::consts::PI / 8.0_f64.sqrt();
    if !time.is_finite() || time < 0.0 || time > free_fall_time {
        return None;
    }

    let target = time * 2.0 * 2.0_f64.sqrt();
    let (mut low, mut high) = (0.0, std::f64::consts::PI);
    for _ in 0..64 {
        let eta = 0.5 * (low + high);
        if eta + eta.sin() < target {
            low = eta;
        } else {
            high = eta;
        }
    }
    Some(0.5 * (1.0 + (0.5 * (low + high)).cos()))
}

/// Compare surface-radius error at tau = 0.5 using three fixed timesteps.
/// Returns `(dt, absolute_radius_error)` pairs against the analytic solution.
pub fn radius_convergence_report() -> [(f64, f64); 3] {
    let reference = analytic_surface_radius(0.5).expect("tau=0.5 is before free fall");
    [0.01, 0.005, 0.0025].map(|dt| {
        let mut simulation = Simulation::default();
        for _ in 0..(0.5 / dt) as usize {
            simulation
                .step(dt)
                .expect("benchmark stays above the cutoff");
        }
        (dt, (simulation.surface().radius - reference).abs())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "{actual} != {expected}"
        );
    }

    #[test]
    fn initial_uniform_sphere_has_monotonic_fixed_enclosed_mass() {
        let simulation = Simulation::default();
        assert_eq!(simulation.shells.len(), SHELL_COUNT);
        assert_eq!(simulation.surface().enclosed_mass, 1.0);
        for pair in simulation.shells.windows(2) {
            assert!(pair[0].radius < pair[1].radius);
            assert!(pair[0].enclosed_mass < pair[1].enclosed_mass);
        }
    }

    #[test]
    fn uniform_shells_follow_the_same_scale_factor() {
        let mut simulation = Simulation::default();
        for _ in 0..40 {
            simulation.step(0.01).unwrap();
        }
        let scale = simulation.surface().radius;
        for shell in &simulation.shells {
            close(shell.radius / shell.initial_radius, scale, 2e-12);
        }
    }

    #[test]
    fn shell_order_and_total_mass_survive_evolution() {
        let mut simulation = Simulation::default();
        for _ in 0..40 {
            simulation.step(0.01).unwrap();
        }
        assert_eq!(simulation.surface().enclosed_mass, 1.0);
        assert!(
            simulation
                .shells
                .windows(2)
                .all(|pair| pair[0].radius < pair[1].radius)
        );
    }

    #[test]
    fn analytic_solution_starts_at_one_and_reaches_zero_at_free_fall_time() {
        close(analytic_surface_radius(0.0).unwrap(), 1.0, 1e-14);
        close(
            analytic_surface_radius(std::f64::consts::PI / 8.0_f64.sqrt()).unwrap(),
            0.0,
            1e-14,
        );
        assert!(analytic_surface_radius(-0.1).is_none());
    }

    #[test]
    fn radius_error_converges_at_common_model_time() {
        let errors = radius_convergence_report().map(|(_, error)| error);
        assert!(errors[1] < errors[0] && errors[2] < errors[1], "{errors:?}");
    }

    #[test]
    fn a_rejected_step_preserves_every_shell_and_the_trajectory() {
        let mut simulation = Simulation::default();
        let shell_before = simulation.shells.clone();
        let trajectory_before = simulation.trajectory.clone();
        assert!(simulation.step(f64::NAN).is_err());
        assert_eq!(simulation.shells, shell_before);
        assert_eq!(simulation.trajectory, trajectory_before);
        assert_eq!(simulation.elapsed, 0.0);
        assert!(simulation.status.is_some());
    }

    #[test]
    fn surface_cutoff_stops_before_accepting_any_boundary() {
        let mut simulation = Simulation::default();
        for shell in &mut simulation.shells {
            shell.radial_velocity = -20.0 * shell.initial_radius;
        }
        let shells_before = simulation.shells.clone();
        assert_eq!(
            simulation.step(0.04),
            Err("Surface numerical cutoff reached")
        );
        assert_eq!(simulation.shells, shells_before);
        assert_eq!(simulation.elapsed, 0.0);
    }

    #[test]
    fn crossed_current_boundaries_are_rejected() {
        let mut simulation = Simulation::default();
        let last_index = simulation.shells.len() - 1;
        simulation.shells[last_index - 1].radius = 0.8;
        assert_eq!(
            simulation.step(0.01),
            Err("Current shell state is invalid or crossed")
        );
    }
}
