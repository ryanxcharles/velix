# Experiment 1: Prepare Local 0.1.0 Homebrew Release

## Description

Prepare the Velix `0.1.0` Homebrew release locally without creating public tags,
GitHub Releases, or pushing the tap.

This experiment handles the irreversible-release prerequisites: Velix version
identity, stable source archive generation, local tap/formula authoring, and a
local Homebrew install test from a package-only artifact. A later experiment
will publish the already-validated tag, GitHub Release, and tap.

The release will use only:

- source repo: `github.com/astrohackerlabs/velix`;
- tap repo: `github.com/astrohackerlabs/homebrew-velix`.

## Changes

- Update Velix release identity:
  - set workspace package version to `0.1.0`;
  - update repository/homepage metadata to `astrohackerlabs/velix`;
  - update version formatting so `vlx --version` prints `velix 0.1.0` for this
    release instead of Helix calver formatting.
- Add repeatable release tooling, likely `scripts/release.sh`, with a
  package-only mode that can:
  - verify a clean source checkout;
  - verify `vlx --version` matches the requested version;
  - create a stable source archive release asset;
  - compute the archive SHA256;
  - write a local Homebrew formula that points at that package-only archive;
  - optionally publish only when an explicit publish flag is set.
- Use an explicit source remote or URL for future publishing so the local
  `origin` remote cannot accidentally publish tags to `ryanxcharles/velix`.
- Decide and implement the runtime grammar strategy for Homebrew:
  - use a stable release source archive uploaded by the release script;
  - make the formula set `HELIX_DEFAULT_RUNTIME=libexec/"runtime"` at build
    time;
  - install committed runtime assets under `libexec/runtime`;
  - fetch and build tree-sitter grammars during formula installation into the
    installed runtime, or bundle prebuilt grammars in the release artifact if
    formula-time grammar builds are not viable;
  - declare any required Homebrew build dependencies for the chosen strategy.
- Add or update Velix release/Homebrew documentation if needed.
- Create or initialize `~/dev/homebrew-velix` locally without pushing.
- Add local `Formula/velix.rb` to `~/dev/homebrew-velix`.
- Verify local Homebrew installation from the package-only archive and local tap
  formula.
- Record the package-only archive path, SHA, local tap path, and verification
  evidence in this experiment file.

This experiment must not:

- create or push `v0.1.0`;
- create a GitHub Release;
- create or push `astrohackerlabs/homebrew-velix`;
- publish a Homebrew tap commit.

## Verification

- `cargo fmt`
- A test or script-level assertion covers the new version formatting for
  `0.1.0`, including the no-git/source-archive path.
- `cargo build --release -p helix-term`
- `target/release/vlx --version` prints `velix 0.1.0` or `velix 0.1.0 (<hash>)`
  before packaging from the git checkout.
- Package-only release command succeeds and records the SHA256 for the exact
  archive referenced by the local formula.
- `tar tf <package-only-archive>` includes Cargo files, `helix-term`, and
  runtime themes/queries.
- `~/dev/homebrew-velix/Formula/velix.rb` exists and points at the package-only
  archive SHA.
- Local Homebrew install succeeds from the local formula. If a prior install
  exists, use a clean reinstall flow and record the commands.
- `$(brew --prefix)/bin/vlx --version` reports `velix 0.1.0`.
- Runtime verification proves more than process exit:
  - `$(brew --prefix)/bin/vlx --health rust` shows runtime query/highlight
    assets and grammar support as available, or an equivalent explicit grammar
    count/highlight smoke proves installed grammars are present;
  - the installed runtime path is recorded.
- A basic non-interactive launch smoke, such as `vlx --help`, succeeds from the
  Homebrew-installed binary.
- `brew uninstall velix` or a documented cleanup/retain decision is recorded.
- `gh repo view astrohackerlabs/velix --json nameWithOwner,url`
- `scripts/build-issues-index.sh`
- `git diff --check`

## Design Review

Claude external review via `skills/claude-review`: **Changes required**.

Required findings and resolutions:

- Runtime grammar handling was undefined, so a source archive would likely ship
  a grammarless editor. Resolved by requiring the experiment to choose and
  implement a grammar strategy and verify grammar availability.
- The original design bundled local preparation with irreversible public
  publishing, so a failed install could burn a public `v0.1.0` tag. Resolved by
  narrowing Experiment 1 to local/package-only validation and deferring public
  publishing to a separate experiment.
- Verification could not distinguish a working release from a broken one.
  Resolved by requiring installed-version verification, explicit grammar/runtime
  checks, and a source-archive version-format assertion.

Should-fix findings addressed:

- The design now requires explicit source remote/URL selection for future
  publishing so `origin` cannot accidentally push to the wrong repository.
- The design now requires a stable release source archive and SHA for the
  formula instead of relying on ambiguous generated tag archives.
- The design now calls out the no-git/source-archive version path so the
  `build.rs` change is verified where it matters.

Re-review: **Approved**. Claude confirmed the revised design resolves runtime
grammar handling, separates local validation from public publishing, strengthens
verification for installed version/runtime behavior, and addresses the remote
selection, stable archive SHA, and no-git version-format concerns.

## Result

**Result:** Pass

Implemented the local release preparation path and verified a Homebrew install
from a local tap.

Changed release identity and packaging:

- set the workspace package version to `0.1.0`;
- changed workspace repository/homepage metadata to
  `https://github.com/astrohackerlabs/velix`;
- changed `helix-loader/build.rs` to use the literal Cargo package version
  instead of Helix calver formatting, preserving the short git hash only when
  available;
- added `scripts/release.sh` for package-only release validation;
- added `docs/homebrew.md` with the local validation flow;
- installed bash, fish, and zsh completions from the formula;
- ignored generated `dist/` output.

The package strategy bundles built runtime grammars in the release archive.
`scripts/release.sh` builds `vlx`, ensures runtime grammars exist, excludes
`runtime/grammars/sources/`, writes a local formula, and refuses public
publishing with `VELIX_RELEASE_PUBLISH=1` until a later reviewed experiment.

Local release evidence:

- `VELIX_RELEASE_ALLOW_DIRTY=1 scripts/release.sh 0.1.0` passed.
- Package: `dist/velix-0.1.0-arm64-apple-darwin.tar.gz`
- SHA256: `1a328bd0f0bbfb259bc56e2628f3fa160ae399f1ca98111bea828dba409c252a`
- Formula: `/Users/astrohacker/dev/homebrew-velix/Formula/velix.rb`
- Local tap commit: `5b88b1e Add local Velix formula`
- Source repo check:
  `{"nameWithOwner":"astrohackerlabs/velix","url":"https://github.com/astrohackerlabs/velix"}`

Verification commands and results:

- `cargo fmt` passed.
- `bash -n scripts/release.sh` passed.
- `cargo build --release -p helix-term` passed inside the release script.
- `target/release/vlx --version` during packaging printed
  `velix 0.1.0 (b453ac78)`.
- `tar tf dist/velix-0.1.0-arm64-apple-darwin.tar.gz` showed `Cargo.toml`,
  `helix-term/`, runtime themes/queries, and `runtime/grammars/rust.dylib`.
- `brew tap astrohackerlabs/velix /Users/astrohacker/dev/homebrew-velix` passed.
- `brew install --build-from-source astrohackerlabs/velix/velix` passed and
  installed `/opt/homebrew/Cellar/velix/0.1.0`.
- `$(brew --prefix)/bin/vlx --version` printed `velix 0.1.0`, proving the
  source-archive/no-git version path.
- `$(brew --prefix)/bin/vlx --help` printed the Velix help text.
- `$(brew --prefix)/bin/vlx --health rust` showed:
  - `Tree-sitter parser` available;
  - `Highlight queries` available;
  - `Textobject queries` available;
  - `Indent queries` available;
  - `Tags queries` available;
  - `Rainbow queries` available.
- Installed runtime path: `/opt/homebrew/Cellar/velix/0.1.0/libexec/runtime`
- Installed grammar count: `293`
- Installed Rust grammar:
  `/opt/homebrew/Cellar/velix/0.1.0/libexec/runtime/grammars/rust.dylib`
- Installed Cellar size after the corrected formula install: `2.3G`
- `brew test astrohackerlabs/velix/velix` passed.
- `scripts/build-issues-index.sh` passed with `1 open, 9 closed`.
- `git diff --check` passed.

Cleanup decision: keep the local Homebrew install and local tap in place for the
next publishing experiment, where the public tap will replace this local tap.

No public tag, GitHub Release, GitHub tap repository, or tap push was created in
this experiment.

## Conclusion

The local package contract works. Velix can build as version `0.1.0`, the source
archive preserves that version when built outside a git checkout, the formula
installs `vlx`, and the installed runtime contains working grammars and queries.

The next experiment should publish the validated release publicly: commit this
result, design the public publish step, create/push `v0.1.0`, create the GitHub
Release with the tested archive shape, create `astrohackerlabs/homebrew-velix`,
push the formula with a public release URL, and verify installation from the
public tap.

The publish design must account for the platform-specific grammar libraries and
large installed grammar footprint in the local artifact. The local package
bundles macOS `arm64` `.dylib` grammars and installed at `2.3G`, so the public
release should either publish platform-specific artifacts or build grammars
during formula installation instead of presenting one archive as portable across
all Homebrew hosts.

## Completion Review

External Claude review via `skills/claude-review`: **Approved**.

Claude found no blocking issues and approved the result commit. Non-blocking
findings were handled before commit:

- The local archive bundles platform-specific grammar libraries. Recorded this
  as a carry-forward constraint for the public publish experiment.
- The generated formula installs shell completions. Recorded that in the result.
- The local validation docs used a direct formula path even though Homebrew
  required a tapped formula. Updated the docs to use the local tap flow.
- The generated formula used verbose cargo output and had a redundant
  `runtime/grammars/sources` removal. Removed both from the script.

Review artifacts:

- Prompt: `logs/claude-review/20260629-062418-248891-prompt.md`
- Raw output: `logs/claude-review/20260629-062418-248891-stdout.json`
