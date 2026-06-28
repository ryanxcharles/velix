# Experiment 1: Rename the terminal executable

## Description

Rename the primary Velix terminal executable from `hx` to `vlx`. This experiment
keeps the project and crate names intact, but changes the command users build,
install, invoke, complete, and launch from desktop metadata.

The experiment is intentionally limited to authoritative repository-owned
entrypoints and documentation. Vendored grammar source documentation and
upstream package-manager notes may still mention Helix or `hx` where they are
describing upstream Helix packages or third-party language projects rather than
the Velix binary.

## Changes

- Update `helix-term/Cargo.toml` so Cargo builds and runs the terminal binary as
  `vlx`, and Debian package assets install `vlx` launcher and shell completion
  files.
- Update `helix-term/src/main.rs` so CLI help displays `vlx` in the usage line.
- Rename the launcher and all shell completion files under `contrib/` from `hx`
  to `vlx`, including Bash, Fish, Zsh, Elvish, and Nushell completions, and
  update their command names and internal command invocations.
- Update desktop and appstream metadata under `contrib/` so the advertised
  executable is `vlx`.
- Update Nix packaging metadata in `default.nix` so shell completions and
  `meta.mainProgram` point at `vlx`.
- Update repository-owned build, install, usage, health-check, language-server,
  language-grammar, runtime tutor, and contributor documentation that tells
  users or developers to invoke the Velix editor binary.
- Leave broader Helix project identity, internal crate names, config directory
  names, runtime environment variables, and upstream-package references alone
  unless they directly claim the Velix executable is `hx`.

## Verification

- Run `cargo fmt`.
- Run `prettier --write --prose-wrap always --print-width 80` on edited Markdown
  files.
- Run `cargo build -p helix-term --release --locked` and verify
  `target/release/vlx` exists and is executable.
- Run `target/release/vlx --version` to verify the renamed binary launches.
- Run `target/release/vlx --help` and verify the usage line says
  `vlx [FLAGS] [files]...`, with no stale `hx [FLAGS]` usage.
- Run `cargo metadata --no-deps --format-version 1` or an equivalent Cargo
  manifest check to verify the `helix-term` package binary target is named `vlx`
  and no `hx` binary target remains in that package.
- Search repository-owned binary-entrypoint surfaces for stale `hx` executable
  references, including `book/`, `docs/`, `runtime/tutor`, `contrib/`,
  `default.nix`, `helix-term/src/main.rs`, and `helix-term/Cargo.toml`. Any
  remaining `hx` matches must be either outside this experiment's authoritative
  surface or explicitly documented as upstream Helix/vendor material.

## Design Review

**Reviewer:** separate Codex adversarial-review agent

**Verdict:** Changes required

The reviewer found that the initial design missed the hard-coded CLI help usage
string in `helix-term/src/main.rs`, repository-owned user-facing invocation docs
and `runtime/tutor`, Nix packaging metadata in `default.nix`, and Nushell
completion coverage. The design was updated to include those files and to verify
`vlx --help`, Nix metadata, all `contrib/completion/hx.*` files, and a broader
stale-`hx` search over the authoritative binary-entrypoint surface.

The same reviewer re-reviewed the revised design and approved it with no
required findings remaining.

## Result

**Result:** Pass

The terminal executable was renamed to `vlx` across the Cargo binary target,
Cargo default run target, Debian package assets, launcher, shell completions,
desktop/appstream metadata, Nix package metadata, CLI help usage, user-facing
build/install/usage docs, contributor docs, and runtime tutor text.

Verification completed:

- `cargo fmt` and `cargo fmt --check` passed.
- `prettier --write` and `prettier --check --prose-wrap always --print-width 80`
  passed for edited Markdown files.
- `cargo build -p helix-term --release --locked` passed.
- `test -x target/release/vlx` passed; `target/release/vlx` exists and is
  executable.
- `target/release/vlx --version` launched successfully.
- `target/release/vlx --help` shows `vlx [FLAGS] [files]...` in the usage line.
- `cargo metadata --no-deps --format-version 1` reports `default_run = "vlx"`
  and the `helix-term` binary target list as `["vlx"]`.
- `cargo run -p helix-term -- --version` runs `target/debug/vlx --version`,
  confirming Cargo's default-run path.
- The stale-`hx` search over `book/`, `docs/`, `runtime/tutor`, `contrib/`,
  `default.nix`, `helix-loader/src/grammar.rs`, `helix-term/src/main.rs`, and
  `helix-term/src/logging.rs`, and `helix-term/Cargo.toml` found only upstream
  package-manager references and the `lite-modal-hx` plugin name, not Velix
  binary-entrypoint references.

## Conclusion

`vlx` is now the primary Velix executable name. The remaining `hx` references in
the searched surface are preserved because they describe upstream Helix package
manager behavior or an external plugin name, not the Velix binary.

## Completion Review

**Reviewer:** separate Codex adversarial-review agent

**Verdict:** Approved

The reviewer initially found one required stale runtime message in
`helix-loader/src/grammar.rs`, where grammar error text still suggested
`hx --grammar fetch`, and one optional stale source comment in
`helix-term/src/logging.rs`. Both were fixed. The reviewer re-reviewed the
result, independently verified the fixes and stale-reference search, and
approved the completed experiment with no required findings remaining.
