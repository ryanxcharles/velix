# Experiment 1: Build a Vim and LazyVim compatibility matrix

## Description

Build a local-evidence compatibility matrix that classifies the keybindings and
input semantics Velix would need for a first useful Vim/LazyVim mode.

The goal is to answer Issue 1's first decision: whether Velix can start with a
default keymap/profile swap, needs only a handful of new commands, or needs a
new Vim grammar layer beside Helix's current `KeyTrie` lookup. This experiment
is intentionally read-only with respect to editor behavior; it should produce
research artifacts and a recommendation, not code changes.

The matrix should use three references:

- **Velix/Helix implementation:** `helix-term/src/keymap/default.rs`,
  `helix-term/src/keymap.rs`, `helix-term/src/config.rs`,
  `helix-term/src/commands.rs`, and `helix-term/tests/test/helpers.rs`.
- **Neovim/Vim semantics:** local docs under `vendor/neovim/runtime/doc/`,
  especially `motion.txt`, `change.txt`, `map.txt`, `visual.txt`, and
  `repeat.txt`.
- **LazyVim workflow bindings:**
  `vendor/lazyvim/lua/lazyvim/config/keymaps.lua`,
  `vendor/lazyvim/lua/lazyvim/plugins/editor.lua`,
  `vendor/lazyvim/lua/lazyvim/plugins/lsp/keymaps.lua`, and picker/plugin specs
  that define file, buffer, diagnostic, git, and LSP workflows.

## Changes

Create a research result file:

- `issues/0001-vim-keybinding-research/vim-lazyvim-compatibility-matrix.md`

The matrix must include these sections:

1. **Executive recommendation**
   - Recommend one initial architecture: default keymap replacement, named `vim`
     keymap profile, generated/configured Vim layer, or explicit Vim grammar
     state beside `KeyTrie`.
   - State what should be implemented first and what should be deferred.
2. **Helix keymap architecture summary**
   - Summarize how default keymaps, user key overrides, pending trie state,
     sticky nodes, command sequences, counts, and integration-test helpers work.
   - Cite file paths and line numbers where possible.
3. **Core Vim normal-mode matrix**
   - Cover at least: basic motions, word motions, line motions, search motions,
     insert/append/open, delete/change/yank, paste, undo/redo, visual/select
     mode, text objects, registers, macros, counts, and dot-repeat.
   - Classify each row as `Remap only`, `New command`, `New grammar/state`, or
     `Deferred`.
4. **LazyVim workflow matrix**
   - Cover at least: file finder, live grep/search, buffers, diagnostics,
     quickfix/location-style lists, LSP definitions/references/actions, symbols,
     git hunks/log/blame, windows/splits, tabs, terminal, save/quit/session, and
     UI toggles.
   - Map LazyVim leader-style bindings to existing Helix commands where
     possible.
   - Name the exact LazyVim Lua file or plugin spec that owns each workflow
     binding.
5. **First implementation slice**
   - Propose the first code experiment after this research.
   - Include exact files expected to change.
   - Include a minimal verification plan using existing tests or small new
     integration tests.
6. **Risks and deferrals**
   - Identify compatibility gaps that should not block the first usable build.
   - Identify risky semantic areas, especially operator-pending mode, counts,
     visual/select mode differences, multiple selections, registers, and
     dot-repeat.

Do not modify Rust code in this experiment. Do not change default keybindings
yet.

## Verification

This experiment passes when all of the following are true:

1. `vim-lazyvim-compatibility-matrix.md` exists and contains all six required
   sections listed above.
2. Every non-`Deferred` core Vim classification cites both:
   - the source behavior reference from local Neovim help; and
   - the relevant Velix/Helix implementation evidence.
3. Every non-`Deferred` LazyVim workflow classification cites both:
   - the exact local LazyVim Lua file or plugin spec that defines the binding or
     workflow; and
   - the relevant Velix/Helix implementation evidence.
4. Any classification that cannot cite both sides explicitly explains why one
   side is not applicable.
5. The result file includes a clear recommendation for the next code experiment
   and states why a pure keymap change is or is not enough.
6. The result file identifies at least one first-slice implementation that can
   be verified with `helix-term` tests.
7. Markdown formatting has been run:

   ```bash
   prettier --write --prose-wrap always --print-width 80 \
     issues/0001-vim-keybinding-research/01-compatibility-matrix.md \
     issues/0001-vim-keybinding-research/vim-lazyvim-compatibility-matrix.md \
     issues/0001-vim-keybinding-research/README.md \
     issues/README.md
   ```

8. The issue index has been regenerated:

   ```bash
   scripts/build-issues-index.sh
   ```

Before implementation begins, this design must be reviewed by another AI agent
and approved. Record the review result in this file, then commit the approved
plan.

## Design Review

**Reviewer:** Fresh-context Codex adversarial reviewer.

**Initial verdict:** Changes required.

The reviewer found that the verification criteria allowed each matrix row to
cite only one side of the comparison, which was too weak for deciding whether a
binding is a remap, new command, or new grammar/state. The reviewer also noted
that LazyVim workflow rows should name the exact Lua file or plugin spec that
owns each binding.

**Fixes made:**

- Required every non-`Deferred` core Vim row to cite both local Neovim behavior
  evidence and local Velix/Helix implementation evidence.
- Required every non-`Deferred` LazyVim workflow row to cite both the exact
  LazyVim Lua/plugin source and local Velix/Helix implementation evidence.
- Required explicit explanation when both sides cannot be cited.
- Required each LazyVim workflow row to name the exact owning Lua file or plugin
  spec.

**Final verdict:** Approved. No Required findings remained.
