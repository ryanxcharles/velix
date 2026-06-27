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
- Run `cargo test -p helix-term --test integration vim_profile -- --nocapture`.
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
