# Experiment 1: Make Vim profile the default

## Description

Velix currently defines a Vim keymap profile, but `Config::default()` and config
loading without `editor.keymap` still use the Helix-style default keymap. That
means Vim-profile fixes such as bare `G` going to the end of the document do not
work in a fresh Velix run.

This experiment will make the Vim profile the default base keymap whenever no
explicit profile is configured, while preserving explicit
`editor.keymap = "default"` as the escape hatch for the Helix-style keymap.

## Changes

- `helix-term/src/config.rs`
  - Change `Config::default()` to use `keymap::vim()`.
  - Change missing-profile fallbacks in `Config::load` from
    `KeymapProfile::Default` to `KeymapProfile::Vim`.
  - Update config tests so empty config and `Config::default()` resolve to the
    Vim profile.
  - Add or update tests proving explicit `keymap = "default"` still resolves to
    the Helix-style keymap, local profile still overrides global profile, and
    user `[keys]` remaps still merge over the default Vim profile.
- `helix-term/tests/test/commands/vim_profile.rs`
  - Update the default-keymap preservation test so the no-config case now proves
    bare `G` uses Vim behavior out of the box.
  - Keep explicit `keymap = "default"` coverage for Helix-style behavior where
    needed.
- `book/src/vim-profile.md`
  - Update the wording from opt-in Vim profile to default Vim profile.
  - Document `keymap = "default"` as the way to choose the Helix-style keymap.

## Verification

- Run `cargo fmt`.
- Run `cargo test -p helix-term config::tests:: --lib`.
- Run
  `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`.
- Run `prettier --write --prose-wrap always --print-width 80` on changed
  Markdown files.

Pass criteria:

- Empty config and `Config::default()` use `keymap::vim()`.
- Explicit `editor.keymap = "default"` still uses `keymap::default()`.
- Local profile selection still overrides global profile selection.
- User key remaps still merge over the default Vim profile.
- A no-config integration test proves bare `G` goes to the end of the document.
- Documentation clearly says Vim is the default and Helix-style keys are
  selected with `keymap = "default"`.

## Design Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer reported no Required findings. It independently checked that the
issue README links this experiment as Designed, the experiment has the required
sections and pass criteria, the scope is narrow, the technical plan matches the
current config architecture, and the verification covers empty/default config,
explicit `default`, local/global precedence, user key merges, and no-config `G`
behavior. It also ran read-only checks: `cargo fmt --check`,
`cargo test -p helix-term config::tests:: --lib --no-run`, and
`cargo test -p helix-term --features integration --test integration vim_profile -- --list`.

## Result

**Result:** Pass

The Vim profile is now the default keymap for config loading and
`Config::default()`. Missing `editor.keymap` falls back to `keymap::vim()`,
while explicit `editor.keymap = "default"` still loads the Helix-style keymap.

The integration test helper still merges partial config overrides over
`Config::default()`, so no-config and partial-config integration tests exercise
the same Vim default. A new exact-config helper lets tests use the exact keymap
produced by config loading. The updated integration tests prove that bare `G`
goes to the end of the document without config, and that explicit
`keymap = "default"` preserves Helix-style key behavior without residual
Vim-only bindings from the helper.

Verification run:

- `cargo fmt`
  - Pass.
- `cargo test -p helix-term config::tests:: --lib`
  - Pass: 7 passed, 0 failed.
- `cargo test -p helix-term --features integration --test integration vim_profile -- --nocapture`
  - Pass: 16 passed, 0 failed, 176 filtered out.
- `cargo test -p helix-term --features integration --test integration test_jump_undo_redo -- --nocapture`
  - Pass: 1 passed, 0 failed, 191 filtered out.
- `prettier --write --prose-wrap always --print-width 80 book/src/vim-profile.md`
  - Pass.

Completion review initially requested one helper fix: `AppBuilder::with_config`
must not make explicit `editor.keymap = "default"` integration tests retain
Vim-only bindings by merging a loaded default-profile config over the Vim
default. The fix keeps `with_config` for partial override configs and adds
`with_exact_config` for tests that need the exact keymap produced by
`Config::load`.

## Conclusion

The default keymap behavior now matches Velix's Vim-first goal. Users who want
the Helix-style keymap can still opt into it with `editor.keymap = "default"`,
and user key remaps still merge over the selected base profile.

## Completion Review

Reviewer: separate Codex adversarial reviewer.

Final verdict: Approved.

The reviewer initially requested one helper fix: explicit
`editor.keymap = "default"` integration tests should not merge a loaded
default-profile config over the Vim default, because that could leave residual
Vim-only bindings. After the helper split and added `with_exact_config` usage,
the reviewer approved the result with no Required findings. It independently
verified `cargo fmt --check`, the config unit tests, the focused Vim-profile
integration tests, the `test_jump_undo_redo` partial-override integration test,
and Prettier checks on changed Markdown.
