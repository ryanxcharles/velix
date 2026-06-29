# Velix

Velix is a fork of Helix, a Rust-based modal editor. The goal of this repository
is to investigate what it would take to keep Helix's architecture while offering
Vim-style keybindings. Full Vim compatibility is not required unless a specific
issue says otherwise; favor pragmatic compatibility backed by small experiments.

[Agent development guide](https://agents.md/).

## Rules

Do exactly what the user asks. No more, no less. Never assume they want
something they did not ask for. Never change code unless explicitly asked.

When editing Rust code, always run `cargo fmt`. Accept the formatter output as
the source of truth. Do not manually undo, minimize, or selectively revert
`cargo fmt` formatting changes, including import ordering or wrapping changes.

When researching Helix or Vim behavior, prefer local source, built-in help, and
small executable experiments over memory. If a claim depends on current upstream
Helix behavior, verify it against the local checkout or the specific upstream
revision being compared.

## Project Notes

- `helix-term/` contains much of the terminal UI and command/key handling.
- `helix-view/` contains editor state and view behavior.
- `helix-core/` contains text, selection, movement, syntax, and lower-level
  editing primitives.
- `runtime/` contains default configuration, themes, and runtime assets.
- `book/` and `docs/` contain user-facing documentation.

Keep fork changes easy to inspect. Prefer narrowly scoped changes, and document
behavioral tradeoffs in issues before larger keybinding migrations.

## Epics

Epics live in `epics/`. They describe coherent goals that are larger than one
issue and track the checklist of work that should become issues or experiments.

Use epics for product or workflow directions that need multiple issues to
complete. Do not use an epic as a substitute for an issue: implementation work
still happens through issues and experiments.

See `epics/README.md` for the epic schema, index, and template. Regenerate the
epic index after creating or closing epics with:

```bash
scripts/build-epics-index.sh
```

## Issues and Experiments

Every significant concrete work item gets an issue in `issues/`. Issues describe
the problem, background, constraints, and proposed direction. Experiments are
the incremental steps that solve the issue.

The full issue index is at `issues/README.md`. Regenerate it with:

```bash
scripts/build-issues-index.sh
```

### Routing Contract

`AGENTS.md` intentionally gives only the routing-level workflow rules. Detailed
procedures live in workflow skills:

- `epics` for epic creation, updates, and closure;
- `issues-and-experiments` for the default automated experiment workflow;
- `manual-issues-and-experiments` for the manual workflow variant;
- `adversarial-review` for same-agent in-session review when explicitly chosen;
- `claude-review` and `codex-review` for explicit external review modes;
- `orthogonal-review` for cross-harness review routing.

Project skills live under `skills/`. Agent harnesses expose skills through
`.codex/skills/` and `.claude/skills/`, which are real directories containing
individual symlinks, not whole-directory symlinks. Shared skills may link to the
same `skills/<name>/` implementation. Harness-specific skills may use the same
public skill name with different targets, but the difference must be documented
in the issue or docs that introduce it.

When adding or auditing skill links, compare each agent directory against the
shared skill list and check for broken symlinks. If a link intentionally
diverges for one harness, document the exception near the change.

Non-negotiable rules:

- create or update an issue before significant work;
- every new issue README frontmatter must specify both the solution workflow
  (`workflow = "issues-and-experiments"` or
  `workflow = "manual-issues-and-experiments"`) and the reviewer
  (`review_mode = "same-agent"`, `review_mode = "external-claude"`, or
  `review_mode = "external-codex"`). New automated issues default to orthogonal
  review: Codex-authored issues use `review_mode = "external-claude"` and
  Claude-authored issues use `review_mode = "external-codex"`, with
  `review_routing = "orthogonal-review"`;
- design and conclude one experiment at a time;
- never list future experiments upfront;
- get a separate AI review before implementation and before the result commit;
- use the issue README's `review_mode` for design and completion reviews unless
  a specific experiment records a deliberate deviation;
- record the review mode, reviewer harness or command, verdict, required
  findings, and resolutions in `## Design Review` and `## Completion Review`;
- commit the reviewed plan before implementation;
- commit the reviewed result before designing the next experiment;
- record results in the experiment file and update the issue README experiment
  status;
- close issues by adding a `## Conclusion`, setting `status = "closed"` and
  `closed = "YYYY-MM-DD"` in frontmatter, and rebuilding `issues/README.md`.

### Issue Shape

Each new issue is a folder named `issues/{NNNN}-{slug}/`, where `NNNN` is the
next zero-padded issue number and the slug is lowercase hyphenated. The issue
spine is `README.md` with TOML frontmatter:

```toml
+++
status = "open"
opened = "YYYY-MM-DD"
workflow = "issues-and-experiments"
review_mode = "external-claude"
review_routing = "orthogonal-review"
+++
```

Use `workflow = "issues-and-experiments"` for the fully automated workflow and
`workflow = "manual-issues-and-experiments"` for the manual workflow. Use
orthogonal review by default for automated issues:

```toml
review_mode = "external-claude" # Codex-authored issue
review_routing = "orthogonal-review"
```

Claude-authored issues use `review_mode = "external-codex"` with the same
`review_routing`. Use `review_mode = "same-agent"` only when the user explicitly
requests same-harness in-session review or no external reviewer is available.
Closed issue frontmatter also includes `closed = "YYYY-MM-DD"`.

New issue READMEs start with a title, goal, background, and analysis or proposed
solution. A new issue does not list experiments until the first experiment is
designed.

### Future Experiment Shape

For experiments created from now on, use:

```text
exp-NNNN-{descriptive-name}.md
```

`NNNN` is zero-padded in creation order within the issue, and
`{descriptive-name}` is lowercase hyphenated. Link each experiment from the
issue README under `## Experiments` with one of these statuses: `Designed`,
`In progress`, `Pass`, `Partial`, or `Fail`.

Each experiment file contains:

1. `# Experiment {N}: {descriptive title}`
2. `## Description`
3. `## Changes`
4. `## Verification`
5. `## Design Review`, when the design review is recorded
6. `## Result` and `## Conclusion`, after implementation
7. `## Completion Review`, when the result review is recorded

### Historical Immutability

Closed issues are historical records. They are immutable and must not be
modified unless the user explicitly requests a specific historical edit.

Do not migrate, rename, rewrite, normalize, or retrofit historical issue or
experiment files to the future experiment naming convention. Older issues keep
their original shapes and filenames.

## Remember

Never change code unless explicitly asked. Never make unrequested changes. Do
exactly what the user asks.
