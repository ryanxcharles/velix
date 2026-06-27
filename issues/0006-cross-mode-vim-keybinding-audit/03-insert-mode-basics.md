# Experiment 3: Insert-mode basics

## Description

Experiments 1 and 2 fixed and audited select-mode behavior. Issue 6 also
requires auditing common Vim bindings in every relevant mode, and insert mode
currently has only minimal coverage in the audit table.

This experiment will audit the common Vim insert-mode bindings that Velix can
support with its existing insert-mode command surface:

- `Esc`: return to normal mode.
- `C-[`: return to normal mode.
- `Backspace` / `C-h`: delete the character before the cursor.
- `C-w`: delete the word before the cursor.
- `C-u`: delete backward on the current line, with exact Velix behavior tested
  and compared to Vim's inserted-text / `'backspace'` nuance.
- `C-j` / `C-m` / `Enter`: insert a newline, using Velix's `<ret>`
  representation for Enter / `C-m` where appropriate.
- `C-r`: insert register contents.
- `C-x`: invoke completion.

The experiment will classify unsupported or different insert-mode controls, such
as Vim insert `C-k` digraph entry, Vim's full `C-o` one-normal-command behavior,
exact `C-t` / `C-d` indent controls, and advanced `C-x` completion submodes, if
Velix lacks a direct compatible command surface.

## Changes

- `helix-term/src/keymap/default.rs`
  - Add Vim-profile insert-mode aliases only where Velix has direct compatible
    commands and the default insert map does not already cover the key. The
    expected first candidate is `C-[ => normal_mode`.
- `helix-term/tests/test/commands/vim_profile.rs`
  - Expand insert-mode keymap assertions for the audited bindings.
  - Add integration tests for representative editing behavior:
    - `C-[` exits insert mode.
    - `C-h` / Backspace delete the previous character.
    - `C-w` deletes the previous word.
    - `C-u` deletes backward on the current line, including an indented-line
      case that records Velix's first-non-whitespace behavior.
    - `C-j` / Enter insert a newline.
  - Add keymap-level evidence for `C-r` and `C-x`, and integration coverage if a
    stable non-LSP test can prove the behavior without opening asynchronous UI
    that makes the test brittle.
  - Add audit evidence for insert-mode `C-k` as a documented difference: Vim
    uses `C-k` for digraph entry, while Velix currently maps it to
    `kill_to_line_end`.
  - Preserve explicit `editor.keymap = "default"` behavior for any added Vim
    insert-mode alias.
- `issues/0006-cross-mode-vim-keybinding-audit/audit.md`
  - Add Experiment 3 rows for each audited insert-mode binding.
- `book/src/vim-profile.md`
  - Document supported insert-mode basics and any explicitly deferred insert
    grammar.

## Verification

- Run `cargo fmt`.
- Run
  `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`.
- Run `prettier --write --prose-wrap always --print-width 80` on changed
  Markdown files.

Pass criteria:

- Insert-mode `Esc`, `C-[`, Backspace, `C-h`, `C-w`, `C-u`, `C-j`, `C-m` /
  Enter, `C-r`, `C-x`, and `C-k` have mode-specific audit rows with source
  evidence, observed behavior, verification, status, and fix or alternative.
- Keymap assertions prove each Vim-compatible supported insert-mode binding
  resolves to the expected Velix command under the Vim profile.
- Integration tests prove the representative editing behavior for exit,
  character deletion, word deletion, current-line backward deletion, and newline
  insertion.
- `C-u` records the tested Velix behavior on both ordinary and indented lines,
  rather than overclaiming exact Vim inserted-text / `'backspace'` semantics.
- `C-k` is documented as different by design or unsupported for Vim
  compatibility unless this experiment implements digraph entry.
- Any unsupported Vim insert-mode behavior is documented with a precise reason
  and the closest Velix alternative.
- Explicit `editor.keymap = "default"` remains unchanged for any touched
  binding.
- User-facing docs match tested behavior.

## Design Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer initially rejected the design because it overclaimed insert-mode
`C-k` as Vim-compatible. Neovim uses `C-k` for digraph entry, while Velix maps
it to `kill_to_line_end`. The design was revised to classify `C-k` as a
documented difference unless digraph behavior is implemented. The revision also
added `C-m` / Enter coverage and required `C-u` tests to record Velix behavior
on ordinary and indented lines rather than claiming exact Vim inserted-text /
`'backspace'` semantics. After those changes, the reviewer approved the design
with no Required findings.

## Result

**Result:** Pass

Experiment 3 audited common insert-mode Vim bindings. The Vim profile now adds
`C-[ => normal_mode`, matching Vim's alternate escape key. Existing insert-mode
bindings for `Esc`, Backspace / `C-h`, `C-w`, `C-u`, `C-j` / Enter, `C-r`, and
`C-x` were covered with keymap assertions and representative integration tests
where stable.

The audit deliberately does not claim Vim compatibility for insert-mode `C-k`:
Vim uses `C-k` for digraph entry, while Velix keeps Helix's `kill_to_line_end`
behavior. It also documents `C-u` as Velix `kill_to_line_start` behavior rather
than exact Vim inserted-text / `'backspace'` semantics.

Verification run:

- `cargo fmt`
  - Pass.
- `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`
  - Pass: 35 passed, 0 failed, 176 filtered out.
- `prettier --write --prose-wrap always --print-width 80` on changed Markdown
  files
  - Pass.

## Completion Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer initially required one audit fix: the older Experiment 1 `C-u`
insert-mode row still classified the binding as fully `Works`, contradicting
Experiment 3's tested `Different by design` classification for Velix
`kill_to_line_start` behavior on indented lines. The audit row was updated to
match the tested Experiment 3 behavior and point to the new verification. The
reviewer rechecked the diff, formatting, and integration tests, then approved
with no Required findings.

## Conclusion

Velix now has tested insert-mode coverage for common Vim editing keys that map
to the existing command surface. The experiment added the missing `C-[` escape
alias and documented insert-mode differences that should not be overclaimed,
especially `C-k` digraph entry and advanced `C-x` completion submodes. Follow-up
experiments should continue with remaining cross-mode gaps such as registers,
paste metadata, command/search prompt behavior, marks, and operator-pending
grammar.
