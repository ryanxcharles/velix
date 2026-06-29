+++
status = "open"
opened = "2026-06-29"
workflow = "issues-and-experiments"
review_mode = "same-agent"
+++

# Issue 9: Import TermSurf Workflow

## Goal

Update Velix's AI workflow files to match the current TermSurf issue,
experiment, skill, and epic workflow while preserving Velix-specific project
guidance and historical issue records.

## Background

Velix currently keeps the detailed issue and experiment workflow in `AGENTS.md`.
TermSurf has moved that detailed process into shared workflow skills, added an
`epics/` planning layer, replaced whole-directory agent skill symlinks with
per-skill links, and added an epic index generator.

Velix already has `CLAUDE.md` symlinked to `AGENTS.md`, but `.codex/skills` and
`.claude/skills` are whole-directory symlinks to `skills`. Velix also lacks the
TermSurf workflow skills for epics, manual issue workflows, orthogonal review,
and external Codex/Claude review helpers.

## Analysis

The import should be adapted, not copied blindly. Product-specific TermSurf
skills for browser engines, Ghostty, Zig, and Homebrew release behavior should
not become Velix defaults unless they are rewritten for Velix.

The migration should:

- keep Velix's project summary, Rust formatting rule, and Helix/Vim research
  guidance;
- add epics as a planning layer above issues;
- move detailed issue and experiment mechanics from `AGENTS.md` into skills;
- adopt future experiment filenames of `exp-NNNN-{slug}.md`;
- preserve historical issue and experiment filenames without rewriting closed
  records;
- add workflow metadata for future new issues;
- migrate `.codex/skills/` and `.claude/skills/` to real directories with
  individual symlinks;
- import or adapt only workflow-relevant skills;
- add `scripts/build-epics-index.sh`;
- update `scripts/build-issues-index.sh` with TermSurf's safer reserved-key
  parsing behavior.

## Experiments

- [Experiment 1: Import TermSurf workflow files](01-import-termsurf-workflow-files.md) -
  **Pass**
