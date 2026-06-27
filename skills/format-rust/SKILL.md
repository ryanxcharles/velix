---
name: format-rust
description:
  "Format Rust files with cargo fmt. Use after creating or editing any Rust file
  (.rs)."
---

# Format Rust

Run `cargo fmt` after creating or editing Rust files.

## When This Skill Applies

After every write or edit to any `.rs` file in the project. This includes all
Helix/Velix crates such as `helix-core`, `helix-term`, `helix-view`, and
`xtask`.

## Process

After your edits are complete, run:

```bash
cargo fmt
```

Accept the formatter output as the source of truth.
