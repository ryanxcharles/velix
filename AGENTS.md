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

## Issues and Experiments

Every significant piece of work gets an issue in `issues/`. Issues describe the
problem, provide background, and propose solutions. Experiments are the
incremental steps that solve the problem.

### Issue Structure

Each issue is a **folder**. The `README.md` is the issue **spine** (frontmatter,
goal, background, analysis, an ordered index of experiments, and the final
conclusion). **Every experiment is its own numbered file** in the same folder;
the README never contains experiment bodies, only links to them.

```text
issues/0001-vim-normal-mode-baseline/
+-- README.md                     <- spine: frontmatter, goal, background,
|                                    the ordered Experiments index, conclusion
+-- 01-map-basic-motions.md       <- Experiment 1
+-- 02-wire-operator-pending.md   <- Experiment 2
+-- 03-...                        <- one file per experiment, in sequence
```

The folder name is `{NNNN}-{slug}`. The number is zero-padded to 4 digits and
globally sequential. The slug is lowercase, hyphenated, and describes the topic.

The full index of all issues is at `issues/README.md`. Regenerate it with:

```bash
scripts/build-issues-index.sh
```

#### Frontmatter

Every issue `README.md` starts with TOML frontmatter:

```toml
+++
status = "open"
opened = "2026-06-27"
+++
```

Or for closed issues:

```toml
+++
status = "closed"
opened = "2026-06-27"
closed = "2026-06-27"
+++
```

Issues may add their own TOML frontmatter keys to `README.md`, experiment files,
or other issue docs for issue-specific metadata, as long as:

- the reserved workflow keys are preserved: `README.md` always carries `status`
  and `opened`, plus `closed` when closed;
- additive keys are valid TOML between the `+++` delimiters and do not
  contradict the reserved keys or the index tooling;
- the issue documents its own added schema in its `README.md`.

#### README.md Structure

After the frontmatter, a new issue's `README.md` has these sections:

1. **Title** (H1): `# Issue {N}: {descriptive title}`
2. **Goal**: one or two sentences describing the desired outcome.
3. **Background**: context, prior work, constraints.
4. **Architecture** / **Analysis** / **Proposed Solutions**: technical details.

A new issue's README has no experiments listed yet.

As experiments are created, the README grows an **`## Experiments`** section: an
ordered list linking to each experiment file, one per line, with a one-line
status. The README holds the links and statuses only; never put experiment
bodies in the README. Example:

```markdown
## Experiments

- [Experiment 1: Map basic Vim motions](01-map-basic-motions.md) - **Pass**
- [Experiment 2: Wire operator-pending mode](02-wire-operator-pending.md) -
  **Partial** (needs count handling)
- [Experiment 3: Add text objects](03-text-objects.md) - **Designed**
```

Keep each status to one of: `Designed`, `In progress`, `Pass`, `Partial`,
`Fail`. Update the line when the experiment's result is recorded, so the README
doubles as an at-a-glance progress tracker.

When the issue is solved or abandoned, add the **`## Conclusion`** section to
the README.

#### Experiment Files

Each experiment lives in its own file `NN-{slug}.md` in the issue folder, where
`NN` is a zero-padded two-digit number in creation order. The slug is lowercase,
hyphenated, and describes the experiment.

An experiment file may begin with optional TOML frontmatter (`+++ ... +++`)
before its H1 title for issue-specific metadata such as agent provenance.
Experiment frontmatter is optional and must not replace the required H1 title
and H2 sections.

Each experiment file contains:

1. **Title** (H1): `# Experiment {N}: {descriptive title}`
2. **Description**: what and why.
3. **Changes**: specific code changes, listed by file.
4. **Verification**: how to test, with concrete steps and pass/fail criteria.
5. **Result** and **Conclusion**: added after the experiment runs.

Keep each file focused. If one grows past about 1000 lines, split the work into
the next numbered experiment.

### Multiple Open Issues

Multiple issues can be open at the same time. This allows interleaving work: a
large Vim-compatibility issue can stay open while smaller issues are opened and
closed alongside it.

### Experiments

#### When to Create an Experiment

Only after the issue's requirements are clear. Each experiment is designed,
implemented, and concluded before the next one is designed.

**Never list experiments upfront.** The outcome of each experiment informs what
comes next.

#### One at a Time

Design and implement one experiment at a time. The result of Experiment 1
directly informs what Experiment 2 should be.

#### AI Review Gate

Every experiment must be reviewed by another AI agent before moving to the next
stage.

1. **Design review before implementation**
   - After writing the experiment design, ask another AI agent to review it.
   - Fix all real issues found by the review.
   - Record the review result in the experiment file.
   - Do not implement the experiment until the reviewing agent approves the
     design.

2. **Result review before the next experiment**
   - After implementation, verification, and result recording, ask another AI
     agent to review the completed experiment and result.
   - Fix all real issues found by the review.
   - Record the completion-review result in the experiment file.
   - Do not design or implement the next experiment until the reviewing agent
     approves the completed output.

The reviewing agent may be Codex, Claude, or another explicitly requested agent,
but it must be separate from the implementation pass.

Adversarial reviewers are allowed up to **15 minutes** to complete a review.
After spawning a reviewer, do not interrupt it, demand a bounded verdict, close
it, or proceed around it before that time has elapsed unless the user explicitly
asks you to stop or change direction. If the reviewer finishes earlier, use its
verdict normally.

#### Experiment Commits

Every experiment has two required commit points:

1. **Plan commit**: after the experiment design is written, reviewed, fixed,
   approved, and linked from the issue README, commit the experiment plan before
   implementation begins.
2. **Result commit**: after implementation, verification, result recording,
   completion review, and any required fixes, commit the experiment result
   before designing the next experiment.

These commits must be separate. Do not combine an experiment plan and its result
in the same commit, and do not start the next experiment before the previous
experiment's result commit exists.

#### Recording Results

After testing, append the result inside the experiment's own file, below
Verification:

```markdown
## Result

**Result:** Pass / Partial / Fail

{description}

## Conclusion

{what we learned, what the next experiment should be}
```

Then update that experiment's status on its line in the README's
`## Experiments` index. All three outcomes are valuable; failed experiments
eliminate dead ends.

### Closing an Issue

When the issue is solved or abandoned, add a `## Conclusion` section to the
issue `README.md`, after the `## Experiments` index. Summarize what was learned
and the outcome. Update the frontmatter to `status = "closed"` with a `closed`
date. Regenerate the index:

```bash
scripts/build-issues-index.sh
```

### Immutability

Closed issues are historical records. They are immutable and must never be
modified. History stays as it was written.

### Process Summary

1. **Create the issue**: `issues/{NNNN}-{slug}/README.md` with frontmatter,
   goal, background, analysis. No experiments yet.
2. **Design Experiment 1**: create `01-{slug}.md` with the experiment body, and
   add a link to it under `## Experiments` in the README with status `Designed`.
3. **Review and commit the plan**: get another AI agent to approve the design,
   fix real findings, record the review result, and commit the experiment plan.
4. **Implement Experiment 1**: write the code.
5. **Record the result**: append `## Result` and `## Conclusion` inside
   `01-{slug}.md`, and update its status on the README index line.
6. **Review and commit the result**: get another AI agent to approve the
   completed output, fix real findings, record the completion review, and commit
   the experiment result.
7. **Repeat**: create `02-{slug}.md` for the next experiment, link it from the
   README, and continue until the goal is met.
8. **Close the issue**: write the `## Conclusion` in the README, update
   frontmatter, rebuild the index.

## Remember

Never change code unless explicitly asked. Never make unrequested changes. Do
exactly what the user asks.
