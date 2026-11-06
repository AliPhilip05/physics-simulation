# Month of progress — Weeks 11–14

## Week 11: configurable particle count

Generalized the deterministic antipodal sampler to accept even particle counts
from 4 through 1024. The UI cycles among 32, 64, 128, and 256. Resampling keeps
the current spherical/oblate initial geometry and spin selection, preserves
`M_total = 1` and `R0 = 1`, and resets the model time.

## Week 12: finite-particle comparison

At `t/t0 = 0.2` and `dt = 0.002`, compare each selected sample with the same
deterministic `N = 256` realization. The view reports absolute differences in
RMS radius and principal minor-to-major shape ratio. The 256-particle result is
a finite reference, not a continuum solution. The measurements do not vary
softening and do not isolate all sources of finite-N error.

| N | RMS-radius difference | Shape-ratio difference |
| ---: | ---: | ---: |
| 32 | 8.32705869e-4 | 1.69537985e-1 |
| 64 | 7.01070897e-5 | 9.73833249e-2 |
| 128 | 1.51588725e-4 | 5.00848738e-2 |
| 256 | 0 | 0 |

Radius differences are not monotonic between 64 and 128, so the readout does
not claim a smooth convergence rate. It shows that these deterministic finite
samples produce different collapse histories.

## Weeks 13–14: rolling diagnostics and plot

The model records RMS radius and principal shape ratio every `0.02 t0`, keeping
at most 600 samples. The 3D window plots both quantities over the retained
model-time interval. The axis is intentionally normalized: radius by its value
at the start of the current run and shape ratio on `[0, 1]`. Changing N, shape,
or rotation starts a fresh run; reset retains the selected initial setup.

## Verification status

- `cargo fmt --check`, `cargo build --locked --offline`,
  `cargo clippy --locked --offline -- -D warnings`, and `git diff --check`
  passed after implementation.
- A standalone numerical readout generated the table above. It also confirmed
  51 history samples from `t/t0 = 0` through `1` after 500 updates at
  `dt = 0.002`.
- Added regression tests for count validation/mass normalization, finite-N
  report structure, and bounded model-time history. The Cargo test suite was
  not run.
- Desktop interaction and plot layout need visual review.

## Resume here

Run the Cargo tests and inspect the plot after changing particle count, shape,
and spin. Check resize behavior and whether the small plot remains legible. A
next physics phase needs a scoped question: improve finite-N sampling and
softening studies, or begin a separate pressure-supported fluid model. This
point-particle simulation cannot model pressure or a stellar surface.
