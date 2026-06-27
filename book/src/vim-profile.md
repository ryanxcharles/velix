# Vim Keymap

Velix uses its Vim-oriented keymap profile by default. A fresh Velix config uses
Vim-style normal-mode keys such as `G` for file end without requiring an
explicit keymap setting.

To use the Helix-style keymap instead, set this in `config.toml`:

```toml
[editor]
keymap = "default"
```

You can also set `keymap = "vim"` explicitly. User `[keys]` remaps still apply
on top of the selected profile.

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
| `q{reg}`  | Record macro into register                  |
| `@{reg}`  | Replay macro from register                  |
| `.`       | Repeat the last insert change               |
| `p/P`     | Paste after/before                          |
| `dd`      | Select the current line and delete it       |
| `yy`      | Select the current line and yank it         |
| `cc`      | Select the current line, change it, insert  |

`dd`, `yy`, and `cc` are intentionally implemented as a small linewise slice,
not as full Vim operator-pending mode.

## Select-mode Bindings

Velix maps common Vim visual-mode motions onto Helix select mode where the
selection behavior is close enough:

| Key | Action                                          |
| --- | ----------------------------------------------- |
| `v` | Enter Helix select mode                         |
| `V` | Select the current line and enter select mode   |
| `$` | Extend the selection to line end                |
| `0` | Extend the selection to line start              |
| `^` | Extend the selection to first non-whitespace    |
| `G` | Extend to counted line, or file end without one |

`V` is a line-selection entry point into Helix select mode. It is not yet full
Vim linewise Visual mode with linewise register and paste metadata.

## LazyVim-like Bindings

For workflows that are not core Vim editing grammar, the profile adds aliases
for common LazyVim-style commands when Velix already has a direct command:

| Key               | Action                              |
| ----------------- | ----------------------------------- |
| `Space Space`     | File picker                         |
| `Space /`         | Global search                       |
| `Space f f`       | File picker                         |
| `Space f F`       | File picker in current directory    |
| `Space f g`       | Changed-file picker                 |
| `H/L`             | Previous/next buffer                |
| `[b` / `]b`       | Previous/next buffer                |
| `Space ,`         | Buffer picker                       |
| `Space b b`       | Buffer picker                       |
| `Space b p/n`     | Previous/next buffer                |
| `[d` / `]d`       | Previous/next diagnostic            |
| `Space x x`       | Diagnostics picker                  |
| `Space x X`       | Workspace diagnostics picker        |
| `gd` / `gr`       | Go to definition/reference          |
| `gD` / `gy`       | Go to declaration/type definition   |
| `gI` / `gK`       | Go to implementation/signature help |
| `Space a`         | Code action                         |
| `Space c a`       | Code action                         |
| `Space c r`       | Rename symbol                       |
| `Space s s`       | Document symbols                    |
| `Space s S`       | Workspace symbols                   |
| `[h` / `]h`       | Previous/next change                |
| `Space g g`       | Changed-file picker                 |
| `C-h/j/k/l`       | Move between windows                |
| `Space -`         | Horizontal split                    |
| `Space \|`        | Vertical split                      |
| `Space w d`       | Close window                        |
| `Space w s/v`     | Horizontal/vertical split           |
| `Space w h/j/k/l` | Move between windows                |

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
- `.` uses Velix's existing repeat-last-insert behavior. Full Vim dot-repeat for
  arbitrary normal-mode changes and operator forms is deferred.
- `Q` remains Helix's direct macro-record command as a fallback. Vim-style macro
  recording should use `q{reg}`.
- `[g` and `]g` keep Helix/Velix change navigation. LazyVim's comparable
  Gitsigns hunk navigation uses `[h` and `]h`; Velix maps those aliases to the
  same change navigation because it has no Gitsigns hunk command.
- `ma` and `mi` keep Helix text-object selection under the match menu. Vim
  operator text-object forms such as `diw`, `ci"`, and `ya)` are not implemented
  as Vim grammar.
- `v` enters Helix select mode. `V` selects the current line in Helix select
  mode, but it does not create full Vim linewise Visual metadata. `C-v` does not
  start Vim blockwise Visual mode.
- `C-s` keeps the Helix jumplist behavior, not LazyVim's save-file mapping. Use
  `:write` / `:w` to save, or add a user remap.
- `K` keeps Helix `keep_selections`, not LazyVim hover.
- `[e` and `]e` keep Helix tree-sitter entry navigation, not LazyVim
  severity-specific diagnostic navigation.
- Multiple selections are preserved where Helix commands naturally support them.

## Deferred

The profile does not yet implement full Vim input grammar. These areas need
explicit parser/state work before Velix can claim closer Vim compatibility:

- operator-pending commands such as `dw`, `d$`, `cw`, and `yap`;
- count multiplication such as `2d3w`;
- text-object operator forms such as `diw`, `ci"`, and `ya)`;
- register-prefix grammar such as `"ayy` and `"_dd`;
- Vim named marks and mark jumps such as `ma`, `'a`, and `` `a ``; use the Helix
  jumplist with `C-s`, `C-o`, `C-i`, and `Space j`;
- full Vim dot-repeat with `.` beyond the current repeat-last-insert behavior;
- exact linewise/characterwise register metadata and paste behavior, including
  full Vim linewise Visual mode metadata after `V`;
- blockwise Visual mode;
- LazyVim persistence/session mappings such as `Space q s`, `Space q S`,
  `Space q l`, and `Space q d`;
- LazyVim terminal mappings such as `Space f t`, `Space f T`, and `C-/`;
- LazyVim tab mappings under `Space Tab`; use Velix buffers and windows instead;
- LazyVim UI toggles under `Space u`;
- LazyVim `Space c f` format-current-buffer mapping; Velix's existing
  `format_selections` command is range/selection formatting, not the same
  behavior.
