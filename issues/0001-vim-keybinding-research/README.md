+++
status = "open"
opened = "2026-06-27"
+++

# Issue 1: Vim keybinding research

## Goal

Determine what Velix needs to change to offer Vim-style keybindings on top of
Helix's Rust editor architecture, with a pragmatic compatibility target rather
than a requirement for perfect Vim emulation.

## Background

Velix starts as a Helix fork. Helix already has a modal editor, a documented
keymap system, and many bindings that overlap with Vim, but its editing model is
selection-first rather than Vim's operator-pending verb-then-motion model.

Initial local source inspection found the likely implementation surface:

- `helix-term/src/keymap/default.rs` defines the default normal, insert, and
  select mode keymaps.
- `helix-term/src/keymap.rs` defines the `KeyTrie`, `Keymaps`, pending-key
  state, sticky nodes, reverse maps, and `merge_keys`.
- `helix-term/src/config.rs` starts from `keymap::default()` and merges user key
  overrides from config.
- `helix-view/src/input.rs` defines key event parsing/display and special key
  names used by keymaps.
- `helix-term/src/commands.rs` contains static editor commands exposed to
  keymaps.
- `helix-term/tests/test/helpers.rs` provides integration-test helpers that
  execute key sequences against editor state.
- `book/src/remapping.md`, `book/src/keymap.md`, and `book/src/from-vim.md`
  document current keymap behavior and known Vim differences.

Local reference checkouts are available under gitignored `vendor/`:

- `vendor/neovim/` at `cef31fde6a` from `https://github.com/neovim/neovim`. Use
  this for base Vim/Neovim semantics, especially `runtime/doc/motion.txt`,
  `runtime/doc/change.txt`, `runtime/doc/map.txt`, `runtime/doc/visual.txt` when
  present, and related help files.
- `vendor/lazyvim/` at `459a4c3b` from `https://github.com/lazyvim/lazyvim`. Use
  this for modern Neovim/LazyVim workflow bindings, especially
  `lua/lazyvim/config/keymaps.lua`, `lua/lazyvim/plugins/editor.lua`,
  `lua/lazyvim/plugins/lsp/keymaps.lua`, and plugin specs that define picker,
  buffer, diagnostics, git, and LSP behavior.

The existing keymap layer looks friendly to simple binding changes: default
bindings are centralized, user remaps are already data-driven, and command
sequences can be bound in TOML. The uncertain part is whether Vim-style behavior
can be represented as remapping existing commands or whether Velix needs new
state and commands for Vim grammar.

## Analysis

There are three likely layers of work:

1. **Pure keymap replacement.** Map Vim-compatible keys to existing Helix
   commands where semantics already match closely. Examples include `h`, `j`,
   `k`, `l`, `i`, `a`, `o`, `O`, `/`, `?`, `n`, `N`, `u`, `p`, `P`, and parts of
   `g`, `z`, and `Ctrl-w`.
2. **Command aliasing and command sequences.** Bind Vim-like keys to short
   command sequences where Helix already has the primitives, such as line
   selection plus delete/change/yank. This may cover basics like `dd`, `yy`, and
   some visual/select-mode operations.
3. **New Vim grammar support.** Add behavior that does not fit Helix's current
   selection-first commands, especially operator-pending input (`d{motion}`,
   `c{motion}`, `y{motion}`), counts (`3dw`, `2j`, `d2w`), repeated operators
   (`dd`, `cc`, `yy`), text-object grammar (`diw`, `ci"`, `ya)`), dot-repeat,
   and edge cases where Vim motions are linewise, characterwise, or exclusive.

The research should avoid assuming that full Vim compatibility is cheap. The
first decision is whether Velix should:

- replace Helix's default keymap with a Vim-oriented default;
- add a named keymap profile such as `editor.keymap = "vim"`;
- keep Helix defaults and ship a generated/configured Vim layer;
- or introduce a new input grammar beside the existing `KeyTrie` lookup.

For editor features that exceed raw Vim, such as file pickers, symbol pickers,
diagnostics, LSP actions, buffer navigation, and git workflows, LazyVim is the
preferred compatibility reference. The goal is not to copy LazyVim's plugin
stack, but to make Velix's richer Helix-native features feel familiar to users
coming from a LazyVim setup.

## Questions

- Which Vim bindings can be implemented by changing only
  `helix-term/src/keymap/default.rs`?
- Which Vim bindings need new commands in `helix-term/src/commands.rs` but not a
  new key-dispatch model?
- Which Vim bindings require new pending input state beyond the current
  `Keymaps::state` trie path?
- How should counts be represented and tested?
- Can operator-pending mode be modeled as a keymap/sticky node, or does it need
  explicit editor/input state?
- How should Velix reconcile Vim visual mode with Helix select mode and multiple
  selections?
- What compatibility target is worthwhile for the first usable Velix build?

## Proposed Research Plan

Start with a read-only compatibility matrix. Compare Vim's core normal-mode
grammar against Helix's current commands and classify each candidate binding as:

- **Remap only**: existing command or command sequence is close enough.
- **New command**: key dispatch can stay unchanged, but a missing editor command
  is needed.
- **New grammar/state**: behavior needs counts, operator-pending state, or
  motion/operator composition.
- **Deferred**: not needed for the first compatibility milestone.

Then design the first experiment around the smallest useful vertical slice,
probably basic normal-mode motions plus a small number of repeated operators
such as `dd`, `yy`, and `cc`, because those quickly expose whether the existing
trie/sequence machinery is enough.

## Experiments

- [Experiment 1: Build a Vim and LazyVim compatibility matrix](01-compatibility-matrix.md) -
  **Designed**
