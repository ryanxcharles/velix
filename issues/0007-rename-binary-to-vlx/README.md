+++
status = "open"
opened = "2026-06-28"
+++

# Issue 7: Rename binary to vlx

## Goal

Rename the Velix command-line binary to `vlx`, and make the build, install, and
developer documentation consistently describe `vlx` as the primary executable.

## Background

Velix is a fork of Helix, whose editor binary is `hx`. A short Velix binary name
keeps the terminal workflow close to Helix while giving the fork a distinct
command. Prior discussion considered `vx`, `vlx`, and `velix`; `vx` already has
more visible CLI collisions, while `vlx` is acceptable despite some existing
package-name noise.

The rename should be treated as a packaging and user-entrypoint change, not as a
modal-editing behavior change. The implementation needs to preserve the ability
to build and run Velix in release mode, update references that tell users which
binary to invoke, and verify that the produced release binary is named `vlx`.

## Analysis

The likely implementation surface includes Cargo package metadata, binary target
configuration, build/install documentation, and any scripts or tests that refer
to the current executable name. The audit should distinguish the project name
`Velix` from the executable name `vlx`: the project should remain Velix, while
the primary terminal command becomes `vlx`.

The first experiment should locate every authoritative binary-name reference in
the repository, identify the minimal rename path, and then update the build
configuration and docs together. Verification should include a release build or
an equivalent Cargo target check that proves `target/release/vlx` is produced
and that stale binary-name instructions are not left in user-facing docs.

## Proposed Solutions

Use `vlx` as the primary executable name. Keep `velix` only if the experiment
finds a concrete compatibility reason for a long alias; otherwise prefer one
clear binary name to avoid documenting multiple entrypoints prematurely.

## Experiments

- [Experiment 1: Rename the terminal executable](01-rename-terminal-executable.md) -
  **Designed**
