+++
status = "open"
opened = "2026-06-27"
+++

# Issue 6: Cross-mode Vim keybinding audit

## Goal

Audit common Vim keybindings in every relevant Velix mode, then fix every
binding that can reasonably be made Vim-like within Helix's architecture. This
must include `V` for line selection and `$` in visual/select mode.

## Background

Issue 4 audited the first Vim and LazyVim keybinding slices, but the result was
too normal-mode-heavy. It verified normal-mode `$`, then missed that `$` does
not work as a Vim-like motion while extending a selection. It also classified
`V` as part of a broad visual-mode limitation instead of testing and fixing the
basic user workflow: selecting the current line.

Issue 5 made the Vim keymap the default, which raises the quality bar. A fresh
Velix session should not merely have Vim-like normal-mode aliases; common Vim
muscle memory should be audited across normal, visual/select, insert, command,
operator-like workflows, register workflows, macro workflows, search workflows,
and any other mode where Vim users reasonably expect key behavior.

The motivating known failures are:

- `V` does not select the current line.
- `$` works in normal mode but not in visual/select mode.

## Analysis

The audit must avoid the mistake from Issue 4: a binding cannot be marked
working globally because it works in one mode. Each binding needs mode-specific
evidence. For example, normal-mode `$` and visual-mode `$` are separate audit
rows because one moves the cursor and the other should extend the selection.

The relevant implementation surface is expected to include:

- `helix-term/src/keymap/default.rs`, where default and Vim-profile keymaps are
  defined for normal, select, and insert modes.
- `helix-term/src/keymap.rs`, where key trie lookup and keymap merging can hide
  mode-specific gaps.
- `helix-term/src/ui/editor.rs`, where count handling, pending keymaps, and mode
  dispatch affect whether Vim-style sequences can work.
- `helix-term/src/commands.rs`, where movement, selection, macro, register,
  search, paste, and editing commands live.
- `helix-term/tests/test/commands/vim_profile.rs`, where executable Vim-profile
  coverage should be expanded.
- `book/src/vim-profile.md`, where user-facing Vim compatibility and remaining
  limitations must match tested behavior.
- Local Vim/Neovim help under `vendor/neovim/runtime/doc/`, plus existing Issue
  4 audit material, for expected behavior evidence.

The work should classify and fix bindings by mode:

- **Normal mode:** motions, edits, operators or operator-like shortcuts,
  registers, marks/jumps, macros, repeat, search, paste, undo/redo, windows,
  buffers, and leader workflows.
- **Visual/select mode:** characterwise selection behavior, linewise selection
  behavior, selection-extending motions such as `$`, `0`, `^`, `G`, word
  motions, search motions, yanking/deleting/changing selections, and anchor
  manipulation where Vim has an equivalent.
- **Insert mode:** escape, common control-key editing, completion-related keys,
  and any Vim insert-mode bindings Velix can support without deeper state.
- **Command/search/prompt modes:** only where Velix exposes comparable mode
  behavior and keymaps; otherwise document the Helix alternative.
- **Unsupported Vim modes or grammars:** operator-pending mode, blockwise Visual
  mode, Ex command-line editing details, named marks, and other model-specific
  areas must be tested or precisely inspected before being marked unsupported.

Fix policy:

- Fix every common Vim binding that has a direct or close Velix command surface.
- If exact Vim behavior is impossible because Helix's selection-first or
  multiple-selection model lacks the needed state, document the limitation and
  provide the closest Helix/Velix alternative.
- Do not use broad categories such as "visual mode differs" to avoid fixing
  specific compatible bindings like `V` or visual/select-mode `$`.
- Do not claim a binding works unless it has executable evidence or a precise
  keymap assertion for the relevant mode.

## Proposed Solutions

Start with one experiment that builds a complete cross-mode audit inventory from
local Vim/Neovim sources and the current Velix keymaps, then fixes a small
high-confidence batch of known failures. The first batch should include `V` and
visual/select-mode `$`, and should add tests that fail before the fix and pass
afterward.

Subsequent experiments should proceed one batch at a time, using the prior
experiment's results to choose the next highest-value set of bindings. Each
experiment should update the audit table and user-facing documentation as
needed.

The durable audit table should be issue-local and mode-specific, with columns
for binding, mode, source evidence, expected Vim behavior, observed Velix
behavior, verification, status, fix, and Helix/Velix alternative.

## Experiments

- [Experiment 1: Inventory and select-mode motions](01-inventory-and-select-mode-motions.md) -
  **Designed**

## Constraints

- Preserve the issue workflow: design one experiment, review it, commit the
  plan, implement and verify it, completion-review it, and commit the result.
- Do not list experiments upfront.
- Do not modify closed issues.
- Run `cargo fmt` after Rust edits.
- Run Prettier after Markdown edits.
- Treat `V` and visual/select-mode `$` as required fixes unless implementation
  evidence proves they are blocked by Helix architecture.
- Keep Helix-style defaults available through `editor.keymap = "default"` unless
  a future issue explicitly changes that policy.
