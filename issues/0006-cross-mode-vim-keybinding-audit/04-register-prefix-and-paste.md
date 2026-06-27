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
