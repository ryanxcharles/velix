# Experiment 2: Add exact LazyVim workflow aliases

## Description

Add and audit a second small batch of LazyVim-compatible normal-mode aliases
where Velix already has a direct command with close enough behavior. Experiment
1 deliberately corrected overclaims around `[g` and `]g`; this experiment turns
that lesson into exact-key aliases instead of broad compatibility claims.

The scope is limited to aliases with local LazyVim source evidence and existing
Velix commands:

- `[b` and `]b`: previous and next buffer, from
  `vendor/lazyvim/lua/lazyvim/config/keymaps.lua`.
- `<leader>,`: buffer picker, from
  `vendor/lazyvim/lua/lazyvim/plugins/extras/editor/snacks_picker.lua`.
- `<leader>ca`: code action, from
  `vendor/lazyvim/lua/lazyvim/plugins/lsp/init.lua`.
- `[h` and `]h`: previous and next git hunk in LazyVim Gitsigns, from
  `vendor/lazyvim/lua/lazyvim/plugins/editor.lua`; Velix will map these to
  existing previous/next change navigation and document that semantic
  difference.
- `<leader>-` and `<leader>|`: horizontal and vertical splits, from
  `vendor/lazyvim/lua/lazyvim/config/keymaps.lua`.

This experiment should not implement external plugin features such as Gitsigns,
Trouble, Snacks, or LazyGit. It should only add aliases to existing Velix
commands, test those aliases, and update the audit/docs to distinguish exact
LazyVim key spelling from Velix's closest built-in semantics.

## Changes

- Update `helix-term/src/keymap/default.rs` Vim profile mappings:
  - `[b` / `]b` to previous/next buffer.
  - `[h` / `]h` to previous/next change.
  - `<space>,` to `buffer_picker`.
  - `<space>ca` to `code_action`.
  - `<space>-` to `hsplit`.
  - `<space>|` to `vsplit`.
- Extend `helix-term/tests/test/commands/vim_profile.rs` keymap assertions for
  every alias in this batch.
- Update `issues/0004-vim-lazyvim-keybinding-audit/audit.md` with an Experiment
  2 section for each alias, including source evidence, observed Velix command,
  verification, status, and any semantic difference.
- Update `book/src/vim-profile.md` to document only the aliases that the Vim
  profile now supports, and keep semantic differences under `## Different`.
- Record the design review and final result review in this experiment file.

## Verification

Pass criteria:

- The design review is recorded below and has final verdict `APPROVED` before
  implementation begins.
- Every alias in this experiment resolves to the intended `MappableCommand` in
  `vim_profile_maps_lazyvim_workflow_aliases`.
- Existing Vim-profile tests still pass:
  `cargo test -p helix-term --features integration --test integration vim_profile`.
- Keymap/config regression tests still pass: `cargo test -p helix-term keymap`
  and `cargo test -p helix-term config`.
- Rust and markdown formatting pass: `cargo fmt --check` and
  `prettier --check --prose-wrap always --print-width 80` for changed markdown
  files.

Fail criteria:

- Any alias in this scope lacks local LazyVim source evidence.
- Any alias maps to a command whose semantics are not documented when they
  differ from LazyVim's plugin-backed behavior.
- The implementation adds aliases outside this experiment's scope.

## Design Review

Reviewer: `Dirac` via adversarial-review subagent.

Final verdict: `APPROVED`.

Result:

- No required findings.
- The reviewer confirmed that every proposed alias has local LazyVim source
  evidence, existing Velix commands, concrete verification, and documented
  semantic handling for `[h`/`]h`.

## Result

**Result:** Pass

Implemented the exact LazyVim alias batch without adding external plugin
features.

Changes made:

- Added `[b` and `]b` aliases for previous/next buffer.
- Added `<space>,` for the buffer picker.
- Added `<space>ca` for code actions while keeping existing `<space>a`.
- Added `[h` and `]h` aliases to Velix previous/next change navigation, with
  documentation that this differs from LazyVim's Gitsigns hunk command.
- Added `<space>-` and `<space>|` aliases for horizontal and vertical splits.
- Extended Vim-profile keymap assertions and added Experiment 2 rows to
  `audit.md`.
- Updated the Vim-profile docs with the new aliases and their semantic
  difference.

Verification run:

- `cargo test -p helix-term --features integration --test integration vim_profile`
  - passed: 11 tests.
- `cargo test -p helix-term keymap` - passed: 13 tests.
- `cargo test -p helix-term config` - passed: 6 tests.
- `cargo fmt --check` - passed.
- `prettier --check --prose-wrap always --print-width 80 issues/0004-vim-lazyvim-keybinding-audit/README.md issues/0004-vim-lazyvim-keybinding-audit/02-add-exact-lazyvim-workflow-aliases.md issues/0004-vim-lazyvim-keybinding-audit/audit.md book/src/vim-profile.md`
  - passed.

Completion review:

- Reviewer: `Maxwell` via adversarial-review subagent.
- Final verdict: `APPROVED`.
- Findings: none.

## Conclusion

Velix now supports this batch of exact LazyVim key spellings where direct Velix
commands already existed. The main tradeoff remains `[h`/`]h`: the keys now work
as aliases, but they target Velix change navigation rather than Gitsigns hunk
state because Velix has no Gitsigns command surface.

The next experiment should audit another focused batch, preferably either
LazyVim save/quit/UI toggles with explicit unsupported classifications, or the
next core Vim grammar gap such as operator-pending motion forms.
