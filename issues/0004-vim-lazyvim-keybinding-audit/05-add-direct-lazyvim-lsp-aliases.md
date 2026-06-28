# Experiment 5: Add direct LazyVim LSP aliases

## Description

Experiment 4 classified the remaining audit categories and identified a small
set of common LazyVim LSP/code aliases that Velix can support with existing
commands. These are not blocked by Helix's selection model or missing plugin
features, so they should be fixed rather than left as unsupported.

This experiment will add only direct aliases where the LazyVim behavior has a
clear existing Velix command:

- `gI` -> `goto_implementation`
- `gK` -> `signature_help`
- `<leader>cr` -> `rename_symbol`

It will not add LazyVim mappings that require missing features or different
semantics, such as severity-specific diagnostic navigation, LazyVim sessions,
embedded terminal commands, Vim tabs, UI toggles, codelens, source actions, or
rename-file workflows. It also will not add `<leader>cf` in this pass because
LazyVim formats the current buffer, while Velix's existing `format_selections`
command formats the active selection/range.

## Changes

- `helix-term/src/keymap/default.rs`
  - Add the three exact LazyVim aliases to the Vim profile.
- `helix-term/tests/test/commands/vim_profile.rs`
  - Update keymap assertions so the three aliases resolve to the expected
    commands.
  - Keep representative unsupported/different assertions for mappings that
    remain intentionally unsupported or Helix-shaped.
- `issues/0004-vim-lazyvim-keybinding-audit/audit.md`
  - Add an Experiment 5 table recording these three aliases as fixed.
  - Update Experiment 4's direct-alias row so it no longer claims these three
    bindings are unsupported.
- `book/src/vim-profile.md`
  - Document the new LazyVim-like aliases and remove them from the deferred
    list.

## Verification

- Run `cargo fmt`.
- Run
  `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`.
- Run `prettier --write --prose-wrap always --print-width 80` on the changed
  Markdown files.

Pass criteria:

- The three direct aliases resolve to the expected Velix commands under the Vim
  profile.
- Experiment 4's unsupported/different rows still accurately describe only the
  mappings that remain unsupported or intentionally different.
- The user-facing Vim profile documentation matches the tested behavior.
- The targeted Vim-profile tests and formatting pass.

## Design Review

Reviewer: separate Codex adversarial reviewer.

Initial verdict: Changes required.

- Required: `<leader>cf` was not a direct compatible alias because LazyVim
  formats the current buffer, while Velix `format_selections` formats the active
  selection/range. Fixed by removing `<leader>cf` from this direct-alias
  experiment and explicitly leaving it deferred.

Final verdict: Approved. The reviewer confirmed that `<leader>cf` is excluded
with a reason, the audit/table scope consistently says three aliases, and the
pass criteria are under `## Verification`.

## Result

**Result:** Pass

The Vim profile now adds the three direct-compatible LazyVim LSP/code aliases:

- `gI` maps to `goto_implementation`.
- `gK` maps to `signature_help`.
- `<leader>cr` maps to `rename_symbol`.

`<leader>cf` remains unsupported for now because LazyVim formats the current
buffer, while Velix's existing `format_selections` command formats only the
active selection/range. The audit and user-facing documentation now distinguish
that deferred mapping from the three fixed aliases.

Verification run:

- `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`
  - Pass: 15 passed, 0 failed, 176 filtered out.
- `cargo fmt`
  - Pass.
- `prettier --write --prose-wrap always --print-width 80` on changed Markdown
  files
  - Pass.

## Completion Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer reported no Required findings. It independently verified that the
result commit had not yet been made, the source diff only added `gI`, `gK`, and
`<leader>cr`, `<leader>cf` remained unmapped and documented as deferred, nested
keymap merges preserved inherited mappings such as `gd`, `gr`, `gD`, and `gy`,
the experiment file had Result and Conclusion sections, the issue README marked
Experiment 5 as Pass, the targeted Vim-profile tests passed, `cargo fmt --check`
passed, and Prettier check passed.

## Conclusion

The remaining directly compatible LazyVim aliases found by Experiment 4 are now
fixed. Remaining unsupported LazyVim workflow gaps need missing editor features,
plugin integrations, or different semantics rather than simple keymap aliases.
