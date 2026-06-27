# Experiment 1: Inventory and select-mode motions

## Description

Issue 6 exists because the prior Vim audit treated normal-mode evidence as too
broad. Normal-mode `$` works, but select-mode `$` still follows Helix behavior;
`V` was classified as part of a visual-mode limitation even though Velix has a
line-selection command that can satisfy the common Vim workflow.

This experiment will establish a mode-specific audit table and fix the first
high-confidence select-mode batch:

- `V` in normal mode should select the current line using Helix line-selection
  semantics.
- `$` in select mode should extend the selection to line end.
- `0` in select mode should extend the selection to line start.
- `^` in select mode should extend the selection to first non-whitespace.
- `G` in select mode should extend to the last line by default and preserve
  counted line behavior if a compatible command path exists.

The experiment does not claim true Vim linewise Visual mode metadata. It should
document that `V` is implemented as a line-selection entry point into Helix
select mode, not as a full Vim visual-line state with linewise register/paste
metadata.

## Changes

- `issues/0006-cross-mode-vim-keybinding-audit/audit.md`
  - Create the issue-local durable audit table.
  - Use one row per binding and mode, not one row per key globally.
  - Seed the table with common Vim normal, visual/select, insert, command/search
    and known unsupported grammar categories.
  - Mark the Experiment 1 batch with tested status, source evidence, observed
    behavior, and fix or alternative.
- `helix-term/src/keymap/default.rs`
  - Add Vim-profile normal-mode `V` as a sequence that selects the current line
    and leaves the editor in select mode.
  - Add Vim-profile select-mode overrides for `$`, `0`, `^`, and `G`.
  - Preserve `editor.keymap = "default"` behavior by changing only the Vim
    profile.
- `helix-term/src/commands.rs`
  - Add a Vim-specific select-mode command only if existing commands cannot
    express counted and uncounted select-mode `G` correctly.
- `helix-term/tests/test/commands/vim_profile.rs`
  - Add keymap assertions for the new normal and select-mode Vim-profile
    bindings.
  - Replace the old `V` missing assertion with the new expected behavior.
  - Add integration tests proving `V` selects the current line and select-mode
    `$` extends to line end.
  - Add integration or keymap-level tests for select-mode `0` and `^` according
    to what the command surface can verify reliably.
  - Add integration tests for both uncounted and counted select-mode `G`, such
    as `vG` extending to the last line and `v2G` extending to line 2, unless
    implementation evidence proves a specific case is blocked and the experiment
    documents that blocker precisely.
  - Keep explicit `editor.keymap = "default"` coverage so Helix-style defaults
    do not gain these Vim-profile bindings.
- `book/src/vim-profile.md`
  - Document the newly supported select-mode Vim motions.
  - Document the exact `V` limitation: line selection is supported, but full Vim
    linewise Visual mode metadata remains unsupported.

## Verification

- Run `cargo fmt`.
- Run `cargo test -p helix-term config::tests:: --lib`.
- Run
  `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`.
- Run `prettier --write --prose-wrap always --print-width 80` on changed
  Markdown files.

Pass criteria:

- The audit table exists and distinguishes bindings by mode.
- Normal-mode `V` in the Vim profile resolves to line selection behavior and is
  no longer classified as missing.
- A no-config integration test proves `V` selects the current line.
- Select-mode `$` in the Vim profile resolves to `extend_to_line_end`.
- An integration test proves `v$` or an equivalent select-mode sequence extends
  to line end.
- Select-mode `0` and `^` are either fixed with keymap/integration evidence or
  explicitly documented with precise implementation evidence if either is
  blocked.
- Select-mode `G` has integration evidence for both uncounted and counted Vim
  behavior, or the experiment precisely documents why one case is blocked by the
  command/model surface.
- Explicit `editor.keymap = "default"` still preserves Helix-style behavior for
  the touched keys.
- User-facing docs match the tested behavior and do not claim full Vim linewise
  Visual mode.

## Design Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer initially required stronger verification for select-mode `G`.
Because the design promises both uncounted and counted Vim behavior, keymap
assertions alone would not prove the combined behavior. The design was revised
to require integration tests for both `vG` extending to the last line and `v2G`
extending to line 2, unless implementation evidence proves a specific blocker.
After that revision, the reviewer approved the design with no Required findings.

## Result

**Result:** Pass

Experiment 1 created the mode-specific Issue 6 audit table and fixed the first
select-mode Vim batch.

Implemented behavior:

- Normal-mode `V` now selects the current line and enters Helix select mode.
- Select-mode `$` now extends to line end.
- Select-mode `0` now extends to line start.
- Select-mode `^` now extends to first non-whitespace.
- Select-mode `G` now uses `vim_extend_to_line`, preserving Vim's counted
  behavior while making bare `G` extend to the last line.

The implementation does not claim full Vim linewise Visual mode. `V` is a
line-selection entry point into Helix select mode; linewise register and paste
metadata remain deferred.

Verification run:

- `cargo fmt`
  - Pass.
- `cargo test -p helix-term config::tests:: --lib`
  - Pass: 7 passed, 0 failed.
- `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`
  - Pass: 24 passed, 0 failed, 176 filtered out.
- `prettier --write --prose-wrap always --print-width 80 issues/0006-cross-mode-vim-keybinding-audit/audit.md book/src/vim-profile.md`
  - Pass.

## Conclusion

The first cross-mode audit batch fixed the specific regressions that motivated
Issue 6: `V` now performs useful line selection, and `$` now works in select
mode. It also added related select-mode line motions so the Vim profile is not
normal-mode-only for this basic motion family. Follow-up experiments should
continue the mode-specific audit table rather than treating any key as globally
working based on one mode.

## Completion Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer reported no Required findings. It independently verified
`cargo fmt --check`, the config unit tests, the focused Vim-profile integration
tests, and Prettier checks on changed Markdown. The only Optional note was to
include the new untracked `audit.md` file in the result commit.
