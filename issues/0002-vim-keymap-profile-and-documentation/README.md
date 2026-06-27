+++
status = "closed"
opened = "2026-06-27"
closed = "2026-06-27"
+++

# Issue 2: Vim keymap profile and documentation

## Goal

Build the first usable Vim-oriented Velix keybinding profile, starting with easy
remaps, then adding the minimal grammar/state support needed for high-value Vim
editing behavior. Document which bindings are Vim-like, which bindings are
LazyVim-like, and which bindings are intentionally deferred because they need
harder semantic work.

## Background

Issue 1 concluded that Velix should not attempt Vim compatibility by replacing
Helix's keymap wholesale. The practical direction is a selectable `vim` keymap
profile that preserves Helix defaults unless the user opts in, plus an explicit
Vim grammar/state layer for behavior that Helix's existing key trie and command
sequences cannot faithfully model.

The current Helix-style keymap machinery should cover many low-risk bindings:

- Standalone normal-mode motions such as `h`, `j`, `k`, `l`, `w`, `b`, `e`,
  `gg`, `G`, `0`, `^`, and `$`, when the underlying Helix command semantics are
  close enough.
- Insert/open commands such as `i`, `a`, `I`, `A`, `o`, and `O`.
- Search and repeat-search commands such as `/`, `?`, `n`, `N`, and possibly
  `*`.
- Undo, paste, window, buffer, picker, diagnostic, and LSP aliases where Velix
  already has an equivalent editor command.

The harder work is Vim's input grammar. Vim normal mode is not just a table of
key sequences; it includes counts, operators, motions, text objects, register
prefixes, motion type metadata, and repeat state. For example, `dd`, `d$`, `dw`,
`2d3w`, `ci"`, `"ayy`, `"_dd`, and `.` all depend on state that pure remapping
cannot fully express.

For Helix-native features that raw Vim does not define, Velix should use LazyVim
as the compatibility reference. That includes picker, file search, global grep,
buffer, diagnostics, LSP, git, and common window-management workflows. The goal
is not to copy LazyVim's plugin stack; it is to make Velix feel familiar to a
LazyVim user when invoking equivalent built-in features.

## Analysis

This issue has three work streams that should be advanced through one experiment
at a time:

1. **Easy keybinding mappings.** Add a selectable Vim keymap profile and bind
   the standalone keys whose behavior can be implemented with existing commands
   or existing command sequences. These should be tested as profile behavior,
   not as changes to the default Helix-style keymap.
2. **Vim grammar/state layer.** Add the smallest explicit state layer needed for
   high-value Vim behavior that cannot be represented as static mappings. The
   first useful slice is likely linewise repeated operators such as `dd`, `yy`,
   and `cc`; later slices can extend toward operator-pending
   `operator + motion`, counts, text objects, registers, and dot-repeat.
3. **Keybinding documentation.** Maintain a clear compatibility document that
   separates:
   - Vim-like bindings implemented by Velix.
   - LazyVim-like workflow bindings implemented by Velix.
   - Bindings that are superficially mapped but semantically different from Vim.
   - Bindings that are significantly harder because they need grammar/state,
     command semantics, or editor model changes.

The documentation is part of the product surface, not an afterthought. Each
implementation experiment should update it so users can tell what compatibility
level the profile actually provides.

## Proposed Solutions

Add a named keymap profile, probably configured as a small explicit option such
as `editor.keymap = "vim"` or an equivalent existing configuration shape if the
local source suggests a better fit. The default profile should remain compatible
with upstream Helix behavior unless a later issue explicitly changes that
policy.

Implement easy mappings in the profile first. Favor mappings that have obvious
local commands and low semantic risk. When a key would look Vim-like but behave
substantially differently, either document the difference or defer the binding
until the needed state support exists.

Introduce Vim grammar/state incrementally. The first grammar experiment should
avoid pretending to solve full Vim; it should prove a narrow, testable slice and
make room for later extension. A good first target is explicit handling of
linewise repeated operators such as `dd`, `yy`, and `cc`, because those are
high-value, easy to explain, and reveal where command dispatch must differ from
plain keymap lookup.

Create or update user-facing documentation for the Vim profile as part of each
experiment. The documentation should include at least these categories:

- **Vim-like:** keys whose behavior is intended to match Vim closely enough for
  everyday use.
- **LazyVim-like:** keys that map Velix built-ins to familiar LazyVim workflow
  shortcuts.
- **Different:** keys that are available but intentionally retain Helix/Velix
  semantics.
- **Harder/deferred:** keys that need more grammar/state, motion metadata,
  registers, repeat tracking, text-object support, or deeper editor-model
  changes.

## Constraints

- Do not mutate the closed Issue 1 records.
- Keep upstream Helix-style keybindings available unless a future issue
  explicitly changes the default.
- Prefer local source, `vendor/neovim`, `vendor/lazyvim`, built-in help, and
  executable tests over memory when documenting behavior.
- Keep each experiment narrow enough to design, review, implement, verify,
  review again, and commit separately under the issue workflow.
- Avoid claiming Vim compatibility for bindings whose behavior is only
  superficially similar.

## Open Questions

- What is the least invasive configuration shape for selecting the Vim profile?
- Should the first mapping experiment include only core Vim motions, or also
  high-value LazyVim workflow aliases?
- Where should the Vim grammar/state live relative to `KeyTrie`, command
  dispatch, and editor state?
- What test helpers are sufficient for profile-specific keybinding behavior?
- Where should compatibility documentation live: `book/`, `docs/`, `runtime/`,
  or a combination of user-facing docs plus issue-local implementation notes?

## Experiments

- [Experiment 1: Selectable Vim profile](01-selectable-vim-profile.md) -
  **Pass**

## Conclusion

Issue 2 is solved by [Experiment 1](01-selectable-vim-profile.md). Velix now has
an opt-in `editor.keymap = "vim"` profile that keeps the default Helix-style
keymap unchanged unless selected, merges user `[keys]` overrides on top of the
selected profile, and rejects unknown profile names clearly.

The first profile implements low-risk Vim-like standalone normal-mode bindings,
LazyVim-like workflow aliases for existing Helix features, and a minimal
linewise `dd`/`yy`/`cc` slice using existing Helix command sequences. The
behavior is documented in `book/src/vim-profile.md`, including the current
Helix-semantic differences and deferred grammar work.

Further Vim compatibility should continue in new issues. The next major work is
explicit Vim grammar/state for operator-pending motions, counts, text objects,
register prefixes, dot-repeat, and more exact linewise paste/register fidelity.
