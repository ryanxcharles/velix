# Experiment 1: Make Vim profile the default

## Description

Velix currently defines a Vim keymap profile, but `Config::default()` and config
loading without `editor.keymap` still use the Helix-style default keymap. That
means Vim-profile fixes such as bare `G` going to the end of the document do not
work in a fresh Velix run.

This experiment will make the Vim profile the default base keymap whenever no
explicit profile is configured, while preserving explicit
`editor.keymap = "default"` as the escape hatch for the Helix-style keymap.

## Changes

- `helix-term/src/config.rs`
  - Change `Config::default()` to use `keymap::vim()`.
  - Change missing-profile fallbacks in `Config::load` from
    `KeymapProfile::Default` to `KeymapProfile::Vim`.
  - Update config tests so empty config and `Config::default()` resolve to the
    Vim profile.
  - Add or update tests proving explicit `keymap = "default"` still resolves to
    the Helix-style keymap, local profile still overrides global profile, and
    user `[keys]` remaps still merge over the default Vim profile.
- `helix-term/tests/test/commands/vim_profile.rs`
  - Update the default-keymap preservation test so the no-config case now proves
    bare `G` uses Vim behavior out of the box.
  - Keep explicit `keymap = "default"` coverage for Helix-style behavior where
    needed.
- `book/src/vim-profile.md`
  - Update the wording from opt-in Vim profile to default Vim profile.
  - Document `keymap = "default"` as the way to choose the Helix-style keymap.

## Verification

- Run `cargo fmt`.
- Run `cargo test -p helix-term config::tests:: --lib`.
- Run
  `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`.
- Run `prettier --write --prose-wrap always --print-width 80` on changed
  Markdown files.

Pass criteria:

- Empty config and `Config::default()` use `keymap::vim()`.
- Explicit `editor.keymap = "default"` still uses `keymap::default()`.
- Local profile selection still overrides global profile selection.
- User key remaps still merge over the default Vim profile.
- A no-config integration test proves bare `G` goes to the end of the document.
- Documentation clearly says Vim is the default and Helix-style keys are
  selected with `keymap = "default"`.

## Design Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer reported no Required findings. It independently checked that the
issue README links this experiment as Designed, the experiment has the required
sections and pass criteria, the scope is narrow, the technical plan matches the
current config architecture, and the verification covers empty/default config,
explicit `default`, local/global precedence, user key merges, and no-config `G`
behavior. It also ran read-only checks: `cargo fmt --check`,
`cargo test -p helix-term config::tests:: --lib --no-run`, and
`cargo test -p helix-term --features integration --test integration vim_profile -- --list`.
