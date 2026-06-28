# Experiment 3: Audit macro and repeat bindings

## Description

Audit and fix the next core Vim keybinding cluster: macro recording/replay and
repeat. Local Neovim help documents `q{reg}` to record a macro, bare `q` to stop
recording, `@{reg}` to replay a macro, and `.` to repeat the last change.

Velix already has macro commands, but the inherited Helix key spelling is not
Vim-like: `Q` records and `q` replays. Velix also has special handling for `.`,
but it is not full Vim dot-repeat grammar. This experiment should separate what
can be made Vim-like now from what must be documented as partial or deferred.

Local evidence:

- `vendor/neovim/runtime/doc/repeat.txt` documents `.` repeat, `q{reg}` macro
  recording, and `@{reg}` macro replay.
- `helix-term/src/commands.rs` contains `record_macro` and `replay_macro`.
- `helix-term/src/ui/editor.rs` supports pseudo-pending `on_next_key` callbacks,
  which can read the register key after `q` or `@`.
- `helix-term/src/keymap/default.rs` currently maps Helix defaults as
  `Q => record_macro` and `q => replay_macro`.

## Changes

- Add Vim-specific mappable commands in `helix-term/src/commands.rs`:
  - `vim_record_macro`: if a macro is currently recording, stop it; otherwise
    wait for the next key and record into that register.
  - `vim_replay_macro`: wait for the next key and replay that register.
  - Register both commands in the mappable static command list so keymap
    assertions and user remaps can resolve them.
- Update the Vim profile in `helix-term/src/keymap/default.rs`:
  - `q` maps to `vim_record_macro`.
  - `@` maps to `vim_replay_macro`.
  - `Q` remains available only if a direct Helix fallback is intentionally kept
    and documented; otherwise it should stop being advertised as Vim-like.
- Extend `helix-term/tests/test/commands/vim_profile.rs`:
  - keymap assertions for `q` and `@`;
  - integration test that records a small macro with `qa...q` and replays it
    with `@a`;
  - make the macro test fail observably if the register-selection key `a` is
    accidentally stored in the macro body, either by asserting final editor
    state that would include the extra `a` or by inspecting stored register
    contents;
  - integration or keymap evidence for `.` repeat, classifying it as partial if
    it only covers Velix's current insert-repeat behavior.
- Add an Experiment 3 section to
  `issues/0004-vim-lazyvim-keybinding-audit/audit.md` with rows for `q{reg}`,
  `q`, `@{reg}`, and `.`.
- Update `book/src/vim-profile.md` with supported macro keys and the repeat
  limitation.
- Record the design review and final result review in this experiment file.

## Verification

Pass criteria:

- The design review is recorded below and has final verdict `APPROVED` before
  implementation begins.
- `q` and `@` resolve to Vim-specific macro commands in the Vim profile.
- A Vim-profile integration test proves `qa...q@a` records and replays a macro.
- The audit table records whether `.` works, is partial, or is unsupported, with
  executable evidence and a documented Velix alternative or limitation.
- Existing Vim-profile tests still pass:
  `cargo test -p helix-term --features integration --test integration vim_profile`.
- Keymap/config regression tests still pass: `cargo test -p helix-term keymap`
  and `cargo test -p helix-term config`.
- Rust and markdown formatting pass: `cargo fmt --check` and
  `prettier --check --prose-wrap always --print-width 80` for changed markdown
  files.

Fail criteria:

- `q` still resolves to replay in the Vim profile.
- Macro recording includes the register-selection key in the recorded macro.
- Macro replay cannot target an explicit register.
- The experiment claims full Vim dot-repeat without evidence for full Vim repeat
  semantics.

## Design Review

Reviewer: `Gauss` via adversarial-review subagent.

Final verdict: `APPROVED`.

Findings:

- No required findings.
- Optional: explicitly mention registering the new Vim macro commands in the
  mappable static command list.
- Optional: make the macro integration test fail if the register-selection key
  is accidentally recorded.

Fixes:

- Added both optional clarifications to the design before the plan commit.

## Result

**Result:** Pass

Implemented Vim-style macro key spelling and audited dot-repeat without claiming
full Vim repeat compatibility.

Changes made:

- Added `vim_record_macro` and `vim_replay_macro` mappable commands.
- Mapped Vim-profile `q` to `vim_record_macro` and `@` to `vim_replay_macro`.
- Added keymap assertions for the new Vim macro commands and for inherited `Q`
  as an explicitly documented Helix fallback.
- Added an integration test proving `qa...q@a` records and replays a macro from
  an explicit register. The expected output would differ if the register key
  were accidentally recorded into the macro body.
- Added an integration test proving current `.` behavior repeats the last insert
  change.
- Added Experiment 3 audit rows for `q{reg}`, stopping with `q`, `@{reg}`, and
  `.`.
- Updated the Vim-profile docs with macro keys and the dot-repeat limitation.

Verification run:

- `cargo test -p helix-term --features integration --test integration vim_profile`
  - passed: 13 tests.
- `cargo test -p helix-term keymap` - passed: 13 tests.
- `cargo test -p helix-term config` - passed: 6 tests.
- `cargo fmt --check` - passed.
- `prettier --check --prose-wrap always --print-width 80 issues/0004-vim-lazyvim-keybinding-audit/README.md issues/0004-vim-lazyvim-keybinding-audit/03-audit-macro-and-repeat-bindings.md issues/0004-vim-lazyvim-keybinding-audit/audit.md book/src/vim-profile.md`
  - passed.

Completion review:

- Initial completion review verdict: `CHANGES REQUIRED`.
- Required finding: the Vim profile still inherited `Q => record_macro` from
  Helix without audit or documentation, despite the experiment plan requiring an
  intentional decision if `Q` stayed available.
- Fix: audited `Q` as `Different by design`, documented it as a direct Helix
  macro-record fallback, and added a keymap assertion proving the inherited
  binding is intentional.

After fixing the completion-review finding, reran:

- `cargo test -p helix-term --features integration --test integration vim_profile`
  - passed: 13 tests.
- `cargo test -p helix-term keymap` - passed: 13 tests.
- `cargo test -p helix-term config` - passed: 6 tests.
- `cargo fmt --check` - passed.
- `prettier --check --prose-wrap always --print-width 80 issues/0004-vim-lazyvim-keybinding-audit/README.md issues/0004-vim-lazyvim-keybinding-audit/03-audit-macro-and-repeat-bindings.md issues/0004-vim-lazyvim-keybinding-audit/audit.md book/src/vim-profile.md`
  - passed.

Final completion re-review:

- Reviewer: `Planck` via adversarial-review subagent.
- Final verdict: `APPROVED`.

## Conclusion

Vim-profile macro recording and replay now use Vim's `q{reg}` and `@{reg}` key
shape while reusing Velix's existing macro storage and replay machinery. Current
`.` behavior is verified as repeat-last-insert only; full Vim dot-repeat for
normal-mode/operator changes remains deferred.
