# Experiment 2: Publish Public 0.1.0 Homebrew Release

## Description

Publish the first public Velix release to GitHub and Homebrew using the package
shape validated in Experiment 1.

This experiment creates the public release artifacts that Issue 10 requires:

- a `v0.1.0` tag on `astrohackerlabs/velix`;
- a GitHub Release on `astrohackerlabs/velix`;
- the public `astrohackerlabs/homebrew-velix` tap repository;
- a committed and pushed `Formula/velix.rb`;
- a verified install from the public tap.

Experiment 1 proved that the current package bundles macOS `arm64` grammar
libraries and installs successfully on this machine. This first public formula
will therefore be explicit about its target: macOS arm64. It must not imply that
the release artifact is portable to Intel macOS or Linux.

## Changes

- Use the existing committed release tooling from Experiment 1 to generate the
  release archive; do not create a separate closed-source, public mirror, or
  Helix-versioned release repo.
- Generate the release archive from a clean Velix checkout with
  `scripts/release.sh 0.1.0`.
- After the archive is generated, do not rerun `scripts/release.sh` unless the
  archive, SHA, GitHub Release asset, and public formula are all regenerated
  together. The exact tarball whose SHA is recorded must be the tarball uploaded
  to GitHub and referenced by the formula.
- Tag the Velix source repo as `v0.1.0` and push that tag to the explicit
  `astrohackerlabs/velix` remote, not `origin`.
- Create the GitHub Release on `astrohackerlabs/velix` and upload the generated
  `velix-0.1.0-arm64-apple-darwin.tar.gz` artifact.
- Create `astrohackerlabs/homebrew-velix` on GitHub if it does not already
  exist.
- Convert the local `~/dev/homebrew-velix/Formula/velix.rb` from its generated
  local `file://` URL to the public GitHub Release asset URL. After this
  conversion, the tap formula is the source of truth for the public install
  surface; the release script must not be rerun over it.
- Keep the formula source-based for the `vlx` binary but bundled-runtime for
  grammars:
  - `depends_on :macos`;
  - `depends_on arch: :arm64`;
  - `depends_on "rust" => :build`;
  - set `HELIX_DEFAULT_RUNTIME = libexec/"runtime"`;
  - run `cargo install` with `path: "helix-term"`;
  - install `runtime` under `libexec`;
  - install bash, fish, and zsh completions.
- Commit the final public formula locally in `~/dev/homebrew-velix`, then test
  that exact local tap commit before pushing it.
- Push the public tap commit to `astrohackerlabs/homebrew-velix`.
- Record the release URL, tag, artifact SHA, tap repo URL, tap commit, installed
  binary path, runtime path, grammar count, and verification commands.

If a partial publish already exists during implementation, stop and inspect
before proceeding. Do not overwrite a public `v0.1.0` tag, GitHub Release, or
release asset unless the stale object is proven to be from this same experiment
attempt and the deletion/replacement command is recorded in the result.

If `git push upstream main` fails because `upstream/main` has diverged, stop and
inspect the remote history before publishing the tag.

This experiment may publish public artifacts, but only after this design is
approved and committed.

## Verification

- `git status --short --untracked-files=all` is clean before packaging, except
  ignored generated grammar/runtime and `dist/` output.
- `scripts/release.sh 0.1.0` passes without `VELIX_RELEASE_ALLOW_DIRTY=1`.
- `shasum -a 256 dist/velix-0.1.0-arm64-apple-darwin.tar.gz` matches the formula
  SHA and the GitHub Release asset.
- The generated archive is not regenerated between SHA calculation, GitHub
  Release upload, formula conversion, local formula test, and tap push.
- `git push upstream main` pushes the release commit line to
  `astrohackerlabs/velix`.
- `git tag -s v0.1.0` or `git tag -a v0.1.0` creates the release tag. If signing
  is unavailable, record the reason for using an annotated unsigned tag.
- `git push upstream v0.1.0` publishes the tag to `astrohackerlabs/velix`.
- `gh release create v0.1.0 ... --repo astrohackerlabs/velix` creates the
  release with the generated archive attached.
- `gh release view v0.1.0 --repo astrohackerlabs/velix --json url,tagName`
  confirms the public release.
- `gh repo view astrohackerlabs/homebrew-velix --json nameWithOwner,url`
  confirms the tap repository exists.
- `brew audit --strict --online ~/dev/homebrew-velix/Formula/velix.rb` passes or
  any tap-only warnings are recorded and judged non-blocking.
- Local final-formula test before tap push:
  - commit the public formula locally in `~/dev/homebrew-velix`;
  - remove any installed `velix`;
  - untap and retap `astrohackerlabs/velix` from
    `/Users/astrohacker/dev/homebrew-velix`;
  - verify the tapped clone contains the public URL, public SHA,
    `depends_on :macos`, and `depends_on arch: :arm64`;
  - `brew install --build-from-source astrohackerlabs/velix/velix` passes;
  - `$(brew --prefix)/bin/vlx --version` prints `velix 0.1.0`;
  - `$(brew --prefix)/bin/vlx --health rust` shows tree-sitter parser and query
    support available;
  - `brew test astrohackerlabs/velix/velix` passes.
- `git -C ~/dev/homebrew-velix status --short --untracked-files=all` is clean
  after committing and pushing the formula.
- Clean public install flow:
  - remove the local formula test install and local tap;
  - `brew tap astrohackerlabs/velix`;
  - `brew install --build-from-source astrohackerlabs/velix/velix`;
  - `$(brew --prefix)/bin/vlx --version` prints `velix 0.1.0`;
  - `$(brew --prefix)/bin/vlx --help` succeeds;
  - `$(brew --prefix)/bin/vlx --health rust` shows tree-sitter parser and query
    support available;
  - installed runtime path and grammar count are recorded;
  - `brew test astrohackerlabs/velix/velix` passes.
- `scripts/build-issues-index.sh`
- `git diff --check`

## Design Review

Claude external review via `skills/claude-review`: **Changes required**.

Required findings and resolutions:

- The original design was self-contradictory because `scripts/release.sh` would
  regenerate a local `file://` formula after the design asked for a public
  guarded formula. Resolved by using the script only for archive generation and
  making the converted tap formula the public source of truth after conversion.
- The original design pushed the public tap before installing the exact final
  public formula. Resolved by requiring a local install test from the committed
  but unpushed public formula before pushing the tap.

Additional review findings addressed:

- Use declarative platform constraints with `depends_on :macos` and
  `depends_on arch: :arm64`.
- Preserve byte identity between the hashed archive, uploaded release asset, and
  formula SHA by not regenerating the archive between those steps.
- Stop and inspect before recovering from any partial tag, Release, asset, or
  divergent `upstream/main` state.

Re-review: **Approved**. Claude confirmed the revised design resolves the
formula clobbering and pre-push install-test blockers, uses declarative platform
constraints, preserves archive byte identity across upload and formula SHA, and
records stop-and-inspect recovery for partial publish states.

Review artifacts:

- Initial prompt: `logs/claude-review/20260629-063522-785774-prompt.md`
- Initial raw output: `logs/claude-review/20260629-063522-785774-stdout.json`
- Re-review prompt: `logs/claude-review/20260629-063823-717463-prompt.md`
- Re-review raw output: `logs/claude-review/20260629-063823-717463-stdout.json`
