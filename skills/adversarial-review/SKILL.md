---
name: adversarial-review
description:
  "Run an in-session adversarial review of Velix work using a fresh-context
  subagent. Use at experiment design/result gates, or whenever the user asks for
  adversarial / skeptical / red-team review."
---

# Adversarial Review

Run a fresh-context, read-only adversarial review inside the current agent
session by delegating to an adversarial subagent. No external reviewer CLI, no
session id, no logs to manage: spawn a subagent and use the returned verdict and
findings.

Runtime-specific invocation:

- **Codex:** use `multi_agent_v1.spawn_agent` when available. Pass the
  adversarial reviewer instructions in the spawn prompt, plus concrete artifact
  paths.
- **Claude:** use the `Agent` tool with `subagent_type: "adversarial-reviewer"`,
  defined in `.claude/agents/adversarial-reviewer.md`.

## When This Skill Applies

- The user asks for an "adversarial review", "skeptical review", "red team",
  "try to break this", or similar.
- An experiment reaches its **design gate** or **result gate** in the
  `AGENTS.md` experiment flow.
- A change is large, risky, or touches key dispatch, modal editing state,
  command parsing, selection behavior, text object semantics, undo/redo,
  persistence, or `unsafe` Rust.
- Before closing an issue after a complex series of experiments.

## Reviewer Posture

The subagent runs in its own fresh context. It receives only what you put in the
spawn prompt plus whatever it reads itself with available tools. Instruct it to
try to reject the work on evidence, verify claimed gate results independently
where feasible, and return a structured verdict.

Use this reviewer mandate in spawn prompts, or rely on the Claude agent file:

```text
You are the adversarial reviewer for Velix. You are separate from whoever
produced the work under review. Your default posture is skepticism. Try to
reject the work, but every objection must be grounded in evidence you can point
to.

Read-only discipline: do not edit, write, create, move, delete, stage, commit,
push, or run mutating commands. Use shell commands only for inspection and
read-only verification such as git diff/log/show/status, rg, cargo test, cargo
build, and cargo fmt --check. If a check would modify files, do not run it;
state that you could not verify it.

Return:
VERDICT: APPROVED | CHANGES REQUIRED
Then findings, most severe first:
[Required] file:line - issue. Evidence: ... Fix: ...
[Optional] file:line - issue. Evidence: ... Fix: ...
[Nit] file:line - issue. Fix: ...

Approve only when zero Required findings remain. Do not invent findings.
```

## What to Give the Reviewer

Because it starts blind, hand it the artifacts:

- the experiment file: `issues/<n>/NN-*.md`;
- the issue README: `issues/<n>/README.md`;
- the workflow contract: `AGENTS.md`;
- the relevant diff: exact `git diff`, `git diff --staged`, or `git show`
  command;
- the source files it should scrutinize;
- any upstream Helix or Vim reference being compared;
- command output whose truth matters.

## Design-Gate Prompt Template

```text
Review this Velix experiment DESIGN with fresh context. Do not edit anything.

Read:
- the experiment file: issues/<n>/NN-<slug>.md
- the issue README: issues/<n>/README.md
- the workflow contract: AGENTS.md
- relevant source/reference files: <paths>

Try to reject this design. Check:
- the issue README links this experiment with status Designed;
- the experiment has Description, Changes, and Verification;
- scope is narrow enough for one experiment and matches exactly what was asked;
- the technical plan is correct for Helix/Velix architecture;
- verification has concrete pass/fail criteria that would prove the goal;
- required hygiene checks are present: cargo fmt, targeted tests, and relevant
  manual/editor checks.

Return VERDICT (APPROVED | CHANGES REQUIRED) then findings with file:line,
evidence, and a concrete fix. Approve only if no Required findings remain.
```

## Result-Gate Prompt Template

```text
Review this COMPLETED Velix experiment with fresh context. Do not edit anything.

Read:
- the experiment file: issues/<n>/NN-<slug>.md
- the issue README: issues/<n>/README.md
- the implementation diff: run `git diff <plan-commit>..HEAD -- <paths>` or
  inspect the working tree if not yet committed
- the changed source and relevant tests
- the workflow contract: AGENTS.md

Try to reject this result. Check:
- the implementation matches the approved scope;
- it is correct for modal editing behavior and does not regress Helix behavior
  outside the intended compatibility surface;
- tests and manual checks actually prove the claim;
- independently verify claimed gate results where feasible;
- the experiment file has Result and Conclusion, and the README status matches;
- the result commit has not been made before this review.

Return VERDICT then findings with file:line, evidence, and a concrete fix.
Approve only if no Required findings remain.
```

## After the Review

The implementing agent remains responsible for judgment.

1. Accept findings that are real correctness, verification, scope, or workflow
   issues.
2. Reject false positives explicitly, with a concise reason.
3. Re-review after non-trivial fixes until no Required findings remain.
4. Record the review in the experiment file: reviewer type, findings, fixes, and
   final verdict.
5. Respect the commit gates.
6. Give the adversarial reviewer up to 15 minutes to finish unless the user
   explicitly changes direction.
