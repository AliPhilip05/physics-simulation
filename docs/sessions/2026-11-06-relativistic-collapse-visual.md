# Relativistic collapse visual — November 6, 2026

## What changed

Added a separate Oppenheimer–Snyder dust-collapse mode. Press **4** to select
it, then **Space** to run. The existing Newtonian shell and 3D particle modes
remain separate (**2** and **3**). The app now opens in the relativistic visual
mode.

The surface follows the analytic uniform-dust parametric solution
`R/R0 = (1 + cos eta)/2` and
`tau/t0 = (eta + sin eta)/(2 sqrt(2))`. The displayed time is comoving proper
time. Initial compactness is chosen as `2GM/(R0 c^2) = 0.10`, so the initial
surface is ten Schwarzschild radii. Before crossing, a wire sphere marks
`Rs` inside the glowing stellar sphere. When the surface crosses `Rs`, the
visual changes to a 3D black-hole silhouette with a spherical horizon marker;
the graph marks `R = Rs`. The display stops at `R/R0 = 0.02`, before the
analytic singularity.

This is an idealized uniform, pressureless dust solution to general relativity,
not a realistic stellar-evolution calculation. It does not model a star's
pressure-supported life, rotation, mass loss, or emitted light. A distant
observer sees the surface approach the gravitational radius asymptotically;
the finite-time crossing shown here uses the comoving proper-time parameter.
The black-hole silhouette is a diagram. The view does not trace light rays or
produce a telescope image. See [Oppenheimer and Snyder (1939)](https://journals.aps.org/pr/abstract/10.1103/PhysRev.56.455).

## Verification

- Added analytic checks for initial radius, horizon-crossing time, crossing
  compactness, the near-singularity display cutoff, and invalid timesteps.
- A standalone analytic readout reaches `R/R0 = 0.1` and compactness `1` at
  `tau/t0 = 1.095340770031`; the visual cutoff is `R/R0 = 0.02` at
  `tau/t0 = 1.109379314365`.
- `cargo fmt --check`, `cargo build --locked --offline`,
  `cargo clippy --locked --offline -- -D warnings`, and `git diff --check`
  passed. The Cargo test suite was not run.
- `cargo run --locked --offline` started the executable. The review window
  still shows an older 2D build, so the current 3D renderer and interaction
  remain visually unverified.

## Next review

Inspect the new scene and confirm that Space animates the star, the displayed
radius meets `Rs` at the plotted crossing, and reset returns to the initial
state. This analytic mode shows surface crossing in comoving time; a distant
observer's apparent image would require light propagation and redshift.
