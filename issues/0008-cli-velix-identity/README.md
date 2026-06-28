+++
status = "closed"
opened = "2026-06-28"
closed = "2026-06-28"
+++

# Issue 8: CLI Velix identity

## Goal

Update CLI-facing identity text so the `vlx` binary reports itself as Velix
rather than Helix.

## Background

Issue 7 renamed the primary executable to `vlx`, but it intentionally focused on
the binary entrypoint and related packaging/docs. After that rename,
`target/release/vlx --version` still prints `helix 25.07.1 (...)`, and
`target/release/vlx --help` still starts with package identity text such as
`helix-term` and the upstream description.

This is confusing because users invoking `vlx` should see Velix-branded CLI
identity text, even if internal crate names and inherited Helix architecture
remain unchanged.

Known stale CLI-facing strings include:

- `helix-term/src/main.rs` hard-coding `helix` in `--version` output.
- `helix-term/src/main.rs` using `CARGO_PKG_NAME`, currently `helix-term`, in
  the help header.
- `helix-term/src/main.rs` reporting `unable to start Helix` in startup error
  context.
- `helix-term/Cargo.toml` package metadata still describing the terminal package
  with upstream Helix-era identity text.

## Analysis

The fix should distinguish user-facing CLI identity from internal crate and
module naming. Renaming every `helix-*` crate is out of scope; those names are
large architectural identifiers inherited from Helix. The first slice should
only change strings that are emitted by the `vlx` binary or used directly to
generate that emitted CLI identity.

Verification should run the built `vlx` binary and inspect its `--version` and
`--help` output. The issue should not be considered solved while those outputs
still identify the CLI as `helix` or `helix-term`.

## Proposed Solutions

Introduce explicit Velix-facing CLI identity strings where needed instead of
relying blindly on inherited Cargo package names. Keep internal crate names
unchanged unless an experiment proves they directly leak into user-facing CLI
output and cannot be overridden cleanly.

## Experiments

- [Experiment 1: Replace CLI identity strings](01-replace-cli-identity-strings.md) -
  **Pass**

## Conclusion

Issue 8 is complete. The `vlx` binary now reports Velix identity in `--version`
and `--help`, no longer prints `helix`, `helix-term`, or the inherited upstream
description in those CLI identity paths, and keeps the `vlx [FLAGS] [files]...`
usage line from Issue 7.

Internal `helix-*` crate/module names and compatibility surfaces remain
unchanged because they are not the CLI-facing product identity.
