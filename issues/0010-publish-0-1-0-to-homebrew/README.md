+++
status = "open"
opened = "2026-06-29"
workflow = "issues-and-experiments"
review_mode = "external-claude"
review_routing = "orthogonal-review"
+++

# Issue 10: Publish 0.1.0 to Homebrew

## Goal

Publish the first Velix release, version `0.1.0`, through a Homebrew tap owned
by `astrohackerlabs`.

Users should be able to install Velix with Homebrew and run the `vlx` binary
without needing to know or track an upstream Helix version.

## Background

TermSurf publishes through a separate Homebrew tap checkout and a release script
that packages artifacts, uploads a GitHub Release, updates the tap, pushes the
tap, and verifies installation. Velix should follow the same release discipline
but does not need TermSurf's private/public source split.

Velix should use only two repositories:

- source repo: `github.com/astrohackerlabs/velix`;
- Homebrew tap repo: `github.com/astrohackerlabs/homebrew-velix`.

Current local state at issue creation:

- `astrohackerlabs/velix` exists on GitHub and the local checkout has it as the
  `upstream` remote;
- the local checkout's `origin` remote still points at
  `git@github.com:ryanxcharles/velix.git`;
- `astrohackerlabs/homebrew-velix` does not exist on GitHub;
- `~/dev/homebrew-velix` does not exist locally;
- the shipped terminal binary is `vlx` from `helix-term`.

The release must not encode or advertise an upstream Helix version. The first
Velix version is `0.1.0`.

## Analysis

Velix is a command-line Rust application with runtime assets, not a macOS app
bundle. The Homebrew packaging should therefore use a formula in
`homebrew-velix`, not a cask like TermSurf.

The release work needs to decide and implement the package contract. A likely
shape is:

- tag `astrohackerlabs/velix` as `v0.1.0`;
- create a GitHub Release on `astrohackerlabs/velix`;
- publish a source-based or prebuilt release artifact suitable for Homebrew;
- create `~/dev/homebrew-velix` and the GitHub repo
  `astrohackerlabs/homebrew-velix`;
- add `Formula/velix.rb` to the tap;
- install the `vlx` binary;
- install or otherwise make available the Velix runtime assets required by
  `vlx`;
- verify installation from the tap in a clean Homebrew flow.

The implementation should inspect Helix's existing packaging guidance before
choosing the formula details, especially runtime asset handling. It should also
inspect TermSurf's release flow for useful guardrails: package-only validation
before publish, SHA updates, clean tap checkout checks, explicit publish mode,
and post-publish smoke tests.

Additional findings before Experiment 1:

- Homebrew's upstream Helix formula is source-based, sets
  `HELIX_DEFAULT_RUNTIME` at build time, runs `cargo install` with
  `path: "helix-term"`, installs `runtime` under `libexec`, and verifies with
  `hx --health`.
- Velix currently inherits workspace version `25.7.1`, and
  `helix-loader/build.rs` formats versions using Helix calver assumptions. A
  literal workspace version `0.1.0` would currently print as `0.01`, so release
  identity must be fixed before publishing.
- Runtime grammars are not tracked in git. A source archive containing only
  committed files would include themes and queries but no built grammars, so the
  package contract must explicitly include a grammar strategy before publishing.
- Publication should be split from local/package-only validation so a broken
  `v0.1.0` tag and tap formula are not made public before install behavior is
  proven.

## Proposed Solution

Design and implement the release in experiments, one step at a time:

- prepare Velix version identity for `0.1.0` without tracking a Helix version;
- add a Velix release script or documented release procedure, adapted from the
  TermSurf flow but simplified for one public source repo and one Homebrew tap;
- create the missing local and GitHub tap repository: `~/dev/homebrew-velix` and
  `astrohackerlabs/homebrew-velix`;
- add the initial Homebrew formula for `velix`;
- publish `v0.1.0` from `astrohackerlabs/velix`;
- update and push the tap;
- verify `brew tap astrohackerlabs/velix` or the correct tap command,
  `brew install velix`, and `vlx --version`;
- record exact install, upgrade, uninstall, and smoke-test evidence.

## Experiments

- [Experiment 1: Prepare local 0.1.0 Homebrew release](exp-0001-publish-0-1-0-homebrew.md)
  - **Designed**

## Constraints

- Do not create a separate closed-source or public mirror repository for Velix.
- Do not track or expose a Helix version as the Velix release version.
- Keep the release target at `0.1.0` unless the user explicitly changes it.
- Keep Homebrew work under `astrohackerlabs/homebrew-velix`.
- Do not publish to GitHub or push the Homebrew tap until the experiment design
  for that publish step has been reviewed and approved.
- Prefer repeatable scripts over ad hoc release commands when the command
  sequence becomes more than a one-off.

## Verification

This issue is complete when:

- `astrohackerlabs/velix` has a `v0.1.0` tag and GitHub Release;
- `astrohackerlabs/homebrew-velix` exists on GitHub;
- `~/dev/homebrew-velix` exists locally with a clean git status;
- the tap contains a committed `Formula/velix.rb`;
- the formula installs `vlx` and required runtime assets;
- Homebrew install from the tap succeeds on the target machine;
- `vlx --version` reports Velix `0.1.0` or another deliberately chosen
  Velix-specific version string for the release;
- a basic editor launch smoke test passes from the Homebrew-installed binary;
- uninstall or cleanup behavior is documented or verified;
- the issue records the release URLs, tap commit, installed binary path, and
  verification commands.
