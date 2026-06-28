# Experiment 5: Search, jumps, and marks

## Description

Issue 6 requires auditing common Vim keybindings in every relevant mode. Search
and jump keys are already present in the Vim profile keymap assertions, but they
need stronger mode-specific runtime evidence, and named Vim marks are still only
classified through broad missing-key assertions.

This experiment will audit and verify the common search/jump/mark slice:

- Normal `/`, `?`, `n`, and `N`: search forward, search backward, and repeat in
  both directions.
- Normal `*`: search for the word under the cursor with word-boundary behavior.
- Select-mode `n` and `N`: extend the active selection to next/previous search
  matches.
- Normal `C-o` and `C-i`: jump backward and forward through Velix's jumplist.
- Normal `C-s`: save the current selection to the jumplist as the Helix/Velix
  alternative to Vim named marks.
- Normal `ma`, `'a`, and `` `a ``: document that Vim named marks and mark jumps
  are not implemented because `ma` / `mi` are already Helix text-object
  selection commands.

The goal is to avoid overclaiming from keymap presence alone. Search and jump
bindings should have executable behavior tests where the integration harness can
prove the cursor/selection result. Named marks should remain explicitly
unsupported unless this experiment finds an existing compatible Velix command
surface.

## Changes

- `helix-term/tests/test/commands/vim_profile.rs`
  - Add integration tests for forward and reverse search, repeated search with
    `n` / `N`, `*` word search, select-mode `n` / `N` extension, and jumplist
    `C-o` / `C-i` behavior.
  - Keep precise missing assertions for Vim named mark jumps (`'a`, `` `a ``)
    and keymap assertions showing `ma` / `mi` remain Helix text-object selection
    commands.
- `issues/0006-cross-mode-vim-keybinding-audit/audit.md`
  - Add Experiment 5 rows for each audited binding by mode.
  - Correct older rows if they describe search, jumps, or marks too broadly.
- `book/src/vim-profile.md`
  - Document tested search and jump behavior.
  - Keep named marks in the deferred/unsupported section with the Velix jumplist
    alternative.

## Verification

- Run `cargo fmt`.
- Run
  `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`.
- Run `prettier --write --prose-wrap always --print-width 80` on changed
  Markdown files.

Pass criteria:

- Normal `/`, `?`, `n`, `N`, and `*` have executable tests proving cursor
  movement or selected match behavior.
- Select-mode `n` and `N` have mode-specific executable tests proving selection
  extension behavior.
- `C-o` and `C-i` have executable tests proving Velix jumplist traversal in the
  Vim profile.
- `C-s` is documented as the Velix jumplist-save alternative, not as Vim or
  LazyVim save-file behavior.
- `ma`, `'a`, and `` `a `` are not overclaimed: if named marks remain
  unsupported, the audit and docs explain that Helix text-object commands occupy
  `ma` / `mi` and provide the jumplist alternative.

## Design Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer found no Required issues. It confirmed the README links Experiment
5 with status `Designed`, the experiment has the required design sections, the
scope stays within search, jump, and mark basics, and named marks are not
overclaimed. It also checked that the existing source maps normal search and
jumplist keys plus select-mode search extension keys, and that no implementation
or result section was present before the plan commit.

## Result

**Result:** Pass

Experiment 5 added executable coverage for normal-mode search, select-mode
search repeat keys, and jumplist traversal. Normal `/`, `?`, `n`, and `N` work
through Velix's regex prompt and search register state. `C-o` and `C-i` traverse
the Velix jumplist after a jump.

The experiment also recorded two deliberate differences. `*` maps to Helix's
current-selection search command with word-boundary detection; with the default
one-character cursor selection, tested `*n` searches the next matching
character, not Vim's full word-under-cursor target. Select-mode `n` and `N` use
Helix's `extend_search_next` / `extend_search_prev` semantics and active search
direction state rather than exact Vim Visual search behavior.

Named Vim marks remain unsupported in this slice. `ma` and `mi` are Helix
text-object commands, while `'a` and `` `a `` remain unmapped. The documented
Velix alternative is the jumplist flow: `C-s`, `C-o`, `C-i`, and `Space j`.

Verification run:

- `cargo fmt`
  - Pass.
- `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`
  - Pass: 44 passed, 0 failed, 176 filtered out.
- `prettier --write --prose-wrap always --print-width 80` on changed Markdown
  files
  - Pass.

## Completion Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved after fixes.

The reviewer initially required stronger runtime evidence for `n`, `N`, `C-o`,
and `C-i`. The first implementation asserted only the final position of combined
sequences (`/beta<ret>nN` and `G<C-o><C-i>`), which could have passed even if
the middle keys were no-ops. The tests were split so `/beta<ret>n` proves
forward repeat search, `/beta<ret>nN` proves reverse repeat back to the prior
match, `G<C-o>` proves jump-back behavior, and `G<C-o><C-i>` proves forward
jumplist traversal. The `*` test was also renamed to reflect the tested
current-selection behavior. After these fixes, the Vim-profile integration test
suite passed with 44 tests.

## Conclusion

Search and jumplist basics now have mode-specific runtime evidence. The main
remaining gaps from this slice are deeper Vim mark support and exact Vim
word-under-cursor `*` behavior, both of which require behavior beyond the
current direct keymap aliases.
