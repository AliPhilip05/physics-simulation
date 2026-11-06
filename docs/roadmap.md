# Project roadmap

## Aim and first scientific model

Plan for five core weeks and a later spatial 3D extension, with room to spend
longer on derivations and reviews.
The first deliverable is an educational model of spherical collapse, shown as a
2D cross-section. The physical state depends on radius only; the image does not
imply that the calculation is two-dimensional.

We will first learn numerical methods using a pressureless Newtonian sphere,
then visualize the analytic Oppenheimer–Snyder model of relativistic dust
collapse. Dust means pressureless matter, not interstellar grains. This model
isolates gravity after support has been removed. It does not model the star's
preceding life, nuclear burning, pressure support, neutrinos, rotation, or a
supernova. Adding realistic stellar matter is a later research phase.

The relativistic milestone is a separate, explicitly labeled model. Extending
Newtonian gravity to small radii and coloring a circle black cannot establish
black-hole formation. The analytic relativistic solution gives us a tractable
route to that stage without immediately writing an Einstein-equation solver.

The first spatial 3D extension is a separate Newtonian N-body model. It evolves
equal-mass points with three position and velocity components and direct
pairwise gravity. Fixed Plummer softening regularizes close encounters. This is
a collisionless particle model, not a fluid star, and it does not establish
relativistic collapse. A 3D fluid solver needs separate density, pressure,
equation-of-state, and resolution milestones.

The black-hole imaging checkpoint is a third, separate model: a static
Schwarzschild spacetime with backward-traced light rays and a thin emitting
disk. It makes the event-horizon capture region, photon sphere, shadow, and
disk lensing visible. Its disk emissivity is a Newtonian prescription with
relativistic photon energy shifts; it is not a self-consistent accretion-flow
simulation or a rotating (Kerr) black hole. The collapsing dust sphere remains
the time-dependent formation scene, while this mode depicts light around a
black hole after formation.

## How we work

1. Discuss one physical question, its assumptions, and the equations.
2. Agree on a coherent assignment and observable acceptance criteria.
3. You research the resources, choose interfaces, implement, and write your notes.
4. I review correctness, units, Rust structure, and numerical behavior.
5. You make revisions, run the window, and explain what the evidence shows.

Each completed checkpoint keeps `cargo run` working. After the first successful
build, retain `Cargo.lock` and verify with `cargo run --locked`. Avoid starting
several unfinished features at once. The schedule is a guide, not a deadline.

Session records: [October 22, 2026](sessions/2026-10-22.md),
[October 23, 2026](sessions/2026-10-23.md),
[October 26, 2026](sessions/2026-10-26.md),
[October 30, 2026](sessions/2026-10-30.md),
[Week 8](sessions/2026-10-30-week-8.md), and
[Weeks 9–10](sessions/2026-11-06-weeks-9-10.md), and
[Weeks 11–14](sessions/2026-11-06-month-of-progress.md),
[Weeks 15–27](sessions/2026-11-06-three-months.md), and
[black-hole image mode](sessions/2026-11-06-three-months.md). Milestone 1 is complete,
including the fixed-timestep rendering-rate comparison. The current code contains the
single-boundary collapse, spherical material boundaries, and analytic trajectory
comparison for milestones 2–4. A selectable spatial 3D Newtonian particle model
is now included as the first Week 6 extension. Week 7 adds common-time timestep
comparisons and center-of-mass/momentum diagnostics to that view. The regression
suite and 3D desktop behavior still need review before these milestones are
marked complete. Week 8 adds a controlled oblate start and coordinate-axis
shape diagnostics; its shape response still needs numerical review. Continue
with the
[radial-collapse assignment](assignments/02-radial-collapse.md) review.

Assignments grow in scope and implementation guidance decreases as familiarity
increases. By the final stage, guidance consists of requirements and resources;
see the mentoring agreement in `AGENTS.md`.

## Milestones

| Week | Checkpoints you implement | Visible result | Evidence before moving on |
| --- | --- | --- | --- |
| 0: starter, supplied | Read `main`, `simulation`, and `view` | A centered star with normalized radius 1 | App opens, resize works, Escape and window close work |
| 1: state and time | Space pause/resume; R reset; elapsed time; then a fixed simulation timestep independent of rendering | A running/paused clock next to the unchanged star | Paused time stays fixed, reset is exact, slow rendering does not change the intended model timestep |
| 2: one radial trajectory | Define units; derive gravity; implement velocity Verlet; stop safely before the origin or a strong-field cutoff | Surface radius decreases, with radius and radial velocity displayed | Initial velocity is zero, initial acceleration has the expected sign, energy error decreases when timestep is reduced |
| 3: spherical shells | Add radial boundaries, fixed enclosed masses, and a uniform initial density | Concentric shells contract together | Enclosed mass is monotonic, total mass is fixed, shells remain ordered, uniform collapse is homologous |
| 4: numerical validation | Add a radius-versus-time plot or analytic overlay, run timestep comparisons, save reproducible measurements | Numerical trajectory and reference can be compared in the window | Radius error shrinks at common times as timestep is halved; document tolerances and excluded near-singular regime |
| 5: relativistic dust | Study the Oppenheimer–Snyder solution; implement an analytic mode; label its proper time and gravitational-radius scale | A separately labeled relativistic collapse view reaching surface crossing of the Schwarzschild radius | Check initial conditions, mass and radius scales, and crossing time against the analytic solution; explain observer/time choices |
| 6: spatial 3D extension | Add a deterministic cold 3D particle cloud, softened pair gravity, a 3D view, and total-energy diagnostics | Particles move in x, y, and z in a perspective view; switch back to spherical shells | Check initial center of mass, pair-force momentum balance, energy drift, and timestep sensitivity; state finite-N and collisionless limits |
| 7: 3D numerical validation | Compare fixed-timestep runs against a finer-step reference at a common time; display center-of-mass and momentum drift | The 3D view reports common-time RMS-radius and energy errors for three timesteps | Run the regression suite; confirm both errors decrease as the timestep is reduced; inspect the view and document the finite-N/softening limits |
| 8: symmetry breaking | Compare the centered uniform-sphere sample with an oblate version under unchanged gravity, mass, particle count, and timestep | Toggle sphere/oblate starts and watch the three axis RMS extents and minor-to-major ratio evolve | Confirm both starts have the same total mass and outer radius, the oblate start remains centered, and explain that coordinate-axis spreads are not principal axes for a tilted shape |
| 9: shape tensor | Replace axis-aligned spread interpretation with eigenvalues of the centered position-covariance tensor | Principal RMS extents and minor-to-major ratio stay the same when the particle cloud is rigidly rotated | Compare eigenvalues before and after a rigid rotation; distinguish position covariance from the mechanical inertia tensor |
| 10: rotational collapse | Add solid-body azimuthal velocity to the same initial sphere; measure the angular-momentum vector | Toggle spin independently of spherical/oblate shape and display angular momentum | Confirm zero net linear momentum, nonzero initial angular momentum, and its conservation under central pair forces; compare collapse evidence without calling the point cloud a fluid |
| 11: particle-count control | Generalize the deterministic antipodal sampler and model validation to configurable even N | Cycle N = 32, 64, 128, 256 without changing shape, spin, total mass, or R0 | Check each particle has mass 1/N, center of mass is zero, and reset/toggles preserve the selected count |
| 12: finite-N comparison | Compare RMS radius and principal shape at a shared time across particle counts | Report differences from the largest deterministic sample | Treat N=256 as a finite reference only; inspect whether errors vary monotonically and do not call this a continuum-convergence proof |
| 13: bounded history | Sample radius and principal shape at fixed model-time intervals with a fixed memory cap | Keep a rolling recent history while the simulation runs | Confirm sampling uses model time, stays bounded during long runs, and reset clears the previous run |
| 14: history visualization | Draw normalized RMS radius and minor-to-major shape ratio over the retained interval | The 3D view shows current model diagnostics, a rolling history plot, and the finite-N comparison | Visually inspect all four N settings, both shapes, both spin states, resizing, pause, and reset; record limits of the particle model |
| 15: relativistic collapse visual | Add the analytic Oppenheimer–Snyder surface trajectory as a separate mode using comoving proper time and a stated initial compactness | A luminous dust sphere contracts to its Schwarzschild radius, then becomes a black-hole silhouette | Check the parametric surface solution and crossing time; keep it separate from Newtonian modes and label that the display is not a distant-observer image or realistic stellar evolution |
| 16: Schwarzschild ray tracing | Trace camera pixels backward as null geodesics in a static Schwarzschild exterior; classify horizon capture, disk intersection, escape, and near-critical rays | Mode 5 shows a calculated shadow, photon sphere scale, lensed background, and thin disk | Check horizon 2M, photon sphere 3M, critical impact parameter 3√3M, ray classification counts, and image behavior at several inclinations |
| 17: emitting disk and transfer | Add a stated thin-disk emissivity profile and local orbital motion; compute gravitational and Doppler energy shifts along rays | Disk brightness and color vary with radius and viewing inclination | Check limits at low orbital speed and large radius; document the Newtonian emissivity and simplified bolometric transfer |
| 18: visual controls and model boundaries | Connect the static image to the existing mode selector; display units, observer radius, inclination, ray counts, and shortcuts | Switch between animated relativistic collapse (4) and the static black-hole image (5) | Verify the window remains responsive during generation, reset and inclination controls work, and all models stay clearly labeled as separate approximations |
| 19–27: deepen and validate | Review ray tracing, resolution, disk transfer, and comparison to published Schwarzschild lensing results before choosing the next model | Improve scientific evidence before adding spin, disk dynamics, or fluid collapse | Measure integration and image-resolution sensitivity; preserve a small reproducible baseline and document limits before expanding to Kerr or relativistic hydrodynamics |

Optional next phase: hydrostatic pressure and an equation of state, pressure
loss as a collapse trigger, a spherical fluid solver, then relativistic
hydrodynamics. We will scope each separately after the core project works.

## Scientific checkpoints for later sessions

Do not implement this list all at once. We will derive and introduce each item
when its milestone begins.

- Use `f64` for state. Start with dimensionless variables, choosing scales so
  that `G = M_total = R_initial = 1`. Define the conversion back to physical
  units before interpreting any result in seconds or kilometers.
- The Newtonian equation for a spherical material boundary before shell
  crossing is `d²r/dt² = -G M_enclosed / r²`. A finite shell discretization must
  state how its self-gravity is represented; material boundaries with enclosed
  mass are a useful first formulation.
- A uniform sphere released from rest has the reference free-fall time
  `t_ff = sqrt(3 pi / (32 G rho_initial))`. Compare trajectories at finite radii
  rather than attempting to integrate through the singular endpoint.
- Validate the integrator with energy and convergence. For one outer material
  boundary, the conserved specific energy is `v²/2 - G M_total/r`. A total
  energy diagnostic for the whole sphere needs a consistent mass-weighted
  formulation; do not blindly sum that boundary diagnostic over shells.
- Guard against non-finite values, nonpositive radii, shell crossing, and a
  timestep that is too large compared with the local dynamical time.
- For non-spherical clouds, use the eigenvalues of the centered position
  covariance for orientation-independent shape extents. These are not the
  mechanical inertia-tensor moments, which depend on trace minus covariance.
- With central pair forces, total angular momentum should be conserved. State
  the spin axis and angular-speed units for any rotating initial condition.
- Monitor compactness `C = 2 G M / (r c²)` once physical scales are chosen.
  Select and document a weak-field cutoff for the Newtonian mode. It cannot
  substantiate horizon formation.
- In the relativistic model, define radius as areal radius and distinguish
  comoving proper time from a distant observer's time. Surface crossing of
  `r_s = 2 G M/c²` is a useful milestone; it is not a computation of the full
  event horizon. Do not label every interior point with the total exterior mass.
- The cross-section is a diagram of the model state. A distant-camera image
  would also require light propagation and redshift; ray tracing is an extension.

## Code boundaries

```text
src/main.rs        Window, input, fixed-step accumulator and application loop
src/simulation.rs  Normalized shell state, Newtonian gravity, guarded Verlet steps, analytic benchmark, and trajectory history
src/view.rs        Draws material boundaries, diagnostics, and the numerical/analytic trajectory plot
src/nbody.rs       3D equal-mass particles, softened gravity, velocity Verlet, shape and conservation diagnostics
src/view_3d.rs     Perspective rendering of the spatial 3D particle cloud
docs/              Milestones, exercises, and reference material
README.md          Your explanation of how to run and interpret the project
```

Split out `physics`, `integrator`, and `diagnostics` modules when actual behavior
warrants them. There are no empty frameworks to fill before the first exercise.

## Sources

- [Macroquad 0.4.16 documentation](https://docs.rs/macroquad/0.4.16/macroquad/):
  the small Rust window and drawing library used by the starter.
- [Macroquad 3D example](https://docs.rs/crate/macroquad/0.4.16/source/examples/3d.rs):
  perspective camera and 3D primitive rendering.
- [Spherical symmetry breaking in cold gravitational collapse of isolated systems](https://academic.oup.com/mnras/article/446/2/1335/2892279):
  a research reference for spherical shell dynamics and the uniform-sphere
  free-fall benchmark; read the introductory collapse equations first.
- [Gravitational collapse from cold uniform asymmetric initial conditions](https://www.aanda.org/articles/aa/abs/2021/08/aa41040-21/aa41040-21.html):
  finite-N particle collapse and sensitivity to initial asymmetries.
- [An energy-conserving formalism for adaptive gravitational force softening](https://academic.oup.com/mnras/article/374/4/1347/963634):
  softened gravitational potentials and consistent energy accounting; this
  project uses fixed, rather than adaptive, Plummer softening.
- [On Continued Gravitational Contraction, Oppenheimer and Snyder (1939)](https://journals.aps.org/pr/abstract/10.1103/PhysRev.56.455):
  the original idealized relativistic collapse model, for week 5.
- [GR1D: A New Open-Source Code for Spherically-Symmetric Stellar Collapse to Neutron Stars and Black Holes](https://arxiv.org/abs/0912.2393):
  later reading illustrating the additional machinery needed for more realistic
  stellar core collapse. It is not required reading for the first exercises.
