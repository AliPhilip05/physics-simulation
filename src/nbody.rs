//! Small, direct-summation 3D Newtonian N-body model.
//!
//! The particles are equal-mass collisionless tracers in a cold uniform sphere.
//! This is a spatial 3D model, not a fluid or relativistic star solver.

use std::collections::VecDeque;

pub const PARTICLE_COUNT: usize = 64;
pub const PLUMMER_SOFTENING: f64 = 0.05;
const G_NORMALIZED: f64 = 1.0;
pub const VALIDATION_TIME: f64 = 0.2;
pub const SOLID_BODY_ANGULAR_SPEED: f64 = 0.5;
pub const PARTICLE_COUNTS: [usize; 4] = [32, 64, 128, 256];
pub const PARTICLE_STUDY_TIME: f64 = 0.2;
pub const PARTICLE_STUDY_DT: f64 = 0.002;
const HISTORY_INTERVAL: f64 = 0.02;
const HISTORY_CAPACITY: usize = 600;

pub type Vec3d = [f64; 3];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle {
    position: Vec3d,
    velocity: Vec3d,
}

impl Particle {
    pub fn position(&self) -> Vec3d {
        self.position
    }
}

#[derive(Debug)]
pub struct ParticleCloud {
    particles: Vec<Particle>,
    oblate_initial_shape: bool,
    rotating_initial_state: bool,
    history: VecDeque<DiagnosticSample>,
    next_history_time: f64,
    pub elapsed: f64,
    pub paused: bool,
    pub status: Option<&'static str>,
    initial_energy: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DiagnosticSample {
    pub time: f64,
    pub rms_radius: f64,
    pub minor_to_major_ratio: f64,
}

impl Default for ParticleCloud {
    fn default() -> Self {
        Self::for_particle_count(PARTICLE_COUNT).expect("default particle count is valid")
    }
}

impl ParticleCloud {
    /// Build a centered sphere from an even number of antipodal sample points.
    pub fn for_particle_count(count: usize) -> Result<Self, &'static str> {
        if !(4..=1024).contains(&count) || !count.is_multiple_of(2) {
            return Err("Particle count must be even and between 4 and 1024");
        }
        let half_count = count / 2;
        let mut positions = Vec::with_capacity(count);
        for index in 1..=half_count {
            // A deterministic low-discrepancy sample of a uniform unit ball.
            let radial_quantile = radical_inverse(index, 2);
            let radius = radial_quantile.cbrt();
            let cos_theta = 1.0 - 2.0 * radical_inverse(index, 3);
            let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();
            let phi = std::f64::consts::TAU * radical_inverse(index, 5);
            let point = [
                radius * sin_theta * phi.cos(),
                radius * cos_theta,
                radius * sin_theta * phi.sin(),
            ];
            positions.push(point);
            positions.push(scale(point, -1.0));
        }

        let outer_radius = positions
            .iter()
            .map(|position| dot(*position, *position).sqrt())
            .fold(0.0, f64::max);
        for position in &mut positions {
            *position = scale(*position, 1.0 / outer_radius);
        }

        let particles: Vec<Particle> = positions
            .into_iter()
            .map(|position| Particle {
                position,
                velocity: [0.0; 3],
            })
            .collect();
        let mut cloud = Self {
            particles,
            oblate_initial_shape: false,
            rotating_initial_state: false,
            history: VecDeque::with_capacity(HISTORY_CAPACITY),
            next_history_time: HISTORY_INTERVAL,
            elapsed: 0.0,
            paused: true,
            status: None,
            initial_energy: 0.0,
        };
        cloud.initial_energy = cloud.total_energy();
        cloud.push_history_sample();
        Ok(cloud)
    }

    fn configured(count: usize, oblate: bool, rotating: bool) -> Result<Self, &'static str> {
        let mut cloud = Self::for_particle_count(count)?;
        if oblate {
            for particle in &mut cloud.particles {
                particle.position[0] *= 0.85;
                particle.position[1] *= 0.65;
            }
            let outer_radius = cloud.outer_radius();
            for particle in &mut cloud.particles {
                particle.position = scale(particle.position, 1.0 / outer_radius);
            }
            cloud.oblate_initial_shape = true;
            cloud.initial_energy = cloud.total_energy();
            cloud.history.clear();
            cloud.next_history_time = HISTORY_INTERVAL;
            cloud.push_history_sample();
        }
        if rotating {
            cloud.add_solid_body_rotation();
        }
        Ok(cloud)
    }

    pub fn is_oblate(&self) -> bool {
        self.oblate_initial_shape
    }

    pub fn is_rotating(&self) -> bool {
        self.rotating_initial_state
    }

    pub fn fresh_initial_state(&self) -> Self {
        Self::configured(
            self.particles.len(),
            self.oblate_initial_shape,
            self.rotating_initial_state,
        )
        .expect("existing particle count remains valid")
    }

    pub fn with_oblate_initial_shape(&self, oblate: bool) -> Self {
        Self::configured(self.particles.len(), oblate, self.rotating_initial_state)
            .expect("existing particle count remains valid")
    }

    pub fn with_rotation_enabled(&self, rotating: bool) -> Self {
        Self::configured(self.particles.len(), self.oblate_initial_shape, rotating)
            .expect("existing particle count remains valid")
    }

    pub fn with_particle_count(&self, count: usize) -> Result<Self, &'static str> {
        Self::configured(
            count,
            self.oblate_initial_shape,
            self.rotating_initial_state,
        )
    }

    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }

    pub fn diagnostic_history(&self) -> &VecDeque<DiagnosticSample> {
        &self.history
    }

    fn add_solid_body_rotation(&mut self) {
        let omega = SOLID_BODY_ANGULAR_SPEED;
        for particle in &mut self.particles {
            let [x, y, _] = particle.position;
            particle.velocity = [-omega * y, omega * x, 0.0];
        }
        self.rotating_initial_state = true;
        self.elapsed = 0.0;
        self.paused = true;
        self.status = None;
        self.initial_energy = self.total_energy();
        self.history.clear();
        self.next_history_time = HISTORY_INTERVAL;
        self.push_history_sample();
    }

    fn push_history_sample(&mut self) {
        self.history.push_back(DiagnosticSample {
            time: self.elapsed,
            rms_radius: self.rms_radius(),
            minor_to_major_ratio: self.minor_to_major_axis_ratio(),
        });
        if self.history.len() > HISTORY_CAPACITY {
            self.history.pop_front();
        }
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    pub fn mass_per_particle(&self) -> f64 {
        1.0 / self.particles.len() as f64
    }

    pub fn rms_radius(&self) -> f64 {
        (self
            .particles
            .iter()
            .map(|particle| dot(particle.position, particle.position))
            .sum::<f64>()
            / self.particles.len() as f64)
            .sqrt()
    }

    pub fn outer_radius(&self) -> f64 {
        self.particles
            .iter()
            .map(|particle| dot(particle.position, particle.position).sqrt())
            .fold(0.0, f64::max)
    }

    pub fn minor_to_major_axis_ratio(&self) -> f64 {
        let extents = self.principal_rms_extents();
        extents[0] / extents[2]
    }

    /// RMS extents along principal axes of the centered position covariance.
    pub fn principal_rms_extents(&self) -> Vec3d {
        let center = self.center_of_mass();
        let mut covariance = [[0.0; 3]; 3];
        for particle in &self.particles {
            let offset = subtract(particle.position, center);
            for row in 0..3 {
                for column in 0..3 {
                    covariance[row][column] +=
                        offset[row] * offset[column] / self.particles.len() as f64;
                }
            }
        }
        let mut eigenvalues = symmetric_eigenvalues(covariance);
        eigenvalues.sort_by(f64::total_cmp);
        eigenvalues.map(|value| value.max(0.0).sqrt())
    }

    pub fn center_of_mass(&self) -> Vec3d {
        let sum = self
            .particles
            .iter()
            .fold([0.0; 3], |total, particle| add(total, particle.position));
        scale(sum, self.mass_per_particle())
    }

    pub fn total_momentum(&self) -> Vec3d {
        let sum = self
            .particles
            .iter()
            .fold([0.0; 3], |total, particle| add(total, particle.velocity));
        scale(sum, self.mass_per_particle())
    }

    pub fn total_angular_momentum(&self) -> Vec3d {
        let mass = self.mass_per_particle();
        self.particles.iter().fold([0.0; 3], |total, particle| {
            add(
                total,
                scale(cross(particle.position, particle.velocity), mass),
            )
        })
    }

    pub fn total_angular_momentum_magnitude(&self) -> f64 {
        dot(self.total_angular_momentum(), self.total_angular_momentum()).sqrt()
    }

    pub fn center_of_mass_offset(&self) -> f64 {
        dot(self.center_of_mass(), self.center_of_mass()).sqrt()
    }

    pub fn total_momentum_magnitude(&self) -> f64 {
        dot(self.total_momentum(), self.total_momentum()).sqrt()
    }

    pub fn total_energy(&self) -> f64 {
        let mass = self.mass_per_particle();
        let kinetic = self
            .particles
            .iter()
            .map(|particle| 0.5 * mass * dot(particle.velocity, particle.velocity))
            .sum::<f64>();
        let mut potential = 0.0;
        for i in 0..self.particles.len() {
            for j in (i + 1)..self.particles.len() {
                let displacement = subtract(self.particles[j].position, self.particles[i].position);
                let softened_distance =
                    (dot(displacement, displacement) + PLUMMER_SOFTENING.powi(2)).sqrt();
                potential -= G_NORMALIZED * mass * mass / softened_distance;
            }
        }
        kinetic + potential
    }

    pub fn absolute_energy_error(&self) -> f64 {
        (self.total_energy() - self.initial_energy).abs()
    }

    /// Advance every particle with synchronized velocity-Verlet updates.
    /// A failed candidate leaves positions, velocities, and time unchanged.
    pub fn step(&mut self, dt: f64) -> Result<(), &'static str> {
        if let Some(reason) = self.status {
            self.paused = true;
            return Err(reason);
        }
        if !dt.is_finite() || dt <= 0.0 {
            return self.stop("Timestep must be finite and positive");
        }
        if !self.elapsed.is_finite()
            || self.elapsed < 0.0
            || self.particles.len() < 4
            || self.particles.len() > 1024
            || !self.particles.len().is_multiple_of(2)
            || !self.particles.iter().all(|particle| {
                particle.position.iter().all(|v| v.is_finite())
                    && particle.velocity.iter().all(|v| v.is_finite())
            })
        {
            return self.stop("Current particle state is invalid");
        }

        let new_elapsed = self.elapsed + dt;
        if !new_elapsed.is_finite() {
            return self.stop("Proposed simulation time is not finite");
        }

        let old_accelerations = accelerations(&self.particles);
        let new_positions: Vec<Vec3d> = self
            .particles
            .iter()
            .zip(&old_accelerations)
            .map(|(particle, acceleration)| {
                add(
                    add(particle.position, scale(particle.velocity, dt)),
                    scale(*acceleration, 0.5 * dt * dt),
                )
            })
            .collect();
        if !new_positions
            .iter()
            .all(|position| position.iter().all(|value| value.is_finite()))
        {
            return self.stop("Proposed particle position is not finite");
        }

        let new_accelerations = accelerations_at(&new_positions);
        let candidates: Vec<Particle> = self
            .particles
            .iter()
            .zip(new_positions)
            .zip(old_accelerations.iter().zip(new_accelerations))
            .map(
                |((particle, position), (old_acceleration, new_acceleration))| Particle {
                    position,
                    velocity: add(
                        particle.velocity,
                        scale(add(*old_acceleration, new_acceleration), 0.5 * dt),
                    ),
                },
            )
            .collect();
        if !candidates.iter().all(|particle| {
            particle.position.iter().all(|value| value.is_finite())
                && particle.velocity.iter().all(|value| value.is_finite())
        }) {
            return self.stop("Proposed particle state is not finite");
        }

        self.particles = candidates;
        self.elapsed = new_elapsed;
        if self.elapsed + 1e-12 >= self.next_history_time {
            self.push_history_sample();
            while self.next_history_time <= self.elapsed + 1e-12 {
                self.next_history_time += HISTORY_INTERVAL;
            }
        }
        Ok(())
    }

    fn stop(&mut self, reason: &'static str) -> Result<(), &'static str> {
        self.status = Some(reason);
        self.paused = true;
        Err(reason)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimestepResult {
    pub dt: f64,
    pub rms_radius_error: f64,
    pub absolute_energy_error: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParticleCountResult {
    pub count: usize,
    pub rms_radius_difference: f64,
    pub shape_ratio_difference: f64,
}

/// Compare coarse runs against a fine-step reference at the same model time.
/// Errors include both integration error and finite-N model behavior.
pub fn timestep_convergence_report() -> Result<[TimestepResult; 3], &'static str> {
    let reference = run_to_time(0.00025, VALIDATION_TIME)?;
    let reference_radius = reference.rms_radius();
    Ok([0.004, 0.002, 0.001].map(|dt| {
        let cloud = run_to_time(dt, VALIDATION_TIME).expect("all report timesteps are valid");
        TimestepResult {
            dt,
            rms_radius_error: (cloud.rms_radius() - reference_radius).abs(),
            absolute_energy_error: cloud.absolute_energy_error(),
        }
    }))
}

fn run_to_time(dt: f64, end_time: f64) -> Result<ParticleCloud, &'static str> {
    run_to_time_with_count(dt, end_time, PARTICLE_COUNT)
}

/// Compare deterministic finite-particle realizations at a common model time.
/// The largest sample is a reference, not a continuum solution.
pub fn particle_number_report() -> Result<[ParticleCountResult; 4], &'static str> {
    let reference = run_to_time_with_count(
        PARTICLE_STUDY_DT,
        PARTICLE_STUDY_TIME,
        *PARTICLE_COUNTS
            .last()
            .expect("particle study has a reference count"),
    )?;
    let reference_radius = reference.rms_radius();
    let reference_shape = reference.minor_to_major_axis_ratio();
    let reports = PARTICLE_COUNTS.map(|count| {
        let cloud = run_to_time_with_count(PARTICLE_STUDY_DT, PARTICLE_STUDY_TIME, count)?;
        Ok(ParticleCountResult {
            count,
            rms_radius_difference: (cloud.rms_radius() - reference_radius).abs(),
            shape_ratio_difference: (cloud.minor_to_major_axis_ratio() - reference_shape).abs(),
        })
    });
    let [first, second, third, fourth] = reports;
    Ok([first?, second?, third?, fourth?])
}

fn run_to_time_with_count(
    dt: f64,
    end_time: f64,
    count: usize,
) -> Result<ParticleCloud, &'static str> {
    let mut cloud = ParticleCloud::for_particle_count(count)?;
    let steps = (end_time / dt).round() as usize;
    for _ in 0..steps {
        cloud.step(dt)?;
    }
    Ok(cloud)
}

fn accelerations(particles: &[Particle]) -> Vec<Vec3d> {
    accelerations_at(
        &particles
            .iter()
            .map(|particle| particle.position)
            .collect::<Vec<_>>(),
    )
}

fn accelerations_at(positions: &[Vec3d]) -> Vec<Vec3d> {
    let mass = 1.0 / positions.len() as f64;
    let softening_squared = PLUMMER_SOFTENING.powi(2);
    let mut result = vec![[0.0; 3]; positions.len()];
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let displacement = subtract(positions[j], positions[i]);
            let distance_squared = dot(displacement, displacement) + softening_squared;
            let factor = G_NORMALIZED * mass / (distance_squared * distance_squared.sqrt());
            let pair_acceleration = scale(displacement, factor);
            result[i] = add(result[i], pair_acceleration);
            result[j] = subtract(result[j], pair_acceleration);
        }
    }
    result
}

fn radical_inverse(mut index: usize, base: usize) -> f64 {
    let mut factor = 1.0 / base as f64;
    let mut value = 0.0;
    while index > 0 {
        value += factor * (index % base) as f64;
        index /= base;
        factor /= base as f64;
    }
    value
}

fn add(left: Vec3d, right: Vec3d) -> Vec3d {
    [left[0] + right[0], left[1] + right[1], left[2] + right[2]]
}

fn subtract(left: Vec3d, right: Vec3d) -> Vec3d {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

fn scale(vector: Vec3d, factor: f64) -> Vec3d {
    [vector[0] * factor, vector[1] * factor, vector[2] * factor]
}

fn dot(left: Vec3d, right: Vec3d) -> f64 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}

fn cross(left: Vec3d, right: Vec3d) -> Vec3d {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}

fn symmetric_eigenvalues(mut matrix: [[f64; 3]; 3]) -> Vec3d {
    for _ in 0..24 {
        let (mut p, mut q) = (0, 1);
        for (row, column) in [(0, 2), (1, 2)] {
            if matrix[row][column].abs() > matrix[p][q].abs() {
                p = row;
                q = column;
            }
        }
        let off_diagonal = matrix[p][q];
        if off_diagonal.abs() < 1e-15 {
            break;
        }

        let tau = (matrix[q][q] - matrix[p][p]) / (2.0 * off_diagonal);
        let sign = if tau >= 0.0 { 1.0 } else { -1.0 };
        let tangent = sign / (tau.abs() + (1.0 + tau * tau).sqrt());
        let cosine = 1.0 / (1.0 + tangent * tangent).sqrt();
        let sine = tangent * cosine;
        matrix[p][p] -= tangent * off_diagonal;
        matrix[q][q] += tangent * off_diagonal;
        matrix[p][q] = 0.0;
        matrix[q][p] = 0.0;

        let mut index = 0;
        while index < 3 {
            if index != p && index != q {
                let ip = matrix[index][p];
                let iq = matrix[index][q];
                matrix[index][p] = cosine * ip - sine * iq;
                matrix[p][index] = matrix[index][p];
                matrix[index][q] = sine * ip + cosine * iq;
                matrix[q][index] = matrix[index][q];
            }
            index += 1;
        }
    }
    [matrix[0][0], matrix[1][1], matrix[2][2]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_cloud_is_three_dimensional_equal_mass_and_centered() {
        let cloud = ParticleCloud::default();
        assert_eq!(cloud.particles.len(), PARTICLE_COUNT);
        assert!((cloud.mass_per_particle() * PARTICLE_COUNT as f64 - 1.0).abs() < 1e-14);
        assert!((cloud.rms_radius() - (3.0_f64 / 5.0).sqrt()).abs() < 0.08);
        let center_of_mass = cloud
            .particles
            .iter()
            .fold([0.0; 3], |sum, particle| add(sum, particle.position));
        assert!(
            center_of_mass
                .iter()
                .all(|component| component.abs() < 1e-14)
        );
        assert!(
            cloud
                .particles
                .iter()
                .all(|particle| particle.velocity == [0.0; 3])
        );
    }

    #[test]
    fn softened_pair_accelerations_conserve_total_momentum() {
        let cloud = ParticleCloud::default();
        let accelerations = accelerations(&cloud.particles);
        let net_acceleration = accelerations
            .iter()
            .fold([0.0; 3], |sum, acceleration| add(sum, *acceleration));
        assert!(
            net_acceleration
                .iter()
                .all(|component| component.abs() < 1e-12)
        );
    }

    #[test]
    fn accepted_step_moves_particles_inward_and_preserves_center_of_mass() {
        let mut cloud = ParticleCloud::default();
        let initial_radius = cloud.rms_radius();
        cloud.step(0.001).unwrap();
        assert!(cloud.rms_radius() < initial_radius);
        let center_of_mass = cloud
            .particles
            .iter()
            .fold([0.0; 3], |sum, particle| add(sum, particle.position));
        assert!(
            center_of_mass
                .iter()
                .all(|component| component.abs() < 1e-12)
        );
        assert_eq!(cloud.elapsed, 0.001);
    }

    #[test]
    fn invalid_step_is_rejected_atomically() {
        let mut cloud = ParticleCloud::default();
        let particles_before = cloud.particles.clone();
        assert!(cloud.step(f64::NAN).is_err());
        assert_eq!(cloud.particles, particles_before);
        assert_eq!(cloud.elapsed, 0.0);
        assert!(cloud.status.is_some());
    }

    #[test]
    fn common_time_step_study_improves_radius_and_energy_errors() {
        let results = timestep_convergence_report().unwrap();
        assert!(results[1].rms_radius_error < results[0].rms_radius_error);
        assert!(results[2].rms_radius_error < results[1].rms_radius_error);
        assert!(results[1].absolute_energy_error < results[0].absolute_energy_error);
        assert!(results[2].absolute_energy_error < results[1].absolute_energy_error);
    }

    #[test]
    fn accepted_updates_preserve_total_momentum() {
        let mut cloud = ParticleCloud::default();
        for _ in 0..100 {
            cloud.step(0.002).unwrap();
        }
        assert!(cloud.center_of_mass_offset() < 1e-12);
        assert!(cloud.total_momentum_magnitude() < 1e-12);
    }

    #[test]
    fn oblate_initial_condition_is_centered_and_has_a_smaller_axis_ratio() {
        let sphere = ParticleCloud::default();
        let oblate = ParticleCloud::default().with_oblate_initial_shape(true);
        assert!(oblate.is_oblate());
        assert!(oblate.center_of_mass_offset() < 1e-14);
        assert!((oblate.outer_radius() - 1.0).abs() < 1e-14);
        assert!(oblate.minor_to_major_axis_ratio() < sphere.minor_to_major_axis_ratio());
        assert!((oblate.total_energy() - oblate.initial_energy).abs() < 1e-14);
    }

    #[test]
    fn principal_extents_are_invariant_under_a_rigid_rotation() {
        let original = ParticleCloud::default().with_oblate_initial_shape(true);
        let before = original.principal_rms_extents();
        let mut rotated = ParticleCloud::default().with_oblate_initial_shape(true);
        let angle: f64 = 0.73;
        let (sine, cosine) = angle.sin_cos();
        for particle in &mut rotated.particles {
            let [x, y, z] = particle.position;
            particle.position = [cosine * x - sine * y, sine * x + cosine * y, z];
        }
        let after = rotated.principal_rms_extents();
        for axis in 0..3 {
            assert!((before[axis] - after[axis]).abs() < 1e-12);
        }
    }

    #[test]
    fn rotating_cloud_has_centered_spin_and_conserves_angular_momentum() {
        let mut cloud = ParticleCloud::default();
        cloud.add_solid_body_rotation();
        let initial_momentum = cloud.total_angular_momentum();
        assert!(cloud.is_rotating());
        assert!(cloud.total_momentum_magnitude() < 1e-14);
        assert!(initial_momentum[2] > 0.0);
        for _ in 0..100 {
            cloud.step(0.001).unwrap();
        }
        let final_momentum = cloud.total_angular_momentum();
        for axis in 0..3 {
            assert!((initial_momentum[axis] - final_momentum[axis]).abs() < 1e-10);
        }
    }

    #[test]
    fn selectable_particle_counts_preserve_equal_mass_and_initial_configuration() {
        let base = ParticleCloud::default()
            .with_oblate_initial_shape(true)
            .with_rotation_enabled(true);
        let larger = base.with_particle_count(128).unwrap();
        assert_eq!(larger.particle_count(), 128);
        assert!(larger.is_oblate());
        assert!(larger.is_rotating());
        assert!((larger.mass_per_particle() * 128.0 - 1.0).abs() < 1e-14);
        assert!((larger.outer_radius() - 1.0).abs() < 1e-14);
        assert!(larger.center_of_mass_offset() < 1e-14);
        assert!(ParticleCloud::for_particle_count(31).is_err());
    }

    #[test]
    fn diagnostic_history_is_sampled_and_bounded() {
        let mut cloud = ParticleCloud::for_particle_count(4).unwrap();
        for _ in 0..(HISTORY_CAPACITY + 2) {
            cloud.step(HISTORY_INTERVAL).unwrap();
        }
        assert_eq!(cloud.diagnostic_history().len(), HISTORY_CAPACITY);
        assert!(cloud.diagnostic_history().front().unwrap().time > 0.0);
        assert!((cloud.diagnostic_history().back().unwrap().time - cloud.elapsed).abs() < 1e-12);
    }

    #[test]
    fn finite_particle_report_uses_largest_sample_as_zero_difference_reference() {
        let results = particle_number_report().unwrap();
        assert_eq!(results.map(|result| result.count), PARTICLE_COUNTS);
        assert_eq!(results[3].rms_radius_difference, 0.0);
        assert_eq!(results[3].shape_ratio_difference, 0.0);
        assert!(results.iter().all(|result| {
            result.rms_radius_difference.is_finite() && result.shape_ratio_difference.is_finite()
        }));
    }
}
