# Experiment 1: Use Tokyo Night as the true-color default

## Description

Make Tokyo Night the default Velix theme for normal true-color terminals while
preserving the existing explicit theme configuration behavior and the
non-true-color fallback.

This experiment should also resolve the placement of the provided
`tokyonight.toml` file. The repository already contains
`runtime/themes/tokyonight.toml`, and the root `tokyonight.toml` appears to
match it except for the existing runtime file's author comment. The experiment
should verify that relationship and avoid leaving a duplicate root-level theme
file.

## Changes

Expected changes:

- `runtime/themes/tokyonight.toml`
  - Keep Tokyo Night in the runtime theme directory, which is the appropriate
    location for named bundled themes.
  - If the root `tokyonight.toml` contains meaningful changes compared with the
    runtime file, merge them into the runtime file.
  - If the root file is only a duplicate of the runtime theme, remove the root
    copy from the worktree.
- `helix-view/src/theme.rs`
  - Change `Loader::default_theme(true_color)` so true-color terminals default
    to loading the named `tokyonight` runtime theme.
  - Because `default_theme` is an infallible API, loading `tokyonight` must not
    panic. If the runtime theme is missing or malformed, log a warning and fall
    back to the embedded `default` theme.
  - Keep the existing `base16_default` fallback for non-true-color terminals.
  - Keep `Loader::default()` and the reserved `default` theme name intact unless
    code inspection proves changing the embedded `theme.toml` is cleaner.
  - Add tests proving the new default selection behavior and the missing-theme
    fallback.
- `helix-term/src/application.rs`
  - Leave explicit user `theme = "..."` configuration precedence intact. Change
    this file only if tests show the startup fallback needs adjustment.
- `book/src/themes.md`
  - Update user-facing documentation if it currently implies that no-config
    startup uses the old built-in `default` theme.
- `issues/0003-default-tokyo-night-theme/README.md`
  - Update this issue's experiment status and conclusion when the result is
    known.

Do not modify closed issues.

## Verification

This experiment passes when all of the following are true:

1. Tokyo Night lives in `runtime/themes/tokyonight.toml`, and there is no
   duplicate `tokyonight.toml` at the repository root.
2. `Loader::default_theme(true)` resolves to a theme named `tokyonight`.
3. `Loader::default_theme(false)` still resolves to `base16_default`.
4. If `tokyonight` cannot be loaded from the configured theme directories,
   `Loader::default_theme(true)` returns the embedded `default` theme instead of
   panicking.
5. Explicit user theme configuration still wins over the default path. This can
   be proven either by an application/config test or by showing that the
   existing `helix-term/src/application.rs` branch still loads
   `config.theme.as_ref()` before calling `default_theme`.
6. The Tokyo Night theme parses through the normal theme loader.
7. Markdown formatting is run:

   ```bash
   prettier --write --prose-wrap always --print-width 80 \
     issues/0003-default-tokyo-night-theme/README.md \
     issues/0003-default-tokyo-night-theme/01-use-tokyonight-default.md \
     issues/README.md \
     book/src/themes.md
   ```

8. The issue index is regenerated with:

   ```bash
   scripts/build-issues-index.sh
   ```

9. Rust formatting and tests pass:

   ```bash
   cargo fmt
   cargo fmt --check
   cargo test -p helix-view theme
   ```

Before implementation begins, this design must be reviewed by another AI agent
and approved. Record the review result in this file, then commit the approved
plan.

## Design Review

**Reviewer:** Fresh-context Codex adversarial reviewer.

**Initial verdict:** Changes required.

The reviewer found that the design did not specify failure behavior for making
the infallible `Loader::default_theme(true)` path load a runtime theme. A naive
implementation could panic if `tokyonight` were missing or malformed.

The reviewer also noted that the broad `cargo test -p helix-term config`
verification command would not prove explicit-theme precedence through
`Application::load_configured_theme`.

**Fixes made:**

- Required `Loader::default_theme(true)` to attempt `tokyonight` without
  panicking, logging a warning and falling back to the embedded `default` theme
  if loading fails.
- Added a missing-theme fallback verification requirement.
- Removed the broad `helix-term config` command from required verification;
  explicit-theme precedence remains a source-inspection or focused-test
  requirement.

**Final verdict:** Approved. No Required findings remained.

## Result

**Result:** Pass

Implemented Tokyo Night as the true-color default theme by changing
`Loader::default_theme(true)` to load the bundled `tokyonight` runtime theme.
The method remains infallible: if `tokyonight` cannot be loaded, it logs a
warning and falls back to the embedded `default` theme.

The non-true-color path still returns `base16_default`.

The provided root `tokyonight.toml` was compared against
`runtime/themes/tokyonight.toml`. After ignoring the existing author comment and
blank lines, the contents matched exactly, so Tokyo Night was already in the
appropriate runtime location. The duplicate root file was removed from the
worktree.

No user-facing theme documentation change was needed because
`book/src/themes.md` does not claim that no-config startup uses the old embedded
`default` theme.

Verification completed:

- Confirmed `runtime/themes/tokyonight.toml` exists and the duplicate root
  `tokyonight.toml` is gone.
- Added tests proving:
  - `Loader::default_theme(true)` resolves to `tokyonight`;
  - `Loader::default_theme(false)` resolves to `base16_default`;
  - `Loader::default_theme(true)` falls back to embedded `default` if
    `tokyonight` is unavailable;
  - `Loader::default_theme(true)` falls back to embedded `default` if
    `tokyonight.toml` cannot be parsed.
- Confirmed explicit theme configuration precedence by source inspection:
  `helix-term/src/application.rs` still loads `config.theme.as_ref()` before
  calling `editor.theme_loader.default_theme(true_color)`.
- Ran markdown formatting and regenerated the issue index.
- Ran:

  ```bash
  cargo fmt
  cargo fmt --check
  cargo test -p helix-view theme
  ```

## Conclusion

The provided Tokyo Night theme is correctly represented by the existing bundled
`runtime/themes/tokyonight.toml`. Velix now defaults to that theme on true-color
terminals while preserving `base16_default` for non-true-color terminals and
preserving explicit user theme configuration.

## Completion Review

**Reviewer:** Fresh-context Codex adversarial reviewer.

**Initial verdict:** Changes required.

The reviewer found two required issues:

- The required markdown formatting command included `book/src/themes.md`, but
  that file had not been formatted and failed `prettier --check`.
- The malformed-theme fallback claim was under-proven because the test only
  covered a missing runtime path.

**Fixes made:**

- Ran prettier over the full required markdown file set, including
  `book/src/themes.md`.
- Added a temp-runtime test with a syntactically malformed
  `themes/tokyonight.toml` proving that `Loader::default_theme(true)` falls back
  to embedded `default` on TOML parse failure.

**Verification after fixes:**

```bash
cargo fmt
cargo fmt --check
cargo test -p helix-view theme
prettier --check --prose-wrap always --print-width 80 \
  issues/0003-default-tokyo-night-theme/README.md \
  issues/0003-default-tokyo-night-theme/01-use-tokyonight-default.md \
  issues/README.md \
  book/src/themes.md
```

**Final verdict:** Approved. No Required findings remained.

The reviewer independently verified `cargo fmt --check`,
`cargo test -p helix-view theme`, the full required prettier check, root
`tokyonight.toml` removal, continued presence of
`runtime/themes/tokyonight.toml`, and that the result commit had not been made
before approval.
