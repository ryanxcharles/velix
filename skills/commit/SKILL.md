---
name: commit
description: "Write entertaining commit messages as poetry"
---

# GitPoet

Write commit messages that accurately describe changes while delighting readers
with poetic wit.

## Philosophy

Every commit tells a story. GitPoet transforms mundane diffs into memorable
verses. The goal is to make people smile when they read the Velix commit log.

## Format

Each commit message has two parts:

1. **First line**: a short, accurate summary, 50 characters max.
2. **Body**: a short poem, 2-8 lines, that humorously describes the change.

## Style Guidelines

- **Accuracy first**: the poem must accurately describe what changed.
- **Humor over formality**: prefer wit, wordplay, puns, and light absurdity.
- **Keep it short**: poems should be 2-8 lines, not epics.
- **Vary the form**: mix haikus, limericks, couplets, free verse, etc.
- **Stay tasteful**: funny but professional enough for public viewing.

## Examples

### Haiku Style

```text
Fix null pointer crash

A pointer walked alone,
Into the void it did fall;
Now it checks its path.
```

### Limerick Style

```text
Add dark mode toggle

A user who coded at night,
Found the screen far too bright.
So we added a switch,
Now it's dark, what a pitch!
Their retinas now feel just right.
```

### Couplet Style

```text
Refactor motion parser

The cursor once took scenic tours,
Now counts and motions know their chores.
```

## Arguments

`/commit <what>` commits exactly `<what>`. Stage only the files related to
`<what>` and nothing else. Do not include unrelated changes. If no argument is
given, look at all uncommitted changes and commit everything.

## Process

1. **Check the argument.** If the user provided `<what>`, identify exactly which
   files belong to that scope. Stage only those files.
2. **Run `git diff --staged`** to see what is being committed.
3. **Understand the change**: what problem does it solve? What was added,
   removed, or fixed?
4. **Write the title**: accurate, imperative mood, 50 characters max.
5. **Compose the poem**: pick a style that fits the change.
6. **Stage and commit** using the poetic message.

## Commit Message Mechanics

Poem lines must be real newline characters in Git history. Never put literal
`\n` sequences inside a shell string and expect Git to interpret them.

For multi-line commit messages, prefer writing the message to a temporary file
and committing with `git commit -F <file>`:

```text
Title under 50 chars

Poem line one,
Poem line two.
```

Then run:

```bash
git commit -F /path/to/message-file
```

If using `git commit -m`, only use it for separate paragraphs where the shell
arguments already contain the exact intended text. Do not encode poem line
breaks as `\n` inside `-m` arguments.

After committing, verify the rendered message:

```bash
git log -1 --format=%B
```

If literal `\n` appears in the output, amend or rewrite the commit message
immediately before continuing.

## When Not to Use GitPoet

- Merge commits.
- Reverts.
- Version bumps.
- Security fixes.
- Imported upstream patch commits that should retain original authorship.
