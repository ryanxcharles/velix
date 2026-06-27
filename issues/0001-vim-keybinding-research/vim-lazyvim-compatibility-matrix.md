# Vim and LazyVim Compatibility Matrix

## Executive Recommendation

Velix should start with a named `vim` keymap profile plus a small explicit Vim
grammar state, not a wholesale replacement of Helix's current default keymap.

The evidence points to two separate compatibility surfaces:

- A large LazyVim-style workflow layer can be implemented as keymap/profile
  bindings to existing Helix commands and pickers.
- Core Vim editing cannot be represented faithfully by remapping alone because
  Vim has operator-pending mode, count multiplication, motion type
  characterwise/linewise/inclusive/exclusive semantics, and dot-repeat.

The first code experiment should therefore be a small vertical slice:

1. Add a named `vim` keymap profile selected by configuration.
2. Bind low-risk normal-mode and LazyVim workflow keys through that profile.
3. Add only the smallest grammar surface needed for linewise repeated operators:
   `dd`, `yy`, and `cc`.

Defer full `d{motion}`, `c{motion}`, `y{motion}`, text-object operators,
register prefixes, and dot-repeat until the profile mechanism and first operator
tests are proven.

## Helix Keymap Architecture Summary

Helix default keymaps are centralized in `helix-term/src/keymap/default.rs`. The
file builds one `HashMap<Mode, KeyTrie>` containing `Mode::Normal`,
`Mode::Select`, and `Mode::Insert` entries
(`helix-term/src/keymap/default.rs:7`, `helix-term/src/keymap/default.rs:407`).

Key bindings map to command leaves, command sequences, or nested trie nodes.
`KeyTrie` has variants for `MappableCommand`, `Sequence`, and `Node`
(`helix-term/src/keymap.rs:109`). TOML deserialization accepts a string command,
a command list, or a nested map (`helix-term/src/keymap.rs:134`,
`helix-term/src/keymap.rs:144`, `helix-term/src/keymap.rs:172`).

User keymaps currently merge over the default keymap. `Config::default()` sets
`keys: keymap::default()` (`helix-term/src/config.rs:27`), and `Config::load`
starts from `keymap::default()` before applying global and local key overrides
(`helix-term/src/config.rs:67`). `merge_keys` replaces leaves and recursively
merges nodes (`helix-term/src/keymap.rs:42`, `helix-term/src/keymap.rs:365`).

Pending input is trie-path state, not a general Vim command parser.
`Keymaps::state` stores pending keys while matching a key sequence, and
`Keymaps::sticky` stores sticky nodes (`helix-term/src/keymap.rs:264`). Lookup
returns `Pending`, `Matched`, `MatchedSequence`, `NotFound`, or `Cancelled`
(`helix-term/src/keymap.rs:247`). `Keymaps::get` clears pending state after a
matched command or sequence (`helix-term/src/keymap.rs:337`).

Helix already has some count support in command execution. For example,
`goto_line` checks `cx.count` and jumps to a counted line
(`helix-term/src/commands.rs:3971`), and `goto_column_impl` uses `cx.count()`
(`helix-term/src/commands.rs:4044`). That is command-local count handling, not
Vim's operator-count and motion-count multiplication.

The command inventory is broad. Static commands include motions, search,
selection, delete/change/yank/paste, pickers, diagnostics, LSP actions, window
commands, registers, text objects, macros, and command palette entries
(`helix-term/src/commands.rs:305`). The default keymap already binds core
motions (`helix-term/src/keymap/default.rs:9`), word motions
(`helix-term/src/keymap/default.rs:29`), insert/open commands
(`helix-term/src/keymap/default.rs:66`), delete/change/yank/paste
(`helix-term/src/keymap/default.rs:73`, `helix-term/src/keymap/default.rs:154`),
search (`helix-term/src/keymap/default.rs:142`), windows
(`helix-term/src/keymap/default.rs:193`), and space-leader picker/LSP workflows
(`helix-term/src/keymap/default.rs:225`).

Integration tests can execute key sequences against text and selection.
`test_with_config` runs a `TestCase` through `test_key_sequence_with_input_text`
and asserts output text and selection (`helix-term/tests/test/helpers.rs:247`).
`AppBuilder::with_config` can run tests with custom keymaps
(`helix-term/tests/test/helpers.rs:387`).

## Core Vim Normal-Mode Matrix

| Area                                                               | Vim/Neovim behavior                                                                                                                                                                                                                                                                                                                                                        | Velix/Helix evidence                                                                                                                                                                                                                                            | Classification                                                      | Notes                                                                                                                                                                                             |
| ------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `h`, `j`, `k`, `l` basic motions                                   | Vim maps `h`/`l` to counted character-left/right motions and `k`/`j` to counted line-up/down motions (`vendor/neovim/runtime/doc/motion.txt:168`, `vendor/neovim/runtime/doc/motion.txt:310`); LazyVim maps `j`/`k` to display-line movement when no count is present (`vendor/lazyvim/lua/lazyvim/config/keymaps.lua:7`).                                                 | Helix already maps `h`/`j`/`k`/`l` to character and visual-line movement (`helix-term/src/keymap/default.rs:9`).                                                                                                                                                | Remap only                                                          | Keep Helix's visual-line `j`/`k`; it already matches LazyVim's no-count behavior better than raw Vim. Count-sensitive real-line movement is a later refinement.                                   |
| Word motions `w`, `b`, `e`, `W`, `B`, `E`                          | Vim uses word/WORD motions as operator motions and standalone motions (`vendor/neovim/runtime/doc/motion.txt:35`).                                                                                                                                                                                                                                                         | Helix already exposes and binds word and long-word motions (`helix-term/src/keymap/default.rs:29`, `helix-term/src/commands.rs:321`).                                                                                                                           | Remap only for standalone movement; New grammar/state for operators | Standalone motion is easy. `dw`/`cw`/`yw` requires operator-pending semantics.                                                                                                                    |
| Line start/end `0`, `^`, `$`, `gg`, `G`                            | Vim operators depend on motion type, inclusive/exclusive rules, and linewise conversion rules (`vendor/neovim/runtime/doc/motion.txt:70`).                                                                                                                                                                                                                                 | Helix has `goto_line_start`, `goto_first_nonwhitespace`, `goto_line_end`, counted `goto_line`, and `goto_last_line` (`helix-term/src/keymap/default.rs:38`, `helix-term/src/keymap/default.rs:44`, `helix-term/src/commands.rs:3971`).                          | Remap only for standalone movement; New grammar/state for operators | Map `0`, `^`, `$`, `gg`, `G` in the profile. Operator use needs motion type metadata.                                                                                                             |
| Character find `f`, `F`, `t`, `T`                                  | Vim treats these as motions that can compose with operators (`vendor/neovim/runtime/doc/motion.txt:35`).                                                                                                                                                                                                                                                                   | Helix binds `f`, `F`, `t`, `T` to find/till commands (`helix-term/src/keymap/default.rs:14`).                                                                                                                                                                   | Remap only for standalone movement; New grammar/state for operators | Existing commands cover movement. Operator composition needs a parser and typed motion result.                                                                                                    |
| Insert, append, open `i`, `a`, `I`, `A`, `o`, `O`                  | Vim change docs separate insert/replace from delete/change operations and repeat simple changes with `.` (`vendor/neovim/runtime/doc/change.txt:7`, `vendor/neovim/runtime/doc/repeat.txt:16`).                                                                                                                                                                            | Helix already binds `i`, `a`, `I`, `A`, `o`, `O` to insert/open commands (`helix-term/src/keymap/default.rs:66`) and exposes the commands (`helix-term/src/commands.rs:400`).                                                                                   | Remap only initially                                                | Dot-repeat of insert/change should be deferred.                                                                                                                                                   |
| Delete/change/yank operators `d{motion}`, `c{motion}`, `y{motion}` | Vim operators wait for a motion, operate over moved text, support doubled linewise form, and multiply operator/motion counts (`vendor/neovim/runtime/doc/motion.txt:35`, `vendor/neovim/runtime/doc/motion.txt:55`, `vendor/neovim/runtime/doc/change.txt:33`, `vendor/neovim/runtime/doc/change.txt:164`).                                                                | Helix `d`, `c`, `y` operate on the existing selection (`helix-term/src/keymap/default.rs:73`, `helix-term/src/keymap/default.rs:154`); delete/change implementation deletes the current selection, optionally yanking it (`helix-term/src/commands.rs:2983`).   | New grammar/state                                                   | This is the main compatibility gap. Existing commands are useful primitives but do not parse operator plus motion.                                                                                |
| Repeated line operators `dd`, `cc`, `yy`                           | Vim doubled operators operate linewise; counts before or after the operator affect line count (`vendor/neovim/runtime/doc/motion.txt:58`, `vendor/neovim/runtime/doc/change.txt:37`, `vendor/neovim/runtime/doc/change.txt:174`).                                                                                                                                          | Helix can select line bounds with `x`/`X` and then delete/change/yank (`helix-term/src/keymap/default.rs:98`, `helix-term/src/keymap/default.rs:73`, `helix-term/src/keymap/default.rs:154`). Command sequences are supported (`helix-term/src/keymap.rs:144`). | New command or small grammar/state                                  | Could start as profile bindings to sequences like line-select plus operation, but counts and exact cursor/linewise register behavior likely need new commands. This is the best first code slice. |
| Paste `p`, `P`                                                     | Vim paste semantics depend on register type after linewise versus characterwise yank/delete (`vendor/neovim/runtime/doc/motion.txt:70`).                                                                                                                                                                                                                                   | Helix already binds `p`/`P` to paste after/before selection and has paste command leaves (`helix-term/src/keymap/default.rs:156`, `helix-term/src/commands.rs:502`).                                                                                            | Remap only initially; New command later for linewise fidelity       | Basic paste can stay. Correct Vim linewise paste depends on operator/register metadata.                                                                                                           |
| Undo/redo                                                          | Vim binds `u` to undo and `CTRL-R` to redo, both with optional counts (`vendor/neovim/runtime/doc/undo.txt:16`, `vendor/neovim/runtime/doc/undo.txt:33`).                                                                                                                                                                                                                  | Helix binds `u` to undo and `U` to redo, with earlier/later history on Alt variants (`helix-term/src/keymap/default.rs:149`, `helix-term/src/commands.rs:486`).                                                                                                 | Remap only for undo; New command or alias for redo                  | Keep `u`; add a Vim-profile redo binding for `C-r` if it does not conflict with insert-mode register insertion.                                                                                   |
| Search `/`, `?`, `n`, `N`, `*`                                     | Vim maps `/` and `?` to forward/backward search, `n`/`N` to repeat search in same/opposite direction, and `*` to word search (`vendor/neovim/runtime/doc/pattern.txt:17`, `vendor/neovim/runtime/doc/pattern.txt:51`, `vendor/neovim/runtime/doc/pattern.txt:61`); LazyVim refines `n`/`N` based on search direction (`vendor/lazyvim/lua/lazyvim/config/keymaps.lua:67`). | Helix binds `/`, `?`, `n`, `N`, `*` to search commands (`helix-term/src/keymap/default.rs:142`).                                                                                                                                                                | Remap only initially                                                | LazyVim's exact directional `n`/`N` semantics may need command tweaks later; not a blocker.                                                                                                       |
| Visual/select mode                                                 | Vim Visual mode selects an area, then applies an operator (`vendor/neovim/runtime/doc/visual.txt:20`); Visual operators include delete/change/yank/indent and objects (`vendor/neovim/runtime/doc/visual.txt:212`).                                                                                                                                                        | Helix Select mode is cloned from Normal then overridden to extend selections (`helix-term/src/keymap/default.rs:342`); `select_mode` sets `Mode::Select` (`helix-term/src/commands.rs:4096`).                                                                   | New command plus semantic decisions                                 | Helix select mode is a close UI fit, but multiple selections and selection-first behavior differ from Vim Visual. Start by mapping `v` and line selection, defer blockwise Visual.                |
| Text objects `iw`, `aw`, `ip`, etc.                                | Vim text objects are valid in Visual mode or after an operator, and `a`/`i` select around/inner objects (`vendor/neovim/runtime/doc/motion.txt:531`).                                                                                                                                                                                                                      | Helix exposes `select_textobject_around` and `select_textobject_inner` under `m a` and `m i` (`helix-term/src/keymap/default.rs:103`, `helix-term/src/commands.rs:567`).                                                                                        | New grammar/state                                                   | Existing text-object commands are promising primitives, but Vim grammar expects `diw`, `ciw`, `viw`, etc.                                                                                         |
| Registers                                                          | Vim operators accept optional register prefixes (`vendor/neovim/runtime/doc/change.txt:19`, `vendor/neovim/runtime/doc/change.txt:33`).                                                                                                                                                                                                                                    | Helix has `select_register`, register-aware delete/yank paths, and register commands (`helix-term/src/keymap/default.rs:331`, `helix-term/src/commands.rs:2989`, `helix-term/src/commands.rs:554`).                                                             | New grammar/state                                                   | The storage exists, but Vim's prefix grammar needs parser state.                                                                                                                                  |
| Macros `q`, `@`, `Q`                                               | Vim repeat docs describe register execution and repeat (`vendor/neovim/runtime/doc/repeat.txt:137`).                                                                                                                                                                                                                                                                       | Helix binds `Q` to record and `q` to replay; commands exist (`helix-term/src/keymap/default.rs:160`, `helix-term/src/commands.rs:610`).                                                                                                                         | Remap only for first approximation                                  | A Vim profile can swap `q` and replay bindings later. Exact `@{reg}` grammar is deferred.                                                                                                         |
| Counts                                                             | Vim counts multiply across operator and motion counts, and mappings have special count behavior (`vendor/neovim/runtime/doc/motion.txt:55`, `vendor/neovim/runtime/doc/map.txt:728`).                                                                                                                                                                                      | Helix has `cx.count` in some commands (`helix-term/src/commands.rs:3975`), but keymap pending state is sequence matching (`helix-term/src/keymap.rs:264`).                                                                                                      | New grammar/state                                                   | This is a hard boundary for real Vim compatibility.                                                                                                                                               |
| Dot-repeat `.`                                                     | Vim repeats the last change and can replace the previous count (`vendor/neovim/runtime/doc/repeat.txt:16`).                                                                                                                                                                                                                                                                | Helix default binds `A-.` to repeat last motion, not `.` to repeat last change (`helix-term/src/keymap/default.rs:20`, `helix-term/src/commands.rs:357`).                                                                                                       | New grammar/state                                                   | Defer. Dot-repeat needs change recording, count replacement, and operator integration.                                                                                                            |

## LazyVim Workflow Matrix

| Workflow                                   | LazyVim evidence                                                                                                                                                                                                                | Velix/Helix evidence                                                                                                                                                                                                                                                                                                                     | Classification                                                                                 | Notes                                                                                                                                                                       |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Leader groups/discoverability              | LazyVim uses which-key groups for `<leader>f`, `<leader>b`, `<leader>g`, `<leader>x`, windows, tabs, and more (`vendor/lazyvim/lua/lazyvim/plugins/editor.lua:60`).                                                             | Helix nested keymap nodes already have labels and can generate infobox content (`helix-term/src/keymap.rs:57`); default `space` node groups picker/window/LSP commands (`helix-term/src/keymap/default.rs:225`).                                                                                                                         | Remap only                                                                                     | `Space` can remain the practical leader while exposing LazyVim-like group names.                                                                                            |
| File finder and recent/config files        | LazyVim Snacks picker binds `<leader><space>`, `<leader>ff`, `<leader>fF`, `<leader>fg`, `<leader>fr`, and config pickers (`vendor/lazyvim/lua/lazyvim/plugins/extras/editor/snacks_picker.lua:59`).                            | Helix already has file picker, current-directory picker, file explorer, changed-file picker, and last picker under `space` (`helix-term/src/keymap/default.rs:225`, `helix-term/src/commands.rs:403`).                                                                                                                                   | Remap only for existing features; Deferred for recent/projects                                 | Map LazyVim's common finder keys to Helix pickers. Recent/project pickers need separate feature work.                                                                       |
| Live grep/search                           | LazyVim binds grep/search commands under `<leader>/`, `<leader>sg`, `<leader>sG`, `<leader>sw` (`vendor/lazyvim/lua/lazyvim/plugins/extras/editor/snacks_picker.lua:85`).                                                       | Helix has `global_search`, search, search selection, and picker command leaves (`helix-term/src/keymap/default.rs:287`, `helix-term/src/commands.rs:385`).                                                                                                                                                                               | Remap only                                                                                     | Helix `space /` already maps to global search. Add LazyVim aliases in the profile.                                                                                          |
| Buffers                                    | LazyVim binds previous/next buffer on `<S-h>`, `<S-l>`, `[b`, `]b`, plus buffer picker/delete commands (`vendor/lazyvim/lua/lazyvim/config/keymaps.lua:33`).                                                                    | Helix has next/previous buffer under `g n`/`g p`, buffer picker under `space b`, and last accessed/modified file commands (`helix-term/src/keymap/default.rs:55`, `helix-term/src/keymap/default.rs:230`).                                                                                                                               | Remap only for navigation/picker; New command for delete variants                              | Buffer deletion variants are not obvious in static command list; navigation and picker are easy.                                                                            |
| Diagnostics and quickfix-style lists       | LazyVim maps diagnostic jumps `[d`/`]d`, severity jumps, line diagnostics, Trouble diagnostics, loclist, and qflist (`vendor/lazyvim/lua/lazyvim/config/keymaps.lua:124`, `vendor/lazyvim/lua/lazyvim/plugins/editor.lua:205`). | Helix has previous/next/first/last diagnostics and diagnostic pickers (`helix-term/src/keymap/default.rs:111`, `helix-term/src/keymap/default.rs:234`, `helix-term/src/commands.rs:453`).                                                                                                                                                | Remap only for diagnostics; Deferred for quickfix/location list parity                         | Diagnostic navigation and pickers map well. Trouble/quickfix/location-list UI does not have a direct Helix equivalent.                                                      |
| LSP definitions/references/actions/symbols | LazyVim LSP key specs use `gd` for definition capability and capability-gated mappings (`vendor/lazyvim/lua/lazyvim/plugins/lsp/keymaps.lua:22`, `vendor/lazyvim/lua/lazyvim/plugins/lsp/keymaps.lua:57`).                      | Helix already binds goto definition/declaration/type/reference/implementation under `g`, code action under `space a`, symbols under `space s`/`space S`, references under `space h`, and rename under `space r` (`helix-term/src/keymap/default.rs:47`, `helix-term/src/keymap/default.rs:232`, `helix-term/src/keymap/default.rs:237`). | Remap only                                                                                     | This is a strong candidate for LazyVim compatibility aliases.                                                                                                               |
| Git hunks/log/blame                        | LazyVim maps hunk navigation/actions in gitsigns and git log/blame browse picker commands (`vendor/lazyvim/lua/lazyvim/plugins/editor.lua:150`, `vendor/lazyvim/lua/lazyvim/config/keymaps.lua:167`).                           | Helix has changed-file picker and previous/next change commands (`helix-term/src/keymap/default.rs:114`, `helix-term/src/keymap/default.rs:236`, `helix-term/src/commands.rs:415`).                                                                                                                                                      | Remap only for change navigation/changed files; New command or Deferred for hunk actions/blame | Start with changed-file and change navigation. Stage/reset/blame/log need VCS feature work or shell integration.                                                            |
| Windows and splits                         | LazyVim maps `<C-h/j/k/l>`, `<leader>-`, `<leader>\|`, `<leader>wd`, and window groups (`vendor/lazyvim/lua/lazyvim/config/keymaps.lua:13`, `vendor/lazyvim/lua/lazyvim/config/keymaps.lua:198`).                               | Helix has `C-w` and `space w` window nodes with split, close, only, jump, swap, rotate commands (`helix-term/src/keymap/default.rs:193`, `helix-term/src/keymap/default.rs:260`).                                                                                                                                                        | Remap only                                                                                     | Add LazyVim aliases while retaining existing Helix nodes.                                                                                                                   |
| Tabs                                       | LazyVim maps tab workflows under `<leader><tab>` (`vendor/lazyvim/lua/lazyvim/config/keymaps.lua:205`).                                                                                                                         | Helix's visible equivalent is buffers/views, not Vim tabpages; static commands include buffer navigation and split/window management, but not tabpage commands (`helix-term/src/commands.rs:465`, `helix-term/src/commands.rs:537`).                                                                                                     | Deferred                                                                                       | Treat as buffer/view workflows first. True tabpage compatibility is not a first milestone.                                                                                  |
| Terminal                                   | LazyVim binds floating terminal under `<leader>ft`, `<leader>fT`, and `<C-/>` (`vendor/lazyvim/lua/lazyvim/config/keymaps.lua:192`).                                                                                            | Helix static command list in this checkout does not expose a terminal command near the picker/window command inventory (`helix-term/src/commands.rs:403`, `helix-term/src/commands.rs:537`).                                                                                                                                             | Deferred                                                                                       | No direct Helix command found in the inspected command list.                                                                                                                |
| Save/quit/session                          | LazyVim maps save on `<C-s>`, quit all on `<leader>qq`, and session restore commands under `<leader>q` (`vendor/lazyvim/lua/lazyvim/config/keymaps.lua:80`, `vendor/lazyvim/lua/lazyvim/plugins/util.lua:49`).                  | Helix has command mode and typable commands, but static keymap evidence here only shows command mode entry and `C-s` selection save in normal mode (`helix-term/src/keymap/default.rs:64`, `helix-term/src/keymap/default.rs:223`).                                                                                                      | New command or command binding                                                                 | Save/quit can likely bind typable commands in keymap config, but this needs a focused check of typed command mapping behavior before implementation. Sessions are deferred. |
| UI toggles                                 | LazyVim binds many UI toggles under `<leader>u` (`vendor/lazyvim/lua/lazyvim/config/keymaps.lua:144`).                                                                                                                          | Helix has editor configuration and theme support, but the inspected static command list does not expose equivalent toggle commands except feature-specific commands like comments and diagnostics pickers (`helix-term/src/config.rs:12`, `helix-term/src/commands.rs:520`).                                                             | Deferred                                                                                       | Add only toggles that already exist as commands; do not block first build.                                                                                                  |

## First Implementation Slice

The next code experiment should add the keymap-profile mechanism and a tiny Vim
profile, then test it with existing integration helpers.

Expected files:

- `helix-term/src/keymap/default.rs`: keep existing default; add a separate Vim
  profile module/function only if this remains the right module boundary after
  code inspection.
- `helix-term/src/keymap.rs`: likely add profile plumbing only if the profile
  cannot be selected cleanly from config.
- `helix-term/src/config.rs`: add an editor/keymap profile option such as
  `editor.keymap = "vim"` or a top-level equivalent, then select
  `keymap::default()` versus `keymap::vim()`.
- `helix-term/tests/test/commands/`: add focused integration tests that build an
  `AppBuilder` with Vim profile config.

First behavior target:

- Standalone motions and workflow aliases:
  - `h`, `j`, `k`, `l`, `w`, `b`, `e`, `0`, `^`, `$`, `gg`, `G`
  - `i`, `a`, `I`, `A`, `o`, `O`
  - `/`, `?`, `n`, `N`
  - LazyVim-style aliases for file picker, global search, buffer picker,
    diagnostics picker, LSP goto/action/symbols, and windows.
- One linewise operator slice:
  - `dd` deletes the current line.
  - `yy` yanks the current line.
  - `cc` changes the current line and enters insert mode.

Verification plan:

- Add tests using `helix-term/tests/test/helpers.rs::test_with_config` and
  `AppBuilder::with_config` (`helix-term/tests/test/helpers.rs:247`,
  `helix-term/tests/test/helpers.rs:387`).
- Test that the default keymap remains unchanged when the Vim profile is not
  selected.
- Test that the Vim profile maps representative standalone motions and LazyVim
  aliases.
- Test `dd`, `yy`, and `cc` on a small buffer, including cursor position and
  resulting selection.
- Run:

  ```bash
  cargo fmt
  cargo test -p helix-term keymap
  cargo test -p helix-term --test test
  ```

The linewise operator slice can be implemented either as new commands
(`vim_delete_line`, `vim_yank_line`, `vim_change_line`) or as a tiny
operator-state parser. The next experiment should choose the smaller approach
after inspecting how key sequence execution handles multi-command mappings in
tests. If command sequences preserve enough behavior for `dd`/`yy`/`cc`, use
that first; otherwise add the linewise commands and defer a general operator
parser.

## Risks and Deferrals

- **Operator-pending grammar:** Vim has a distinct operator-pending mapping mode
  (`vendor/neovim/runtime/doc/map.txt:364`) and operator motions can be remapped
  separately (`vendor/neovim/runtime/doc/map.txt:450`). Helix has only
  Normal/Select/Insert keymap roots in this checkout
  (`helix-term/src/keymap/default.rs:407`). Full operator mode is deferred.
- **Count multiplication:** Vim multiplies operator and motion counts
  (`vendor/neovim/runtime/doc/motion.txt:55`); Helix count handling is
  command-local (`helix-term/src/commands.rs:3975`). Defer until after the first
  linewise operator slice.
- **Motion type metadata:** Vim distinguishes linewise, charwise, inclusive, and
  exclusive motions (`vendor/neovim/runtime/doc/motion.txt:70`). Existing Helix
  movement commands move or extend selections but do not expose a reusable Vim
  motion type in the keymap layer (`helix-term/src/keymap.rs:247`). Defer.
- **Multiple selections:** Helix commands operate naturally over multiple
  selections; Vim operators usually operate from a single cursor or Visual
  selection. The first profile should preserve Helix multiple-selection behavior
  unless a compatibility test explicitly requires Vim-like narrowing.
- **Registers and linewise paste:** Helix has register storage and register
  selection (`helix-term/src/keymap/default.rs:331`), but Vim linewise paste
  depends on yanked/deleted text type. Defer exact register-type fidelity.
- **Dot-repeat:** Vim `.` repeats the last change with optional count
  replacement (`vendor/neovim/runtime/doc/repeat.txt:16`). Helix currently
  exposes repeat last motion, not repeat last change, in the default keymap
  (`helix-term/src/keymap/default.rs:20`). Defer.
- **LazyVim plugin features:** Trouble, gitsigns staging/reset/blame, terminal,
  tabpages, projects/recent files, sessions, and UI toggles need either Helix
  feature equivalents or new commands. Do not block the first usable Vim profile
  on these.
