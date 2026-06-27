+++
status = "open"
opened = "2026-06-27"
+++

# Issue 3: Default Tokyo Night theme

## Goal

Make Tokyo Night the default Velix theme. Place the provided Tokyo Night theme
file in the appropriate runtime location, then change the editor's default theme
selection so a user with no explicit theme configuration starts in Tokyo Night.

## Background

The repository currently has an untracked `tokyonight.toml` at the repository
root. The runtime theme directory already contains Tokyo Night variants:

- `runtime/themes/tokyonight.toml`
- `runtime/themes/tokyonight_day.toml`
- `runtime/themes/tokyonight_moon.toml`
- `runtime/themes/tokyonight_storm.toml`

Initial inspection suggests the root `tokyonight.toml` matches the existing
`runtime/themes/tokyonight.toml` body, except the runtime file includes an
author comment. The implementation experiment should verify whether the provided
root file should replace, update, or simply be removed in favor of the existing
runtime theme.

Helix's built-in default theme path appears to live in
`helix-view/src/theme.rs`. That file embeds `theme.toml` and exposes
`DEFAULT_THEME`, `Loader::default_theme`, and `Loader::default`. Application
startup in `helix-term/src/application.rs` falls back to
`editor.theme_loader.default_theme(true_color)` when no user theme is
configured.

The user-facing theme documentation is in `book/src/themes.md`. If the default
theme changes user-visible behavior, documentation should be updated where
appropriate.

## Analysis

This issue should be solved with one experiment unless inspection finds that the
theme file itself needs substantial correction.

The experiment should answer:

- Where should the provided `tokyonight.toml` live?
- Is `runtime/themes/tokyonight.toml` already the intended file, or should the
  root copy replace it?
- Should the default change be implemented by changing the embedded
  `theme.toml`, changing `Loader::default_theme`, or changing the application
  fallback to load `tokyonight` by name?
- How should fallback behave if true color is unavailable?
- What tests or small executable checks prove that no-config startup resolves to
  Tokyo Night while explicit user `theme = "..."` configuration still wins?

## Proposed Solutions

Prefer the smallest implementation that keeps Helix's theme loading model
coherent:

- Put the Tokyo Night file under `runtime/themes/` if the provided file is not
  already represented there.
- Avoid leaving duplicate theme files at the repository root.
- Preserve explicit user theme configuration.
- Keep a sensible non-true-color fallback if the existing default-theme logic
  needs one.
- Add focused tests around default theme resolution if the local code exposes a
  testable seam.

Do not modify closed issues. Do not change theme behavior before an experiment
is designed, reviewed, approved, and committed under the issue workflow.

## Experiments

- [Experiment 1: Use Tokyo Night as the true-color default](01-use-tokyonight-default.md) -
  **Designed**
