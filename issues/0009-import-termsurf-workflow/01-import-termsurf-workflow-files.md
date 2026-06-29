# Experiment 1: Import TermSurf Workflow Files

## Description

Import the reusable parts of TermSurf's workflow system into Velix and adapt the
wording, paths, and review guidance for a Helix fork focused on Vim-style
keybindings.

This experiment covers the workflow migration as one coherent change because the
files are tightly coupled: `AGENTS.md` routes to skills, agent skill directories
expose those skills, epics need an index generator, and review skills need
consistent workflow metadata.

Historical Velix issues and experiment filenames must not be migrated or
rewritten.

## Changes

- Update `AGENTS.md`:
  - keep Velix-specific project notes and rules;
  - add an `## Epics` section;
  - replace the long embedded issue workflow with TermSurf-style routing rules;
  - document future issue workflow metadata;
  - document future experiment filenames as `exp-NNNN-{slug}.md`;
  - document per-agent skill symlink strategy.
- Keep `CLAUDE.md` as a symlink to `AGENTS.md`.
- Add `epics/README.md` and `epics/0000-template/README.md`.
- Add `scripts/build-epics-index.sh`.
- Update `scripts/build-issues-index.sh` with the safer TermSurf reserved-key
  parsing behavior while preserving Velix's optional prettier fallback.
- Add or adapt workflow skills under `skills/`:
  - `issues-and-experiments`;
  - `manual-issues-and-experiments`;
  - `epics`;
  - `orthogonal-review`;
  - `claude-review`;
  - `codex-review`;
  - `create-skill`;
  - update `adversarial-review`.
- Keep Velix's existing `commit`, `format-markdown`, and `format-rust` skills.
- Replace `.codex/skills -> ../skills` and `.claude/skills -> ../skills` with
  real directories containing individual symlinks to `../../skills/<name>`.
- Update `.claude/agents/adversarial-reviewer.md` to match the stronger TermSurf
  reviewer contract, adapted to Velix.

## Verification

- `test -L CLAUDE.md && [ "$(readlink CLAUDE.md)" = "AGENTS.md" ]`
- `test -d .codex/skills && test ! -L .codex/skills`
- `test -d .claude/skills && test ! -L .claude/skills`
- Broken skill-link check prints nothing:

  ```bash
  find .codex/skills .claude/skills -maxdepth 1 -type l -print |
    while IFS= read -r path; do
      [ -e "$path" ] || printf '%s\n' "$path"
    done
  ```

- Expected skill-link lists match:

  ```bash
  find skills -mindepth 1 -maxdepth 1 -type d -exec basename {} \; | sort > /tmp/velix-shared-skills.txt
  find .codex/skills -mindepth 1 -maxdepth 1 -type l -exec basename {} \; | sort > /tmp/velix-codex-skills.txt
  find .claude/skills -mindepth 1 -maxdepth 1 -type l -exec basename {} \; | sort > /tmp/velix-claude-skills.txt
  diff -u /tmp/velix-shared-skills.txt /tmp/velix-codex-skills.txt
  diff -u /tmp/velix-shared-skills.txt /tmp/velix-claude-skills.txt
  ```

- `scripts/build-epics-index.sh`
- `scripts/build-issues-index.sh`
- `bash -n scripts/build-epics-index.sh scripts/build-issues-index.sh`
- `rg -n "Never change code unless explicitly asked|cargo fmt|Helix or Vim behavior|helix-term|helix-view|helix-core|runtime/|book/|docs/" AGENTS.md`
  confirms Velix-specific rules and project notes survived the rewrite.
- `git diff --name-status -- issues ':!issues/0009-import-termsurf-workflow' ':!issues/README.md'`
  prints nothing, proving historical issue records were not edited by this
  workflow migration.
- `find issues -mindepth 2 -maxdepth 2 -type f | sort` before and after the
  migration shows no historical experiment files were renamed or removed.
- `rg -n "TermSurf|termsurf|Ghostty|Chromium|WebKit|Roamium|Surfari|Zig|Nerd Font" AGENTS.md skills epics .claude/agents/adversarial-reviewer.md`
  reports no imported product-specific wording except in historical issue
  records.
- `prettier --write --prose-wrap always --print-width 80` on edited Markdown
  files when available.
- `git diff --check`

## Design Review

Codex adversarial reviewer, fresh context: **Changes required**.

Required findings and resolutions:

- The design said `AGENTS.md` must preserve Velix-specific project notes and
  rules, but verification did not prove those requirements survived the rewrite.
  Resolved by adding an `rg` check for the no-unrequested-code-change rule, Rust
  formatting rule, Helix/Vim research guidance, and key Velix project
  directories.
- The design required historical issues and experiment filenames not be migrated
  or rewritten, but verification did not check that historical issue records
  remained untouched. Resolved by adding a historical issue diff check and a
  before/after file-list check for historical experiment filenames.

Re-review: **Approved**. The reviewer confirmed the updated verification checks
preservation of Velix-specific `AGENTS.md` rules/project notes and adds
historical issue diff plus before/after filename validation for historical
experiment records.
