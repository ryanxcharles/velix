# Experiment 6: Prompt command-line basics

## Description

Issue 6 calls out command/search/prompt modes as relevant Vim compatibility
surfaces. Velix does not model these as a separate Vim keymap mode; command,
search, shell, and similar prompts are handled by `helix-term/src/ui/prompt.rs`.
That component has prompt-local editing behavior for common command-line keys
such as `C-r`, `C-w`, `C-u`, cursor movement, history, and completion.

This experiment will audit the prompt-command-line slice that can be tested
without depending on terminal UI rendering:

- Search prompt `C-u`: delete prompt text back to the start of the line.
- Search prompt `C-w`: delete the previous word in the prompt.
- Search prompt `C-r {register}`: insert register contents into the prompt.
- Search prompt `C-b` / `C-f` and `C-a` / `C-e`: move within prompt text.
- Prompt `Esc` / `C-c`: abort the prompt.
- Prompt `Up` / `Down` and `C-p` / `C-n`: history navigation where a prompt has
  a history register.
- Prompt `Tab` / `S-Tab`: completion selection where completions exist.

The audit will classify these as prompt-local behavior, not Vim-profile keymap
entries. If a Vim command-line behavior is implemented by Helix prompt
semantics, the audit should say so. If behavior differs or cannot be
meaningfully tested without brittle UI coupling, the audit should document the
reason and the Velix alternative.

## Changes

- `helix-term/tests/test/commands/vim_profile.rs`
  - Add search-prompt integration tests for stable prompt editing behavior:
    `C-u`, `C-w`, `C-r {register}`, and cursor movement controls where they can
    be proven by the resulting search match.
  - Add keymap or runtime evidence that prompt behavior is not controlled by a
    separate Vim-profile keymap mode.
- `issues/0006-cross-mode-vim-keybinding-audit/audit.md`
  - Add Experiment 6 rows for prompt-local command-line bindings.
  - Mark prompt behavior as `Works`, `Different by design`, or
    `Unsupported for now` with prompt-specific evidence.
- `book/src/vim-profile.md`
  - Document the tested prompt-local command-line controls and distinguish them
    from Vim normal/select/insert keymap entries.

## Verification

- Run `cargo fmt`.
- Run
  `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`.
- Run `prettier --write --prose-wrap always --print-width 80` on changed
  Markdown files.

Pass criteria:

- Prompt-local `C-u`, `C-w`, `C-r {register}`, and cursor movement controls have
  executable tests or precise documentation explaining why they are prompt-local
  rather than Vim-profile keymap entries.
- The audit does not claim prompt behavior is configured through
  `[keys.normal]`, `[keys.select]`, or `[keys.insert]`.
- User-facing docs describe the tested command/search prompt behavior and any
  differences from Vim command-line editing.
- Remaining prompt behaviors that are not tested are documented with a precise
  reason, not left as an unclassified Issue 6 gap.

## Design Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer found no Required issues. It confirmed Experiment 6 is linked from
the issue README with status `Designed`, the experiment has the required design
sections, and no result or implementation is present. It also confirmed prompt
behavior is handled by `helix-term/src/ui/prompt.rs`, not by Vim
normal/select/insert keymaps, and that Markdown formatting plus
`cargo fmt --check` pass in the plan-only state.

## Result

**Result:** Pass

Experiment 6 audited the prompt-command-line slice. Velix prompt behavior is
implemented in `helix-term/src/ui/prompt.rs`, not through the Vim-profile
normal/select/insert keymaps. The experiment added search-prompt integration
tests that prove prompt-local `C-u`, `C-w`, `C-r {register}`, and cursor
movement controls by observing the resulting search match. The runtime cursor
movement test covers `C-b` and `C-e`; companion prompt movement keys such as
`C-f` and `C-a` are recorded from `Prompt::handle_event` source inspection.

The audit also records prompt-local `Esc` / `C-c`, history navigation, and
completion navigation by source inspection. Those controls are prompt component
behavior and apply where a prompt has a history register or completion list.

Verification run:

- `cargo fmt`
  - Pass.
- `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`
  - Pass: 48 passed, 0 failed, 176 filtered out.
- `prettier --write --prose-wrap always --print-width 80` on changed Markdown
  files
  - Pass.

## Completion Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved after fixes.

The reviewer initially required one audit correction: the prompt cursor-movement
row claimed both `C-b` and `C-f` were runtime-tested, but only `C-b` was covered
by the search-prompt integration test. The audit was updated to distinguish
runtime-tested `C-b` / `C-e` behavior from source-inspected `C-f` / `C-a`
behavior, and the result text was narrowed to match. The reviewer rechecked the
source, formatting, and Vim-profile test suite, then approved with no Required
findings.

## Conclusion

The command/search/prompt requirement is now classified. Search prompt editing
has executable evidence for common Vim command-line controls where stable, and
the remaining prompt-local keys are documented from the prompt component rather
than misclassified as Vim keymap entries.
