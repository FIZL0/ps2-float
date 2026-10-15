# Agent notes

`ps2-float`: the PS2's arithmetic (EE FPU, VU0, fdlibm, newlib `rand`/MT19937), bit for bit. Used by
opentennis (`../opentennis`, FIZL0/opentennis), FIZL0/ps2-decomp-kit-linux, FIZL0/ps2-decomp-kit-windows and
FIZL0/ps2-oracle through tag-pinned git dependencies.

- **These libraries are ours.** Agents may fix and extend them when a port needs it (new op, missing function,
  wrong rounding, another SDK's libm flavour): add a test, tag a release (bump `version` in `Cargo.toml`, tag
  `vX.Y.Z`, push the tag), then bump the tag in every dependent above and run their tests. Never patch a vendored
  copy in a dependent.
- No dependencies (only the optional `serde` feature), `no_std`, no `unsafe`, no host transcendentals
  (`f32::sin`, `sqrt` …): results must be identical on every host. `#![deny(clippy::float_arithmetic)]` in each
  model module; host floats only inside the model's own primitives, each `#[allow]` with a reason.
- A behaviour change to an existing function changes every dependent's bit-exact results: prove it (a recording or
  an oracle run in the game that motivated it), say which in the doc comment, and make it a minor version bump.
- Game data never goes in here: constants are the library's own (fdlibm's), described by which build they match.
- Checks: `cargo test`, `cargo clippy --all-targets --features serde -- -D warnings`,
  `cargo build --target wasm32-unknown-unknown`.
- Git identity: FIZL0 only (user + credential helper pinned in `.git/config`).
