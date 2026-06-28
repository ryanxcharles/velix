# Experiment 4: Classify remaining grammar and workflows

## Description

Experiments 1 through 3 fixed high-value Vim-profile gaps, but the issue still
has open audit scope around Vim's broader operator grammar, text objects,
register prefixes, visual modes, marks, and common LazyVim workflow groups.

This experiment will finish the audit by testing representative keymap behavior
for those remaining categories and recording whether each category works, is
different by design, or is unsupported for now. It will not attempt to implement
full operator-pending mode, Vim text-object grammar, Vim marks, tabs, embedded
terminal workflows, or LazyVim plugin-only workflows. Those require editor model
or feature work larger than one audit experiment. Instead, the experiment will
document the Helix/Velix alternative where one exists.

## Changes

- `helix-term/tests/test/commands/vim_profile.rs`
  - Add focused keymap assertions for representative remaining gaps:
    operator-pending forms such as `dw`, text-object forms such as `diw`,
    register-prefix forms such as `"ayy`, visual mode keys, marks and jumps,
    LazyVim save/quit, terminal, session, tab, and UI-toggle mappings.
  - Assert the current Velix command or lack of command for each representative
    sequence so the audit is backed by executable evidence.
- `issues/0004-vim-lazyvim-keybinding-audit/audit.md`
  - Add an Experiment 4 table for the remaining Vim grammar and common LazyVim
    workflow categories.
  - Record source evidence from local Neovim help and vendored LazyVim files,
    observed Velix behavior, verification, status, and alternatives.
- `book/src/vim-profile.md`
  - Expand the Different and Deferred sections so users know which Vim or
    LazyVim keys are intentionally Helix-shaped, unsupported for now, or
    available through another Velix command.

## Verification

- Run `cargo fmt`.
- Run
  `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`.
- Confirm the new keymap assertions prove the audit rows for representative
  remaining categories, including unsupported LazyVim workflow groups and
  Helix-specific alternatives.
- Run `cargo build --release --bin hx`, then run a release Velix smoke check
  with
  `HELIX_RUNTIME=runtime target/release/hx --config <vim-profile-config> <scratch-file>`
  and manually exercise representative interactive workflows from the audit:
  leader pickers that should open, documented unsupported sequences that should
  not claim Vim behavior, and `:write`/`:quit-all` alternatives for unmapped
  LazyVim save/quit keys.
- Run `prettier --write --prose-wrap always --print-width 80` on the changed
  Markdown files.

Pass criteria:

- Every remaining audit category from the Issue 4 open questions is represented
  in `audit.md`.
- Each Experiment 4 row names its keymap assertion, integration/manual check, or
  explicit source-inspection check.
- The user-facing Vim profile documentation matches the tested audit status.
- Each new Different or Deferred documentation claim maps back to an Experiment
  4 audit row.
- The targeted Vim-profile test passes.
- Formatting succeeds.

## Design Review

Reviewer: separate Codex adversarial reviewer.

Initial verdict: Changes required.

- Required: the plan named marks but did not include representative marks
  verification, and the Issue 4 open questions included sessions. Fixed by
  adding marks/jumps and LazyVim session workflow checks to the planned
  assertions.
- Required: verification lacked a manual/editor smoke check. Fixed by adding a
  release Velix smoke check for representative interactive workflows and
  typable-command alternatives.
- Optional: row-to-test/doc traceability was vague. Fixed by requiring each
  Experiment 4 audit row to name its assertion, manual check, or
  source-inspection check, and by requiring new Different/Deferred docs to map
  back to audit rows.

Final verdict: Approved. The reviewer also suggested explicitly building the
release binary before the smoke check; this plan includes that step.

## Result

**Result:** Pass

The remaining Issue 4 audit categories are now represented in `audit.md` with
named verification evidence. The added keymap assertions classify the broad Vim
grammar gaps, Helix alternatives, and common LazyVim workflow gaps that were not
covered by the first three experiments.

Key findings:

- Full Vim operator-pending grammar, operator count multiplication, Vim
  text-object grammar, register-prefix grammar, named marks, linewise Visual
  mode, and blockwise Visual mode remain unsupported for now.
- Helix alternatives are documented for selection-first editing, text objects,
  registers, visual selections, jumplist navigation, save/quit, buffers,
  windows, diagnostics, and pickers.
- Several common LazyVim workflows are plugin- or model-specific and remain
  unsupported: persistence sessions, embedded terminal mappings, Vim tabs, and
  UI toggles.
- Some exact LazyVim LSP aliases are direct follow-up candidates because Velix
  has related commands but not the LazyVim spelling yet: `gI`, `gK`,
  `<leader>cr`, and `<leader>cf`.
- The real test command needs `--features integration`; the design's original
  no-feature command ran zero tests and was corrected here.

Verification run:

- `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`
  - Pass: 15 passed, 0 failed, 176 filtered out.
- `cargo build --release --bin hx`
  - Pass.
- Release smoke check:
  - Ran
    `HELIX_RUNTIME=runtime target/release/hx --config /tmp/velix-vim-smoke-clean.dGZqj9/config.toml /tmp/velix-vim-smoke-clean.dGZqj9/scratch.txt`
    with `keymap = "vim"`.
  - Pressed `Space Space`; the file picker opened.
  - Pressed `Esc`; the picker closed and returned to the scratch buffer.
  - Ran `:write`; Velix reported the scratch file was written.
  - Ran `:quit-all!`; Velix exited cleanly.
  - Verified the scratch file still contained `one` and `two`.
- `cargo fmt`
  - Pass.
- `prettier --write --prose-wrap always --print-width 80` on changed Markdown
  files
  - Pass.

## Completion Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer reported no Required, Optional, or Nit findings. It independently
verified that the result commit had not yet been made, the targeted Vim-profile
tests passed, the release build passed, `cargo fmt --check` passed, Prettier
check passed, the experiment file had Result and Conclusion sections, and the
issue README marked Experiment 4 as Pass.

## Conclusion

The audit is now broad enough to stop treating the remaining keyspace as
unknown. The next experiment should either add the direct LazyVim aliases that
map cleanly to existing Velix commands, or explicitly close the issue with those
aliases deferred.
