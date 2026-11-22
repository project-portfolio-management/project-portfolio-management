# Arithmetic

This specification states how the service, the domain crate, the matcher and the date and time crate do arithmetic:
math must not panic, and it must not wrap. A calculation that cannot be done says so, as a value the caller has to
handle.

## Requirements

1. **No panic from arithmetic.** Production code does not panic because of the numbers it handles: not on overflow, not
   on underflow, not on division by zero, not on an out-of-range date or duration, not on an index past the end.
2. **No silent wrap.** A release build wraps an overflowing `+` without a sound, and a debug build panics on it. Both
   are the same defect. Production code does neither.
3. **Use the `checked_*` functions.** Integer arithmetic goes through `checked_add`, `checked_sub`, `checked_mul`,
   `checked_div`, `checked_rem`, `checked_neg`, `checked_abs`, `checked_pow`, `checked_shl`, `checked_shr` and their
   relatives, never through the bare operators or `abs`, `pow`, `-x` on a value that can reach the edge of its type.
4. **Return `None` (or an error), do not panic or wrap.** Each `checked_*` function returns `Option<T>`; the result is
   `None` where the operation does not fit. For example, `255_u8.checked_add(1)` is `None`, where `255_u8 + 1` panics in
   a debug build and gives `0` in a release build.
5. **The caller decides what `None` means.** A function whose answer can be missing returns `Option<T>` or
   `Result<T, E>` and passes the failure on. A request handler turns it into a stated error response (for the numbers a
   caller sent, a `400` or `422` naming the field; for a number the service computed, a `500` and a log line), never a
   dropped connection. A figure that cannot be computed is shown as "not computed, and why", the project's rule for every
   figure (see [`../07-non-functional-requirements.md`](../07-non-functional-requirements.md)).
6. **No default that hides the failure.** `checked_add(..).unwrap_or(0)`, `saturating_*` and `wrapping_*` change a wrong
   answer into a plausible one. They are allowed only where the clamp is the stated meaning of the value (a progress
   bar held to 0–100, a retry count capped), with a comment that says so.
7. **Dates and durations follow the same rule.** Adding a duration, a number of days or a number of months to a date or
   an instant returns `Option` (`checked_add_signed`, `checked_add_days`, `checked_add_months` and the like). The
   panicking operators (`date + duration`) and constructors (`Duration::days(n)` past the range) do not exist in the
   date and time crate; the `try_*` constructors do.
8. **Conversions do not truncate.** A narrowing conversion uses `try_from` / `try_into` (and handles the `Err`), not
   `as`, which cuts the high bits without a sound. `as` stays for widening, for float-to-integer where the range is
   checked first, and for a value the code has just bounded.
9. **Floating point.** A float result that is `NaN` or infinite is a failure, not a figure: the function returns `None`
   (or an error) rather than carrying the value on. Division by a zero or near-zero divisor is checked before it is done.
10. **Money and time-valued figures** (costs, earned value, durations, dates) are the first place this applies, because a
    wrong figure there is reported as a fact.

## What is not covered

- **Test code.** Tests may use `unwrap`, `expect`, the bare operators and indexing; a panic there fails a test, which is
  what it is for.
- **Build scripts.** A `build.rs` that cannot read its inputs fails the build; it does not run in production. (They still
  return an error from `main`, not a panic, where that costs nothing.)
- **Proven bounds.** A bare operator or index on a value the code has just bounded is permitted only with a one-line
  comment giving the proof, and an `#[allow(clippy::…)]` on that statement, never on a module.

## Enforcement

- **Lints.** The service, domain, date and time and migration crates deny, for non-test code, `clippy::unwrap_used`,
  `clippy::expect_used`, `clippy::panic`, `clippy::unreachable`, `clippy::todo` and `clippy::unimplemented`, and then
  `clippy::indexing_slicing` and `clippy::arithmetic_side_effects`. The matcher already has no site that the first set
  and `indexing_slicing` find.
- **Tests.** A `checked_*` result of `None` has a test that asserts it for each function that can return it (the edge of
  the type: `u8::MAX + 1`, `i64::MIN / -1`, a date past the last one the type holds, a divisor of zero).
- **A panic that gets through** must not drop the connection: the service answers a request whose handler panicked with a
  `500` and logs it, so one bad request cannot take a worker down silently. This is a backstop, not permission to
  panic.

## Status

Stated honestly, as of 2026-11-22: **this is not yet true of the code.** A clippy run over the service workspace's
non-test code finds about 90 `unwrap`, `expect`, `panic!` and `unreachable!` sites and about 480 places that index or
slice, mostly in the service and domain crates; 27 of the first group are the date and time crate's own operators and
constructors. They are removed in steps, and a lint is denied for a crate only once that crate has none left. The
[CHANGELOG](../../CHANGELOG.md) of each project says which steps are done and what is not verified.

## Example

```rust
// Panics in a debug build, wraps to 0 in a release build.
let wrong = 255_u8 + 1;

// None: the caller has to say what an overflow means.
let right: Option<u8> = 255_u8.checked_add(1);
assert_eq!(right, None);

// A date that does not exist is None, not a crash.
let next = date.checked_add_days(Days::new(days));
```
