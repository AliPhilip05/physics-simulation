# First session answer sheet

Use this after attempting the [first-session checks](first-session.md).
Explain the reasoning in your own words rather than memorizing the phrasing.
The numbering matches the guide.

## Comprehension checks

### 1. Radius state

`Star` stores the `radius` field, and `Simulation` contains a `Star`. Both
types are defined in `src/simulation.rs`. The radius is normalized as `R / R0`;
the initial value is `1.0`.

### 2. Model values and pixels

`src/view.rs` converts the radius to `f32` and multiplies it by
`pixels_per_unit`. The scale depends on the smaller window dimension.
Resizing changes the number of pixels used to draw the star; its stored model
radius is preserved.

### 3. Waiting for the next frame

`next_frame().await` yields control back to Macroquad between frames so the
framework can present the drawing and process its event loop. The application
then continues its loop for the next frame, instead of spinning continuously
inside its own loop without yielding.

### 4. Numeric precision

`f64` provides more precision for accumulated time and later physical
calculations. Macroquad's drawing coordinates and sizes use `f32`, so the view
converts values at the rendering boundary. Model values retain their `f64`
representation; conversion to `f32` can lose precision.

### 5. Adding one per frame

Adding `1.0` per frame counts frames. It would add 30 units per second at
30 FPS and 120 at 120 FPS. To accumulate seconds, add each frame's measured
duration in seconds.

### 6. Assignment instead of accumulation

`elapsed = clock` replaces the previous total with the latest frame duration.
The display would show a single frame's duration while running, rather than
total running time or time since the last pause. When paused, that stored value
would freeze because the update is skipped.

### 7. Running time across a pause

The result is **4.02 seconds**: `4.00 + 0.02`. The 10-second pause contributes
nothing because the application skips the update while paused. The previous
four seconds remain in the total; pausing does not reset them.

### 8. The addition assignment operator

For this numeric field, `elapsed += clock` means the same update as
`elapsed = elapsed + clock`: take the stored total, add the new duration,
and store the result back in `elapsed`.

### 9. Different frame rates

Both clocks should accumulate approximately one second per real second of
running. At a steady 30 FPS, each frame contributes about `1/30` second; at
120 FPS, about `1/120` second. Smaller contributions happen more often.
Higher FPS gives more frequent display updates; it does not automatically
make time measurements more accurate.

### 10. Converting the frame duration

`get_frame_time()` returns an `f32` duration that is already in seconds.
`f64::from(...)` converts its numeric representation to `f64`, matching
`Simulation::elapsed` and the `step()` parameter. It does not convert frames
into seconds or recover precision missing from the original `f32` value.

### 11. Nested defaults

`Simulation::default()` initializes its star using `Star::default()`, which
returns a star with `radius = 1.0`. It also sets `elapsed = 0.0` and
`paused = true`. The R handler replaces the entire simulation with this new
default value, restoring all three fields together.

### 12. Calling step directly while paused

Yes. Calling `simulation.step(0.1)` adds `0.1` to the existing elapsed time
because the method itself does not inspect `paused`. In the normal application
loop, `if !simulation.paused` controls whether the method is called. That is
the difference between the caller's condition and the method's own behavior.

Write `step(0.1)` with a decimal point. `step(0,1)` would supply two arguments
and would not match this method's signature.

## Study questions

### 13. Associated functions and receivers

`Star::default()` is an associated function called through the type. Its
definition has no `self` parameter and creates a new value without an existing
star. `star.default()` attempts method-call syntax on a value, which is not
how this function is defined. In a field initializer, `star:` names the field;
it does not create a variable to call methods on.

`Self` is the type of the implementation, such as `Star` or `Simulation`.
Lowercase `self` is the receiver representing the existing instance on which
a method is called.

### 14. Trait implementations and methods on a type

`impl Default for Simulation` implements the standard `Default` trait. Its
`default()` function returns a new `Simulation`, called as
`Simulation::default()`.

`impl Simulation` adds methods directly to the type. Its `step(&mut self,
dt: f64)` method borrows an existing simulation mutably, changes its state,
and is called as `simulation.step(dt)`. It has no explicit return value;
its return type is the unit type, `()`.

### 15. Field visibility

Making a struct public exposes the type, while its fields remain private by
default. Code in `main.rs` and `view.rs` needs permitted access to fields
defined in the sibling `simulation` module. `pub(crate)` permits access within
this crate and is sufficient for that arrangement; `pub` is broader.

Public fields are a design choice. As the model gains rules about valid state,
private fields with methods or getters may help enforce those rules.

### 16. Choosing a clock abstraction

The initial clock only needs the existing `elapsed` and `paused` fields.
That keeps the first behavior small and easy to inspect. A separate clock or
scheduler type may become useful if timekeeping gains related state and
behavior, such as playback speed or an accumulator for fixed timesteps.
Introduce that type when its responsibilities justify it.

### 17. Independent conditions and pattern matching

Separate `if` statements allow more than one key handler to run in a frame.
With Space handled before R, a simultaneous reset restores the initial paused
state after the toggle. A `match` executes one matching arm and is useful
when choosing behavior from one value, especially an enum such as a command
or simulation phase. The current independent key checks are straightforward
with `if` statements.

### 18. Text content and drawing arguments

`format!` constructs the text. `{:.2}` displays a numeric value with two digits
after the decimal point; it does not change the stored value. Ordinary string
literals do not substitute variable names in braces by themselves.

In `draw_text(text, x, y, font_size, color)`, the other arguments choose the
horizontal position, vertical text baseline, font size, and color. The current
Macroquad version accepts text implementing `AsRef<str>`, including a string
literal, an owned `String`, or a borrowed `String`.
