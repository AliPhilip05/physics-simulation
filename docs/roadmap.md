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
[October 23, 2026](sessions/2026-10-23.md), and
[October 26, 2026](sessions/2026-10-26.md). Milestone 1 is complete, including
the fixed-timestep rendering-rate comparison. The current code contains the
single-boundary collapse, spherical material boundaries, and analytic trajectory
comparison for milestones 2–4. A selectable spatial 3D Newtonian particle model
is now included as the first Week 6 extension. Regression tests and desktop
behavior for these changes still need review before the milestones are marked
complete. Continue with the
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
src/nbody.rs       3D equal-mass particles, softened gravity, velocity Verlet, and energy diagnostics
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
