#!/usr/bin/env bash
set -euo pipefail

# Build a local Velix release package and Homebrew formula.
#
# Package-only mode is the default for now. Public publishing is intentionally
# left for a separate reviewed experiment.
#
# Usage:
#   scripts/release.sh [version]
#
# Environment:
#   VELIX_HOMEBREW_TAP_REPO  local tap checkout path
#   VELIX_RELEASE_ALLOW_DIRTY allow package creation from a dirty checkout

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
VERSION="${1:-0.1.0}"
ARCH="$(uname -m)-apple-darwin"
DIST_DIR="$REPO_DIR/dist"
PACKAGE_ROOT_NAME="velix-${VERSION}-${ARCH}"
PACKAGE_ROOT="$DIST_DIR/package/$PACKAGE_ROOT_NAME"
TARBALL="$DIST_DIR/${PACKAGE_ROOT_NAME}.tar.gz"
HOMEBREW_TAP_REPO="${VELIX_HOMEBREW_TAP_REPO:-$HOME/dev/homebrew-velix}"
FORMULA_DIR="$HOMEBREW_TAP_REPO/Formula"
FORMULA_FILE="$FORMULA_DIR/velix.rb"

grammar_glob() {
  case "$(uname -s)" in
    Darwin) echo "*.dylib" ;;
    *) echo "*.so" ;;
  esac
}

die() {
  echo "error: $*" >&2
  exit 1
}

workspace_version() {
  awk '
    /^\[workspace\.package\]$/ { in_workspace = 1; next }
    /^\[/ { in_workspace = 0 }
    in_workspace && /^version[[:space:]]*=/ {
      gsub(/"/, "", $3)
      print $3
      exit
    }
  ' "$REPO_DIR/Cargo.toml"
}

check_version_metadata() {
  local cargo_version
  cargo_version="$(workspace_version)"
  [ "$cargo_version" = "$VERSION" ] ||
    die "workspace version is $cargo_version, expected $VERSION"
}

check_worktree() {
  if [ "${VELIX_RELEASE_ALLOW_DIRTY:-0}" = "1" ]; then
    echo "==> Dirty worktree allowed for local package-only validation."
    return
  fi

  [ -z "$(git -C "$REPO_DIR" status --short --untracked-files=all)" ] ||
    die "working tree is dirty; commit or set VELIX_RELEASE_ALLOW_DIRTY=1 for local validation"
}

check_version_output() {
  local output expected_prefix
  expected_prefix="velix ${VERSION}"
  output="$("$REPO_DIR/target/release/vlx" --version)"
  case "$output" in
    "$expected_prefix" | "$expected_prefix ("*) ;;
    *) die "vlx --version output '$output' does not match '$expected_prefix'" ;;
  esac
  echo "==> Version: $output"
}

ensure_release_binary() {
  echo "==> Building release binary..."
  cargo build --release -p helix-term --manifest-path "$REPO_DIR/Cargo.toml"
  check_version_output
}

ensure_grammars() {
  if find "$REPO_DIR/runtime/grammars" -maxdepth 1 -type f -name "$(grammar_glob)" -print -quit |
    grep -q .; then
    echo "==> Runtime grammars already built."
    return
  fi

  echo "==> Fetching runtime grammar sources..."
  "$REPO_DIR/target/release/vlx" --grammar fetch
  echo "==> Building runtime grammars..."
  "$REPO_DIR/target/release/vlx" --grammar build
}

create_package() {
  echo "==> Creating package tree..."
  rm -rf "$DIST_DIR/package"
  mkdir -p "$PACKAGE_ROOT"

  rsync -a "$REPO_DIR"/ "$PACKAGE_ROOT"/ \
    --exclude ".git/" \
    --exclude ".direnv/" \
    --exclude "target/" \
    --exclude "dist/" \
    --exclude "logs/" \
    --exclude "vendor/" \
    --exclude ".DS_Store" \
    --exclude "runtime/grammars/sources/"

  find "$PACKAGE_ROOT/runtime/grammars" -maxdepth 1 -type f -name "$(grammar_glob)" -print -quit |
    grep -q . || die "package does not include built runtime grammars"

  echo "==> Creating tarball..."
  mkdir -p "$DIST_DIR"
  rm -f "$TARBALL"
  tar -C "$DIST_DIR/package" -czf "$TARBALL" "$PACKAGE_ROOT_NAME"
}

write_formula() {
  local sha url
  sha="$(shasum -a 256 "$TARBALL" | awk '{print $1}')"
  url="file://$TARBALL"

  echo "==> Writing local Homebrew formula..."
  mkdir -p "$FORMULA_DIR"
  if [ ! -d "$HOMEBREW_TAP_REPO/.git" ]; then
    git -C "$HOMEBREW_TAP_REPO" init
  fi

  cat >"$FORMULA_FILE" <<EOF
class Velix < Formula
  desc "Helix-based modal editor with Vim-style keybindings"
  homepage "https://github.com/astrohackerlabs/velix"
  url "${url}"
  sha256 "${sha}"
  license "MPL-2.0"

  depends_on "rust" => :build

  def install
    ENV["HELIX_DEFAULT_RUNTIME"] = libexec/"runtime"
    system "cargo", "install", *std_cargo_args(path: "helix-term")
    libexec.install "runtime"

    bash_completion.install "contrib/completion/vlx.bash" => "vlx"
    fish_completion.install "contrib/completion/vlx.fish"
    zsh_completion.install "contrib/completion/vlx.zsh" => "_vlx"
  end

  test do
    assert_match "velix #{version}", shell_output("#{bin}/vlx --version")
    assert_match "Velix editor", shell_output("#{bin}/vlx --help")
    assert_match "rust", shell_output("#{bin}/vlx --health rust")
  end
end
EOF

  echo "==> Package: ${TARBALL#$REPO_DIR/}"
  echo "==> SHA256: $sha"
  echo "==> Formula: $FORMULA_FILE"
}

main() {
  cd "$REPO_DIR"
  [ "${VELIX_RELEASE_PUBLISH:-0}" != "1" ] ||
    die "public publishing is deferred to a separate reviewed experiment"

  check_version_metadata
  check_worktree
  ensure_release_binary
  ensure_grammars
  create_package
  write_formula
}

main "$@"
