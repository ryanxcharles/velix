# Homebrew

Velix will ship through the `astrohackerlabs/velix` Homebrew tap, backed by the
`astrohackerlabs/homebrew-velix` repository.

The first release target is `0.1.0`.

## Local Package Validation

Before publishing, build a package-only release and local formula:

```bash
VELIX_RELEASE_ALLOW_DIRTY=1 scripts/release.sh 0.1.0
```

The script builds `vlx`, ensures runtime grammars are present, creates a local
release archive under `dist/`, and writes
`~/dev/homebrew-velix/Formula/velix.rb` against that archive.

Install from the local formula for validation:

```bash
brew tap astrohackerlabs/velix ~/dev/homebrew-velix
brew install --build-from-source astrohackerlabs/velix/velix
vlx --version
vlx --health rust
```

Public publishing is handled by a separate reviewed release step.
