#!/usr/bin/env bash
# Deploy rules of conduct:
# - Read AGENTS.md and canonical Notes/docs before changing or deploying.
# - Never print, commit, or copy secrets; use passvault/env files only.
# - Preserve env files, service users, user data, and databases; snapshot state before risky swaps.
# - Verify ports, DNS, TLS, and systemd unit names before changing routes or services.
# - Use managed services only (systemd/pm2); no nohup, background shells, or ad hoc daemons.
# - Run gates and health checks; if deploy fails, stop and roll back rather than improvising.
# - Keep deploy behavior unchanged unless the task explicitly asks for deploy logic changes.
# deploy.sh — AkurAI-CRM release engine.
set -euo pipefail

REPO="olibuijr/akurai-crm"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

say() { printf '\033[1;34m▸ %s\033[0m\n' "$*"; }
die() { printf '\033[1;31m✗ %s\033[0m\n' "$*" >&2; exit 1; }

BUMP="${1:-patch}"
case "$BUMP" in patch|minor|major) ;; *)
  echo "usage: $0 [patch|minor|major]" >&2; exit 2
;; esac

# ── 1. Quality gate ────────────────────────────────────────────────────────
say "Gate: rustfmt"
cargo fmt --all -- --check || die "formatting failed (run: cargo fmt --all)"

say "Gate: clippy (-D warnings)"
cargo clippy --all-targets --all-features -- -D warnings || die "clippy failed"

say "Gate: tests"
cargo test --all || die "tests failed"

# ── 2. Version bump (lockstep) ─────────────────────────────────────────────
CUR="$(tr -d '[:space:]' < VERSION)"
IFS='.' read -r MA MI PA <<< "$CUR"
case "$BUMP" in
  major) MA=$((MA+1)); MI=0; PA=0 ;;
  minor) MI=$((MI+1)); PA=0 ;;
  patch) PA=$((PA+1)) ;;
esac
NEW="$MA.$MI.$PA"
say "Version: $CUR → $NEW ($BUMP)"
printf '%s\n' "$NEW" > VERSION
sed -i -E "s/^version = \"$CUR\"/version = \"$NEW\"/" Cargo.toml
if [ -f site/backend/page.json ]; then
  sed -i -E "s/\"version\": \"v[0-9]+\.[0-9]+\.[0-9]+\"/\"version\": \"v$NEW\"/" site/backend/page.json
fi

# ── 3. CHANGELOG ───────────────────────────────────────────────────────────
DATE="$(date +%Y-%m-%d)"
say "CHANGELOG: cutting [$NEW]"
if [ -f CHANGELOG.md ]; then
  awk -v unrel="## [Unreleased]" -v vh="## [$NEW] - $DATE" '
    $0 == unrel { print unrel; print ""; print vh; next }
    { print }
  ' CHANGELOG.md > CHANGELOG.md.tmp && mv CHANGELOG.md.tmp CHANGELOG.md
fi

# ── 4. Static musl build ────────────────────────────────────────────────────
say "Build: static musl binary"
rustup target add x86_64-unknown-linux-musl >/dev/null 2>&1 || true
cargo build --release --target x86_64-unknown-linux-musl -p akurai-crm \
  || die "musl build failed"

# ── 5. Commit, tag, push ───────────────────────────────────────────────────
say "Git: commit + tag v$NEW"
git add -A
git commit -q -m "release: v$NEW"
git tag -a "v$NEW" -m "v$NEW"

if ! git remote get-url origin >/dev/null 2>&1; then
  if gh repo view "$REPO" >/dev/null 2>&1; then
    git remote add origin "https://github.com/$REPO.git"
  else
    say "Creating GitHub repo $REPO (private)"
    gh repo create "$REPO" --private --source=. --remote=origin
  fi
fi

say "Git: push"
git push -u origin HEAD
git push origin "v$NEW"

say "Released v$NEW ✓"
