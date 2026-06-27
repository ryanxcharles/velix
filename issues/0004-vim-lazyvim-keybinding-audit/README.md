+++
status = "open"
opened = "2026-06-27"
+++

# Issue 4: Vim and LazyVim keybinding audit

## Goal

Audit the Velix Vim profile against basic Vim keybindings and common LazyVim
workflow keybindings by testing Velix itself. For every audited binding, record
whether it works, fix bindings that should be Vim-like or LazyVim-like, and
document any bindings that cannot be made compatible because of Helix-specific
editor model constraints.

## Background

Issue 2 added the first selectable Vim keymap profile and documented a first
slice of intended Vim-like and LazyVim-like behavior. Follow-up manual use found
that at least one documented binding is wrong: plain `G` was documented as going
to file end without a count, but the Vim profile does not override Helix's
default `G => goto_line`, and `goto_line` only moves when a count is present.
This means the profile needs a broader evidence-driven audit, not just spot
fixes.

The previous compatibility research in Issue 1 is still useful for categories
and references, but this issue should not trust a matrix or documentation claim
unless Velix behavior is tested. The audit must exercise the current Velix
binary/test harness with the Vim profile enabled and record observed behavior.

The relevant implementation surface is expected to include:

- `helix-term/src/keymap/default.rs`, where the default and Vim profile keymaps
  are defined.
- `helix-term/src/keymap.rs`, where key trie lookup and command sequences are
  implemented.
- `helix-term/src/commands.rs`, where mappable editing, motion, picker, window,
  LSP, diagnostic, git/change, and selection commands live.
- `helix-term/tests/test/commands/vim_profile.rs`, which currently tests the
  first Vim-profile slice.
- `book/src/vim-profile.md`, which documents the current profile.
- `vendor/neovim/runtime/doc/`, for Vim behavior evidence.
- `vendor/lazyvim/`, for common LazyVim workflow binding evidence.

## Analysis

This issue has two distinct jobs:

1. **Build an auditable keybinding inventory.** Determine the basic Vim
   keybindings Velix should care about first, including examples like `G`, and
   determine the common LazyVim workflow keybindings that should map to existing
   Velix/Helix features where possible.
2. **Test and fix behavior.** For every audited binding, run a concrete Velix
   test or executable check, record whether it works, and either fix the binding
   or document why it cannot be made Vim-like under Helix's model.

The audit should separate categories clearly:

- **Works:** binding behaves close enough to Vim or LazyVim for the stated
  scope.
- **Fixed:** binding did not work before this issue and now has tests proving it
  works.
- **Different by design:** binding exists but intentionally keeps Helix
  semantics, such as multiple-selection behavior or selection-first editing.
- **Unsupported for now:** binding cannot be made compatible without future
  grammar/state, editor model, or feature work.
- **Helix alternative:** when compatibility is unsupported or different, name
  the Helix/Velix binding or workflow users should use instead.

Every claim should be backed by executable evidence where feasible. If a binding
cannot be tested automatically because it opens an interactive picker, LSP
feature, diagnostic UI, or window operation, the experiment should either add a
focused keymap-level assertion or record a manual/editor smoke check with clear
pass/fail criteria.

## Proposed Solutions

Start with one experiment that builds the inventory and test harness shape, then
continue in small experiments that fix batches of bindings. Do not list all
experiments upfront; each completed audit/fix batch should inform the next one.

The first experiment should likely:

- derive a concrete list of basic Vim normal-mode bindings from local Neovim
  help;
- derive a concrete list of common LazyVim workflow bindings from local LazyVim
  files;
- create an issue-local audit table file that records binding, source evidence,
  expected behavior, Velix observed behavior, test command or test name, status,
  fix decision, and Helix alternative where applicable;
- add or extend automated tests for a small high-value batch, starting with
  obvious documented failures such as `G`;
- update `book/src/vim-profile.md` so user-facing docs match tested behavior.

Later experiments should fix additional batches until every audited binding is
classified as working, fixed, intentionally different, or unsupported with a
documented Helix alternative.

## Experiments

- [Experiment 1: Inventory evidence and fix `G`](01-inventory-evidence-and-fix-g.md) -
  **Pass**
- [Experiment 2: Add exact LazyVim workflow aliases](02-add-exact-lazyvim-workflow-aliases.md) -
  **Pass**
- [Experiment 3: Audit macro and repeat bindings](03-audit-macro-and-repeat-bindings.md) -
  **Pass**
- [Experiment 4: Classify remaining grammar and workflows](04-classify-remaining-grammar-and-workflows.md) -
  **Pass**
- [Experiment 5: Add direct LazyVim LSP aliases](05-add-direct-lazyvim-lsp-aliases.md) -
  **Pass**

## Constraints

- Do not modify closed issues.
- Do not claim a binding works unless Velix behavior has been tested or a
  precise keymap assertion proves the binding resolves to the intended command.
- Prefer local Neovim and LazyVim sources over memory for expected behavior.
- Keep Helix defaults available unless a future issue explicitly changes that
  policy.
- Preserve the issue workflow: design one experiment, review it, commit the
  plan, implement and verify it, completion-review it, commit the result, then
  decide the next experiment.
- When Helix's selection-first or multiple-selection model prevents exact Vim
  behavior, document that tradeoff plainly and provide the closest Helix/Velix
  alternative.

## Open Questions

- What is the initial "basic Vim keybindings" scope: normal-mode motions only,
  or motions plus common operators, text objects, registers, macros, and
  dot-repeat?
- Which LazyVim bindings are common enough to include in the first audit:
  finder/search, buffers, diagnostics, LSP, git changes, windows, save/quit,
  terminal, sessions, and UI toggles?
- Which bindings can be verified by command-level keymap assertions, and which
  require full editor-state integration tests?
- Should incompatible bindings remain unbound, map to Helix alternatives, or map
  only when the semantic difference is explicitly documented?
- Where should the durable audit table live: issue-local only, user-facing docs,
  or both?
