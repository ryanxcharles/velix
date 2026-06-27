# Experiment 1: Selectable Vim profile

## Description

Add the first usable opt-in Vim-oriented keybinding profile without changing the
default Helix-style keymap. The profile should prove the architecture
recommended by Issue 1: simple Vim and LazyVim-compatible bindings are ordinary
keymap/profile data, while deeper Vim grammar remains explicit and deferred
unless this experiment needs a tiny linewise operator command.

This experiment intentionally targets a narrow but user-visible vertical slice:

- users can select the profile from `config.toml`;
- the default keymap remains unchanged unless selected;
- representative standalone Vim motions and common LazyVim workflow aliases work
  in the profile;
- `dd`, `yy`, and `cc` work as linewise operator shortcuts closely enough for a
  first usable profile;
- user-facing documentation states what is Vim-like, what is LazyVim-like, what
  keeps Helix semantics, and what is deferred.

## Changes

Expected source changes:

- `helix-term/src/config.rs`
  - Add a small keymap-profile configuration field, using the least invasive
    shape found during implementation.
  - Prefer `editor.keymap = "default" | "vim"` if it can be added without
    leaking Velix-only profile data into unrelated editor configuration.
  - Select the base keymap before merging global and workspace `[keys]`
    overrides, so user remaps still work on top of either profile.
  - Reject unknown profile names with a clear TOML/config error.
- `helix-term/src/keymap/default.rs`
  - Keep `default()` semantically unchanged.
  - Add a separate Vim profile constructor, likely by starting from `default()`
    and merging a focused Vim-profile delta.
  - Include representative standalone Vim mappings: `0`, `^`, `$`, `gg`, `G`,
    `h`, `j`, `k`, `l`, `w`, `b`, `e`, `i`, `a`, `I`, `A`, `o`, `O`, `/`, `?`,
    `n`, `N`, `u`, `C-r`, `p`, and `P` where existing commands are close.
  - Include LazyVim-style workflow aliases that have direct Helix equivalents,
    especially finder/search, buffer navigation/picker, diagnostics picker, LSP
    goto/actions/symbols, git change navigation/changed files, and window
    management.
  - Add `dd`, `yy`, and `cc` as command sequences if tests prove the existing
    selection commands are sufficient.
- `helix-term/src/commands.rs`
  - Add dedicated `vim_delete_line`, `vim_yank_line`, and `vim_change_line`
    commands only if command sequences cannot preserve acceptable linewise
    behavior for `dd`, `yy`, and `cc`.
- `helix-term/tests/test/helpers.rs`
  - Adjust or extend config helpers only if the existing default-key merge makes
    profile-specific integration tests impossible.
- `helix-term/tests/test/commands/`
  - Add focused integration tests proving profile selection, default
    preservation, representative Vim-profile mappings, user remap layering, and
    `dd`/`yy`/`cc`.
- `book/src/`
  - Add user-facing documentation for the Vim profile.
  - Link it from `book/src/SUMMARY.md`.
  - Document the implemented Vim-like bindings, LazyVim-like aliases,
    intentional Helix semantic differences, and deferred grammar work.

Do not modify the closed Issue 1 records.

## Verification

This experiment passes when all of the following are true:

1. `Config::default()` and loading an empty config still produce the exact
   default Helix-style keymap.
2. A config that selects the Vim profile produces a different profile keymap,
   and an unknown profile name fails config loading with a clear error.
3. User `[keys]` overrides merge on top of the selected profile, not only on top
   of the default profile.
4. Integration tests prove at least these profile-specific behaviors:
   - default profile behavior is unchanged for a key that the Vim profile
     changes;
   - Vim profile `0`, `^`, `$`, `gg`, and `G` map to the intended line/file
     motion commands;
   - Vim profile `C-r` maps redo;
   - LazyVim-style finder/search aliases map to existing Helix picker/search
     commands;
   - LazyVim-style buffer navigation/picker aliases map to existing Helix buffer
     commands;
   - LazyVim-style diagnostic aliases map to existing Helix diagnostic commands;
   - LazyVim-style LSP goto/action/symbol aliases map to existing Helix LSP
     commands;
   - LazyVim-style git change navigation and changed-file aliases map to
     existing Helix change commands;
   - LazyVim-style window aliases map to existing Helix window commands;
   - `dd` deletes the current line;
   - `yy` yanks the current line so `p` can paste it;
   - `cc` changes the current line and enters insert mode.
5. A manual/editor smoke check launches Velix with `editor.keymap = "vim"` and
   verifies the same profile surface outside unit tests:
   - `0`, `gg`, `G`, and `C-r` trigger the expected Vim-profile actions;
   - one LazyVim finder/search alias opens the corresponding picker or prompt;
   - `dd`, `yy` followed by `p`, and `cc` behave as documented on a small
     scratch buffer.
6. The documentation page explains how to enable the profile and separates
   bindings into Vim-like, LazyVim-like, different, and deferred categories.
7. The issue README links this experiment and records the final status.
8. Markdown formatting is run:

   ```bash
   prettier --write --prose-wrap always --print-width 80 \
     issues/0002-vim-keymap-profile-and-documentation/README.md \
     issues/0002-vim-keymap-profile-and-documentation/01-selectable-vim-profile.md \
     book/src/SUMMARY.md \
     book/src/vim-profile.md
   ```

9. The issue index is regenerated with:

   ```bash
   scripts/build-issues-index.sh
   ```

10. Rust formatting and tests pass:

```bash
cargo fmt
cargo fmt --check
cargo test -p helix-term config
cargo test -p helix-term keymap
cargo test -p helix-term --test test vim_profile
```

Before implementation begins, this design must be reviewed by another AI agent
and approved. Record the review result in this file, then commit the approved
plan.

## Design Review

**Reviewer:** Fresh-context Codex adversarial reviewer.

**Initial verdict:** Changes required.

The reviewer found two required issues:

- The planned LazyVim alias scope covered finder/search, buffers, diagnostics,
  LSP, git, and windows, but the verification only required one representative
  finder/search alias.
- The verification checklist lacked a manual/editor smoke check for the actual
  editor UI.

The reviewer also suggested adding `cargo fmt --check` after `cargo fmt` so
completion reviewers can verify formatting read-only.

**Fixes made:**

- Added explicit verification requirements for every planned LazyVim alias
  category.
- Added a manual/editor smoke check covering representative motions, redo, a
  finder/search alias, and `dd`/`yy`/`p`/`cc`.
- Added `cargo fmt --check` to the Rust verification commands.
- Fixed the final verification code fence indentation after final approval.

**Final verdict:** Approved. No Required findings remained.
