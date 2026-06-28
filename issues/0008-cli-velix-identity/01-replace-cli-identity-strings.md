# Experiment 1: Replace CLI identity strings

## Description

Change the `vlx` binary's user-visible identity from inherited Helix strings to
Velix strings. This experiment targets only text emitted by the CLI itself or
metadata directly used to generate that text.

Internal crate names such as `helix-term` remain out of scope unless they are
printed directly to users. Runtime/config directory names and environment
variables such as `HELIX_RUNTIME` also remain out of scope; they are inherited
compatibility surfaces rather than the CLI's product identity.

## Changes

- Update `helix-term/src/main.rs` so `--version` prints `velix` instead of
  `helix`.
- Update `helix-term/src/main.rs` so `--help` starts with explicit Velix-facing
  text instead of `CARGO_PKG_NAME` / `CARGO_PKG_DESCRIPTION` values inherited
  from `helix-term`.
- Update `helix-term/src/main.rs` startup error context from
  `unable to start Helix` to Velix wording.
- Update `helix-term/Cargo.toml` package description only if the implementation
  still uses package metadata for CLI-facing text after the source change.

## Verification

- Run `cargo fmt`.
- Run `cargo build -p helix-term --release --locked`.
- Run `target/release/vlx --version` and verify it prints `velix` rather than
  `helix`.
- Run `target/release/vlx --help` and verify the header identifies Velix, does
  not identify the binary as `helix-term`, no longer prints the inherited
  upstream description `A post-modern text editor.`, and still shows
  `vlx [FLAGS] [files]...` in the usage line.
- Search `helix-term/src/main.rs` for stale CLI-facing `helix`, `Helix`,
  `helix-term`, and `CARGO_PKG_NAME` / `CARGO_PKG_DESCRIPTION` usage in help,
  version, and startup error paths. Remaining matches must be non-user-facing or
  explicitly documented.

## Design Review

**Reviewer:** separate Codex adversarial-review agent

**Verdict:** Changes required

The reviewer found that the initial verification did not explicitly reject the
inherited upstream description, even though the issue identified it as part of
the stale CLI identity. The verification was updated to require that
`target/release/vlx --help` no longer prints `A post-modern text editor.`.

The same reviewer re-reviewed the revised design and approved it with no
findings.
