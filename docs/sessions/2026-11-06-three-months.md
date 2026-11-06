# Three-month progression: black-hole imaging

## Result

Added a separate static black-hole image mode, selected with `5`. It backward
traces light rays through a Schwarzschild exterior and colors each image pixel
according to whether the ray is captured, hits the disk, escapes to the
background, or remains near the critical orbit. The existing mode `4` remains
the animated Oppenheimer–Snyder dust-collapse visualization, so the project
can show both the formation-stage surface crossing and a later black-hole
spacetime image.

## Model and units

The image uses geometrized units `G = c = 1` and sets the mass length
`M = GM_physical/c² = 1`. The horizon is at `2M`, the photon sphere at `3M`,
and the Schwarzschild critical impact parameter is `3√3 M`. The camera is
static at `40M`; the thin disk runs from `6M` to `20M`. `A`/`D` or the
left/right arrows change its inclination in five-degree increments, and `R`
resets it. Ray tracing runs on a cancellable background worker; the angle
label updates immediately and the prior frame stays visible until the new one
finishes rendering.

For each pixel, the Schwarzschild null-geodesic orbit equation
`d²u/dφ² + u = 3 M u²`, with `u = 1/r`, is integrated by fixed-step RK4.
The camera's finite-radius local angle is converted to impact parameter using
the static-observer metric factor. Disk intersections use an interpolated
crossing of the disk plane. Emission uses a Newtonian thin-disk radial profile;
the displayed bolometric brightness applies a simplified relativistic
gravitational and orbital Doppler shift. The background star field is
procedural and is warped by the computed asymptotic ray deflection.

## Scope and known limits

This is a first ray-traced Schwarzschild image model, not a simulation of an
evolving black hole or accretion flow. The disk has no hydrodynamics, magnetic
fields, self-shadowing, radiative transfer, or time delays. It does not include
black-hole spin or Kerr frame dragging. The star-collapse animation is an
analytic homogeneous pressureless dust model, not a realistic stellar
interior or a 3D relativistic fluid. Keep those claims separate when describing
the visual result.

## Review needed

- Build and lint with the locked dependency graph.
- Open the visible window, inspect the disk and background lensing at default
  and alternate inclinations, and confirm controls/reset/resize.
- Check image-resolution and geodesic-step sensitivity before treating pixel
  edges or the near-critical ring width as quantitative results.
- Review regeneration speed on the target desktop and consider a cached quality
  setting if higher resolutions are added.
- Update the project README in the learner's own words.

## Reading

- Oppenheimer and Snyder, “On Continued Gravitational Contraction,” *Physical
  Review* 56 (1939), for the idealized dust-collapse model:
  https://journals.aps.org/pr/abstract/10.1103/PhysRev.56.455
- Cunningham and Bardeen, “The optical appearance of a star orbiting an
  extreme Kerr black hole,” *Physical Review D* 12 (1975), for strong-field
  optical appearance and ray-tracing context:
  https://journals.aps.org/prd/abstract/10.1103/PhysRevD.12.323
- “Shadows of spherically symmetric black holes and naked singularities,”
  *Monthly Notices of the Royal Astronomical Society* 481 (2018), for the
  Schwarzschild critical shadow scale `3√3 M`:
  https://doi.org/10.1093/MNRAS/STY2624
