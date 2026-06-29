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
