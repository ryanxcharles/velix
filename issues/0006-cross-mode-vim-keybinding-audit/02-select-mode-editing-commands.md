# Experiment 2: Select-mode editing commands

## Description

Experiment 1 fixed select-mode line motions and created a mode-specific audit
table. The next common Vim visual-mode batch is editing commands that operate on
the active selection.

This experiment will audit and, where needed, fix these Vim visual/select-mode
bindings:

- `d`: delete the highlighted selection.
- `c`: change the highlighted selection and enter insert mode.
- `y`: yank the highlighted selection.
- `>`: indent the selected lines.
- `<`: unindent the selected lines.
- `Esc`: leave select mode.
- `o`: move the cursor to the other end of the selection.

The experiment will not claim full Vim linewise/characterwise register metadata
or blockwise behavior. It will verify Velix's Helix selection semantics and
document any differences.

## Changes

- `helix-term/src/keymap/default.rs`
  - Add a Vim-profile select-mode override for `o` to flip the selection anchor
    and cursor if the current inherited binding does not already match Vim's
    visual-mode `o`.
  - Leave existing inherited select-mode `d`, `c`, `y`, `>`, `<`, and `Esc`
    bindings in place if they already resolve to the appropriate commands.
- `helix-term/tests/test/commands/vim_profile.rs`
  - Add select-mode keymap assertions for `d`, `c`, `y`, `>`, `<`, `Esc`, and
    `o`.
  - Assert exact command targets: `d => delete_selection`,
    `c => change_selection`, `y => yank`, `> => indent`, `< => unindent`,
    `Esc => exit_select_mode`, and Vim select-mode `o => flip_selections`.
  - Add integration tests proving select-mode `d`, `c`, and `y` operate on the
    current selection.
  - Add integration tests proving `>` and `<` target selected lines, including
    buffer contents and the resulting mode/selection behavior.
  - Add an integration test proving select-mode `o` flips the active selection
    endpoint, or document precisely why Helix cannot expose the behavior.
  - Keep explicit `editor.keymap = "default"` coverage for any changed binding.
    In this experiment, that means proving select-mode `o` still uses the
    Helix-style `open_below` behavior under `editor.keymap = "default"`, while
    only the Vim profile maps select-mode `o` to `flip_selections`.
- `issues/0006-cross-mode-vim-keybinding-audit/audit.md`
  - Add Experiment 2 rows for each audited binding and mode.
- `book/src/vim-profile.md`
  - Document newly verified select-mode editing bindings and any Helix-specific
    semantic differences.

## Verification

- Run `cargo fmt`.
- Run
  `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`.
- Run `prettier --write --prose-wrap always --print-width 80` on changed
  Markdown files.

Pass criteria:

- Select-mode `d`, `c`, `y`, `>`, `<`, `Esc`, and `o` have mode-specific audit
  rows with source evidence, observed behavior, verification, status, and fix or
  alternative.
- Keymap assertions prove each audited select-mode binding resolves to the
  expected Velix command under the Vim profile: `d => delete_selection`,
  `c => change_selection`, `y => yank`, `> => indent`, `< => unindent`,
  `Esc => exit_select_mode`, and `o => flip_selections`.
- Integration tests prove the destructive/editing behavior for at least `d`,
  `c`, and `y`.
- Integration tests prove `>` and `<` apply indentation changes to the selected
  lines and record the resulting mode/selection behavior.
- Select-mode `o` is fixed to flip the selection endpoint, or the experiment
  records precise implementation evidence for why it cannot be made Vim-like.
- Explicit `editor.keymap = "default"` remains unchanged for any touched
  binding, including proof that select-mode `o` keeps Helix-style `open_below`
  behavior outside the Vim profile.
- User-facing docs match the tested behavior and do not claim full Vim visual
  metadata.

## Design Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer initially required stronger verification for select-mode `>` and
`<`, because keymap assertions alone would not prove selected-line indentation
behavior. The design was revised to require integration tests for those keys,
exact expected command targets for all keymap assertions, and explicit default
profile coverage proving select-mode `o` remains Helix-style `open_below` unless
the Vim profile is selected. After those revisions, the reviewer approved the
design with no Required findings.
