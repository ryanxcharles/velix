+++
status = "open"
opened = "2026-06-27"
+++

# Issue 5: Default Vim keymap

## Goal

Make the Vim keymap profile the default keymap for Velix when no explicit
`editor.keymap` is configured, while preserving an explicit escape hatch for the
Helix-style default keymap.

## Background

Issue 4 completed the Vim and LazyVim keybinding audit and confirmed that
Vim-specific behavior such as bare `G` going to the end of the document only
works when the Vim profile is selected:

```toml
[editor]
keymap = "vim"
```

That opt-in behavior is confusing for Velix because the fork's purpose is to
offer Vim-style keybindings on top of Helix architecture. A fresh Velix release
should therefore start with the Vim profile unless the user or a workspace
explicitly asks for the Helix-style default profile.

The relevant implementation surface is expected to include:

- `helix-term/src/config.rs`, where missing `editor.keymap` currently falls back
  to `KeymapProfile::Default`.
- `helix-term/src/keymap/default.rs`, where `default()` and `vim()` keymaps are
  defined.
- `helix-term/tests/test/commands/vim_profile.rs`, which verifies Vim-profile
  behavior such as `G`.
- `helix-term/src/config.rs` tests, which already cover profile selection,
  default fallback, local/global precedence, and user key merges.
- `book/src/vim-profile.md` and related configuration docs, which currently
  describe the Vim profile as opt-in.

## Analysis

The change should make Vim the default only when no profile is configured. It
should not remove the Helix-style keymap or prevent users from selecting it
explicitly:

```toml
[editor]
keymap = "default"
```

The likely implementation is to change the keymap fallback in
`helix-term/src/config.rs` from `KeymapProfile::Default` to
`KeymapProfile::Vim`, and to update `Config::default()` so tests and
programmatic default config construction use `keymap::vim()`.

The risky part is preserving precedence:

- local `editor.keymap` should still override global `editor.keymap`;
- explicit `keymap = "default"` should still load the Helix-style keymap;
- user `[keys]` remaps should still merge over the selected base profile;
- tests that intentionally assert pre-profile Helix behavior should either
  request `keymap = "default"` or be updated to the new default expectation.

## Proposed Solutions

Start with one experiment that changes the default profile selection and updates
the focused config/keymap tests and documentation. The experiment should prove
that a fresh config now uses the Vim profile, that explicit `keymap = "default"`
still works, and that `G` works out of the box under the new default.

Do not remove the Helix-style keymap. The compatibility goal is changing the
Velix default, not eliminating Helix behavior.

## Experiments

- [Experiment 1: Make Vim profile the default](01-make-vim-profile-default.md) -
  **Pass**

## Constraints

- Preserve the issue workflow: design one experiment, review it, commit the
  plan, implement and verify it, completion-review it, and commit the result.
- Do not list experiments upfront.
- Do not modify closed issues.
- Keep the change narrowly scoped to default profile selection, tests, and docs.
- Run `cargo fmt` after Rust edits.
