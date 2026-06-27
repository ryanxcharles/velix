# Vim Profile

Velix includes an opt-in Vim-oriented keymap profile. It keeps the default Helix
keymap unchanged unless you enable it in `config.toml`:

```toml
[editor]
keymap = "vim"
```

Use `keymap = "default"` or remove the setting to use the Helix-style keymap.
User `[keys]` remaps still apply on top of the selected profile.

## Vim-like Bindings

The first profile focuses on standalone normal-mode keys whose Helix command is
close enough for everyday Vim muscle memory:

| Key       | Action                                      |
| --------- | ------------------------------------------- |
| `h/j/k/l` | Move left/down/up/right                     |
| `w/b/e`   | Move by word                                |
| `0`       | Go to line start                            |
| `^`       | Go to first non-whitespace character        |
| `$`       | Go to line end                              |
| `gg`      | Go to file start                            |
| `G`       | Go to counted line, or file end without one |
| `i/a`     | Insert before/append after the selection    |
| `I/A`     | Insert at line start/end                    |
| `o/O`     | Open a line below/above                     |
| `/ ?`     | Search forward/backward                     |
| `n/N`     | Repeat search forward/backward              |
| `u`       | Undo                                        |
| `C-r`     | Redo                                        |
| `p/P`     | Paste after/before                          |
| `dd`      | Select the current line and delete it       |
| `yy`      | Select the current line and yank it         |
| `cc`      | Select the current line, change it, insert  |

`dd`, `yy`, and `cc` are intentionally implemented as a small linewise slice,
not as full Vim operator-pending mode.

## LazyVim-like Bindings

For workflows that are not core Vim editing grammar, the profile adds aliases
for common LazyVim-style commands when Velix already has a direct command:

| Key               | Action                           |
| ----------------- | -------------------------------- |
| `Space Space`     | File picker                      |
| `Space /`         | Global search                    |
| `Space f f`       | File picker                      |
| `Space f F`       | File picker in current directory |
| `Space f g`       | Changed-file picker              |
| `H/L`             | Previous/next buffer             |
| `[b` / `]b`       | Previous/next buffer             |
| `Space ,`         | Buffer picker                    |
| `Space b b`       | Buffer picker                    |
| `Space b p/n`     | Previous/next buffer             |
| `[d` / `]d`       | Previous/next diagnostic         |
| `Space x x`       | Diagnostics picker               |
| `Space x X`       | Workspace diagnostics picker     |
| `gd` / `gr`       | Go to definition/reference       |
| `Space a`         | Code action                      |
| `Space c a`       | Code action                      |
| `Space s s`       | Document symbols                 |
| `Space s S`       | Workspace symbols                |
| `[h` / `]h`       | Previous/next change             |
| `Space g g`       | Changed-file picker              |
| `C-h/j/k/l`       | Move between windows             |
| `Space -`         | Horizontal split                 |
| `Space \|`        | Vertical split                   |
| `Space w d`       | Close window                     |
| `Space w s/v`     | Horizontal/vertical split        |
| `Space w h/j/k/l` | Move between windows             |

## Different

The Vim profile still uses Helix's selection-first editing model. That means
some familiar keys are backed by Helix selections rather than Vim's internal
operator and motion types.

Important first-slice differences:

- `j` and `k` keep Helix visual-line movement, matching LazyVim's no-count
  display-line behavior more closely than raw Vim line movement.
- `dd`, `yy`, and `cc` use linewise selections. After `yy` followed by `p`, the
  pasted line may remain selected according to Helix selection behavior.
- Paste uses Helix register semantics. Exact Vim linewise versus characterwise
  paste fidelity is deferred.
- `[g` and `]g` keep Helix/Velix change navigation. LazyVim's comparable
  Gitsigns hunk navigation uses `[h` and `]h`; Velix maps those aliases to the
  same change navigation because it has no Gitsigns hunk command.
- Multiple selections are preserved where Helix commands naturally support them.

## Deferred

The profile does not yet implement full Vim input grammar. These areas need
explicit parser/state work before Velix can claim closer Vim compatibility:

- operator-pending commands such as `dw`, `d$`, `cw`, and `yap`;
- count multiplication such as `2d3w`;
- text-object operator forms such as `diw`, `ci"`, and `ya)`;
- register-prefix grammar such as `"ayy` and `"_dd`;
- dot-repeat with `.`;
- exact linewise/characterwise register metadata and paste behavior;
- blockwise Visual mode.
