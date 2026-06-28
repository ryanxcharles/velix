# Experiment 4: Register prefix and paste basics

## Description

Experiments 1 through 3 covered select-mode motions, select-mode editing, and
insert-mode basics. The durable audit and user documentation still classify Vim
register-prefix grammar such as `"ayy` and `"_dd` as deferred, but the current
Helix command surface already has a `select_register` command that stores the
next key in `editor.selected_register` and passes it to the next mapped command
through `Context`.

This experiment will test that behavior instead of relying on key-trie lookup
alone. The scope is the common Vim register and paste slice that can be
expressed without implementing a general operator-pending engine:

- Normal `"ayy`: yank the current line into named register `a`.
- Normal `"ap` / `"aP`: paste from named register `a` after or before the
  current selection.
- Normal `"_dd`: delete the current line without replacing the unnamed/default
  yank register.
- Select-mode `"ay`: yank the active selection into named register `a`.
- Select-mode `"_d`: delete the active selection without replacing the
  unnamed/default yank register.
- Insert-mode `C-r a`: insert register `a` at the cursor.

The experiment will also document the important limitation: Velix can pass a
selected register to already mapped commands such as `dd`, `yy`, `p`, `P`, `d`,
`y`, and insert `C-r`, but it still does not implement full Vim
register-prefixed operator grammar such as `"adw`, `"adiw`, or count/operator
combinations.

## Changes

- `helix-term/tests/test/commands/vim_profile.rs`
  - Replace keymap-only "missing" assertions for `"ayy` and `"_dd` with
    executable integration tests when the runtime behavior works.
  - Add integration tests for named-register yank/paste, black-hole delete, and
    insert-mode `C-r {register}`.
  - Keep missing assertions for register-prefixed forms that require unsupported
    operator-pending grammar.
- `issues/0006-cross-mode-vim-keybinding-audit/audit.md`
  - Add Experiment 4 rows for each tested register/paste binding by mode.
  - Correct any stale rows that overstate register-prefix grammar as entirely
    deferred.
- `book/src/vim-profile.md`
  - Document supported register-prefix and paste basics.
  - Narrow the deferred register language to operator-pending forms that remain
    unsupported.

## Verification

- Run `cargo fmt`.
- Run
  `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`.
- Run `prettier --write --prose-wrap always --print-width 80` on changed
  Markdown files.

Pass criteria:

- Each listed normal, select, and insert register binding has executable
  evidence or a precise missing assertion in the Vim-profile test suite.
- Named-register tests prove that `"ayy`, `"ap`, `"aP`, select `"ay`, and insert
  `C-r a` use register `a`, not merely the unnamed/default register.
- Black-hole tests prove that `"_dd` and select `"_d` delete text without
  replacing the default yank register.
- Documentation and the audit distinguish supported register-prefix command
  forwarding from unsupported full Vim operator-pending grammar.
- If any listed binding does not work, this experiment either fixes it within
  the existing command/keymap architecture or records the precise Helix-specific
  reason it cannot be made Vim-like in this slice.

## Design Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer found no Required issues. It confirmed the README links Experiment
4 with status `Designed`, the experiment has the required design sections, and
the scope fits Issue 6. It also checked the key dispatch premise:
`select_register` stores `editor.selected_register`, dispatch forwards that
register through `Context`, and the relevant yank, delete, paste, and
insert-register commands consume `cx.register` or a register-specific paste
path. The reviewer approved the plan-only state with no implementation present.

## Result

**Result:** Pass

Experiment 4 verified that Velix already supports the register-prefix slice for
commands that are mapped in the Vim profile. No key dispatch code was needed:
`select_register` stores the selected register, pending keymap dispatch carries
it forward, and the existing yank, delete, paste, and insert-register commands
consume it.

The test suite now proves:

- normal `"ayy` yanks the current line into register `a`;
- normal `"ap` and `"aP` paste from register `a`;
- normal `"_dd` deletes the current line without replacing the default yank
  register;
- select-mode `"ay` yanks the active selection into register `a`;
- select-mode `"_d` deletes the active selection without replacing the default
  yank register;
- insert-mode `C-r a` inserts register `a`.

The audit and user-facing Vim profile documentation now distinguish this
supported register forwarding from still-unsupported register-prefixed
operator-pending grammar such as `"adw` and `"adiw`.

Verification run:

- `cargo fmt`
  - Pass.
- `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`
  - Pass: 40 passed, 0 failed, 176 filtered out.
- `prettier --write --prose-wrap always --print-width 80` on changed Markdown
  files
  - Pass.

## Completion Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer found no Required issues. It verified that the result commit had
not been made before review, the README marks Experiment 4 as `Pass`, the
experiment file includes `Result` and `Conclusion`, named-register tests assert
register `a` contents, black-hole tests preserve the unnamed register in normal
and select modes, and the docs and audit distinguish supported register
forwarding from unsupported operator-pending register grammar. It also reran
`cargo fmt --check` and the Vim-profile integration test suite, which passed
with 40 tests.

## Conclusion

The earlier broad claim that register-prefix grammar such as `"ayy` and `"_dd`
was deferred was too coarse. Velix can support register prefixes for mapped
commands through its existing register forwarding path. The remaining gap is the
deeper Vim grammar: register-prefixed operators with arbitrary motions or text
objects still require operator-pending support or a deliberate selection-first
alternative.
