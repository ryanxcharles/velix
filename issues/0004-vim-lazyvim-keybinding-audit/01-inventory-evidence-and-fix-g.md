# Experiment 1: Inventory evidence and fix `G`

## Description

Create the first auditable keybinding inventory for Issue 4 and use it to fix
one confirmed, high-value Vim-profile bug: `G` should go to the last line when
no count is provided while preserving counted Vim behavior such as `2G`.

This experiment intentionally does not try to migrate every Vim and LazyVim
binding in one pass. It establishes the evidence and verification pattern that
later batches will reuse, then proves that pattern on the known `G` failure.

The inventory sources are local and pinned in the workspace:

- Vim behavior evidence comes from `vendor/neovim/runtime/doc/`, especially
  `motion.txt`, `pattern.txt`, and `undo.txt`.
- LazyVim workflow evidence comes from `vendor/lazyvim/lua/lazyvim/`, especially
  `config/keymaps.lua`, `plugins/editor.lua`, and `plugins/lsp/keymaps.lua`.
- Velix behavior is checked against the current Vim profile in
  `helix-term/src/keymap/default.rs` and the integration/keymap tests in
  `helix-term/tests/test/commands/vim_profile.rs`.

The first inventory scope is normal-mode bindings that are already close to the
existing Velix Vim profile or its documented intent:

- core Vim motions and search: `h`, `j`, `k`, `l`, `0`, `^`, `$`, `gg`, `G`,
  `w`, `b`, `e`, `W`, `B`, `E`, `f`, `F`, `t`, `T`, `/`, `?`, `n`, `N`, `*`;
- core editing/repeat bindings already exposed by the profile or Helix default:
  `u`, `<C-r>`, `dd`, `yy`, `cc`, `p`, `P`;
- common LazyVim workflow aliases already targeted by Issue 2: `<C-h>`, `<C-j>`,
  `<C-k>`, `<C-l>`, `H`, `L`, `<leader><leader>`, `<leader>/`, `<leader>bb`,
  `]d`, `[d`, `<leader>xx`, `gd`, `gr`, `<leader>a`, `<leader>ss`, `<leader>gg`,
  `<leader>wd`;
- Velix change-navigation audit candidates already present in the profile: `]g`,
  `[g`. LazyVim's comparable Gitsigns hunk navigation uses `]h`, `[h` and is
  deferred.

The output is an issue-local audit table that records each binding, reference
source, expected behavior, observed Velix behavior, verification, status, fix
decision, and Helix alternative when exact compatibility is not appropriate.
Rows may be classified as `Works`, `Fixed`, `Different by design`, or
`Unsupported for now`, but every row in this first scope must include either an
automated test name or a concrete manual/keymap verification step.

## Changes

- Add `issues/0004-vim-lazyvim-keybinding-audit/audit.md` with the first
  evidence-backed audit table.
- Add a Vim-profile mappable command in `helix-term/src/commands.rs` for `G`
  semantics: use counted line navigation when `cx.count` is present, otherwise
  go to the last line.
- Update `helix-term/src/keymap/default.rs` so the Vim profile maps normal-mode
  `G` to that Vim-specific command.
- Update `helix-term/tests/test/commands/vim_profile.rs` so the representative
  keymap assertion expects the Vim-specific `G` command.
- Add integration tests proving that pressing `G` with the Vim profile moves the
  cursor to the last line of a multi-line document and that counted `G` still
  moves to the requested line.
- Update `book/src/vim-profile.md` if its `G` documentation needs to distinguish
  no-count `G` from count-based line navigation.
- Record the design review and final result review in this experiment file.

## Verification

Pass criteria:

- `audit.md` contains the full first-scope inventory and every row has source
  evidence, observed Velix behavior, verification, and a status.
- The design review is recorded below and has final verdict `APPROVED` before
  implementation begins.
- The Vim-profile keymap test proves `G` resolves to the Vim-specific `G`
  command.
- The Vim-profile integration tests prove bare `G` moves to the final line and
  counted `G` moves to the requested line.
- Existing Vim-profile tests still pass:
  `cargo test -p helix-term --features integration --test integration vim_profile`.
- Keymap/config regression tests still pass: `cargo test -p helix-term keymap`
  and `cargo test -p helix-term config`.
- Rust and markdown formatting pass: `cargo fmt --check` and
  `prettier --check --prose-wrap always --print-width 80` for changed markdown
  files.

Fail criteria:

- Any first-scope row lacks a concrete source or verification.
- `G` remains mapped to `goto_line`, bare `G` does not move to the last line, or
  counted `G` no longer moves to the requested line.
- The implementation changes unrelated bindings without audit evidence.

## Design Review

Reviewer: `Tesla` via adversarial-review subagent.

Initial verdict: `CHANGES REQUIRED`.

Finding:

- Required: the original design would have mapped `G` directly to
  `goto_last_line`, fixing bare `G` but dropping Vim's counted `[count]G`
  behavior. Neovim documents `G` as "Goto line [count], default last line" in
  `vendor/neovim/runtime/doc/motion.txt`, while Velix's `goto_line` uses
  `cx.count` and `goto_last_line` ignores it.

Fix:

- Revised the design to require a Vim-specific mappable command that uses
  counted line navigation when a count is present and last-line navigation when
  no count is present.
- Added explicit pass/fail criteria for both bare `G` and counted `G`.

Re-reviewer: `Jason` via adversarial-review subagent.

Final verdict: `APPROVED`.

Re-review result:

- No required findings.
- The reviewer confirmed that the revised design addresses the counted `G`
  blocker by requiring a Vim-specific command and tests for both bare `G` and
  counted `G`.

## Result

**Result:** Pass

Implemented the first audit slice and fixed the known `G` failure without
regressing counted Vim behavior.

Changes made:

- Added `audit.md` with first-scope rows for the basic Vim and common LazyVim
  bindings listed in this experiment.
- Added `vim_goto_line`, a Vim-profile command that sends counted `G` through
  counted line navigation and sends bare `G` to the last line.
- Mapped Vim-profile normal-mode `G` to `vim_goto_line`.
- Expanded Vim-profile keymap assertions to cover the first-scope inventory.
- Added integration tests for bare `G` and counted `2G`.
- Updated the Vim-profile documentation so `G` is described as counted-line
  navigation with file-end as the no-count default.

Completion review:

- Initial completion review verdict: `CHANGES REQUIRED`.
- Required finding: the audit and docs overclaimed LazyVim compatibility for
  `[g` and `]g`; local LazyVim maps hunk navigation to `[h` and `]h`, while
  Velix maps `[g` and `]g` to Helix/Velix change navigation.
- Fix: reclassified `[g` and `]g` as `Different by design` in `audit.md`,
  removed them from the LazyVim-like documentation table, and documented the
  LazyVim `[h`/`]h` alternative as deferred.
- Re-review verdict: `CHANGES REQUIRED`.
- Required finding: this experiment description still listed `[g` and `]g` as
  common LazyVim workflow aliases.
- Fix: moved `[g` and `]g` into a separate Velix change-navigation audit
  candidate bullet and explicitly deferred LazyVim `[h`/`]h`.
- Final re-review verdict: `APPROVED`.

Verification run:

- `cargo test -p helix-term --features integration --test integration vim_profile`
  - passed: 11 tests.
- `cargo test -p helix-term keymap` - passed: 13 tests.
- `cargo test -p helix-term config` - passed: 6 tests.
- `cargo fmt --check` - passed.
- `prettier --check --prose-wrap always --print-width 80 issues/0004-vim-lazyvim-keybinding-audit/README.md issues/0004-vim-lazyvim-keybinding-audit/01-inventory-evidence-and-fix-g.md issues/0004-vim-lazyvim-keybinding-audit/audit.md book/src/vim-profile.md`
  - passed.

After fixing the completion-review finding, reran:

- `cargo test -p helix-term --features integration --test integration vim_profile`
  - passed: 11 tests.
- `cargo fmt --check` - passed.
- `prettier --check --prose-wrap always --print-width 80 issues/0004-vim-lazyvim-keybinding-audit/README.md issues/0004-vim-lazyvim-keybinding-audit/01-inventory-evidence-and-fix-g.md issues/0004-vim-lazyvim-keybinding-audit/audit.md book/src/vim-profile.md`
  - passed.

## Conclusion

The audit table and expanded keymap tests give Issue 4 a concrete baseline for
the first batch of Vim and LazyVim bindings. The `G` bug was not a pure keymap
problem: mapping it directly to `goto_last_line` would have lost Vim's counted
`[count]G` behavior, so Velix now has a small Vim-specific wrapper command.

The next experiment should use the same audit-and-test pattern for the next
unsupported Vim grammar area rather than broadening this experiment further.
