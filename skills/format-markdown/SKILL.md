---
name: format-markdown
description:
  "Format markdown files with prettier. Use after creating or editing any
  markdown file (.md)."
---

# Format Markdown

Format markdown files with `prettier` when it is available.

## When This Skill Applies

After every write or edit to any `.md` file in the project. This includes:

- Issue documents under `issues/`
- Documentation under `docs/` and `book/`
- `README.md` files
- `AGENTS.md` and `CLAUDE.md`
- Skill files

## Process

After markdown edits are complete, run:

```bash
prettier --write --prose-wrap always --print-width 80 <file_path>
```

Use the `prettier` CLI directly when it is installed. If `prettier` is not
available, leave the file manually wrapped to roughly 80 columns and report that
formatting could not be run.
