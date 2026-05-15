# Changelog

All notable changes to this project will be documented in this file.
The format is based on Keep a Changelog, and this project adheres to SemVer.

## [Unreleased]

- TBD.

## [0.4.9] - 2026-05-15

- Fixed top tabs truncating filenames to "n…" at startup. Root cause:
  `TCM_SETITEMSIZE` was called from `add_tab` during `WM_CREATE` while the
  tab control still had 0×0 screen dimensions; Windows silently ignored the
  size until the control was shown. Fixed by calling `refresh_top_tab_item_size`
  (and `InvalidateRect`) from `layout_children` after `SetWindowPos` gives the
  tab strip its real dimensions. This fires on both the initial layout and on
  every subsequent `WM_SIZE`, so the correct item size is always applied once
  the control is on screen.

## [0.4.8] - 2026-05-15

- Bumped the per-tab max width clamp from 260 to 400 DPI-scaled px so most
  filenames fit fully in the tab label before any ellipsis truncation kicks
  in. Tabs still grow to fit their text individually; the close `×` stays
  anchored to the right edge.
- Extended dark mode beyond the editor to cover the rest of the window
  chrome, matching Notepad++:
  - **Title bar** now follows the dark theme via
    `DwmSetWindowAttribute(DWMWA_USE_IMMERSIVE_DARK_MODE)` (attribute 20
    on Win10 20H1+ with an attribute-19 fallback for older builds).
  - **Menu bar** background and items are painted dark via the
    undocumented `WM_UAHDRAWMENU` / `WM_UAHDRAWMENUITEM` messages, with
    `SetPreferredAppMode` (uxtheme.dll ordinal 135) +
    `AllowDarkModeForWindow` (ordinal 133) covering popup/context menus.
  - **Status bar** is now subclassed: `WM_PAINT` paints each part with the
    same `tab_host.theme` palette used by the tab strip, with `theme.border`
    separators between parts.
  - **Find / Replace / Find-in-Files / Go-To-Line** dialogs apply dark
    title bars and dark backgrounds on creation; `WM_CTLCOLOR*` returns
    cached dark brushes so Static / Edit / Button / ListBox controls all
    pick up theme colors.
- Added `src/platform/dark_mode.rs` to encapsulate every undocumented
  uxtheme/UAH binding behind safe wrappers that no-op cleanly on older
  Windows builds.

## [0.4.7] - 2026-05-15

- Removed the "Toggle Checkbox" and "Place Checkbox" commands from the editor
  right-click context menu and from the editor command set entirely. The
  `textops/checkbox` module, its helper wrappers in `win32.rs`, and the
  associated `CMD_TOGGLE_CHECKBOX` / `CMD_INSERT_CHECKBOX` constants are all
  gone. Files containing `- [ ] task` style lines continue to display normally
  — the feature was edit-time only with no display side-effects.

## [0.4.6] - 2026-05-15

- Replaced the Hide-Lines / Unhide-All commands with a **Collapse Selection**
  feature available from the editor's right-click context menu. Highlight any
  block of lines, choose `Collapse Selection`, and the first selected line
  stays visible with a clickable `+` marker in the fold gutter; everything
  below it is hidden. Click the `+` to expand. `Expand All Collapsed` in the
  same context menu restores every user-collapsed block in one shot.
- Removed the old `View -> Hide Lines` / `View -> Unhide All Lines` menu items
  and their `Alt+H` / `Alt+Shift+H` accelerators. The underlying
  `SCI_HIDELINES` plumbing is reused by the new collapse path.
- Fixed top tabs truncating to `n...` — every tab now sizes to fit its
  filename (clamped 120-260 DPI-scaled px). Width recomputes on tab open,
  close, rename, and dirty-flag toggle.
- Removed the lighter "buffer bar" beneath the tab strip by subclassing the
  top tab control to paint `WM_ERASEBKGND` and the bottom 3-px seam with the
  active tab theme background. The tab row now meets the editor cleanly in
  both light and dark mode.
- Internal: new Scintilla wrappers for `SCI_MARKERADD` / `DELETE` /
  `DELETEALL` / `GET` and `SCI_GETLINEVISIBLE`; new
  `USER_COLLAPSE_MARKER` (#5) with `SC_MARK_PLUS` shape; fold-margin mask
  extended to render it alongside the lexer fold markers.

## [0.4.5] - 2026-05-15

- Reworked strikethrough back to an indicator-based toggle. Selecting text and
  invoking `Strikeout` (menu, context menu, or new `Ctrl+Shift+X` accelerator)
  now draws a clean strike line with **no `~~` characters added to the buffer**.
  Re-invoking on already-struck text removes the strike. Strike state is
  persisted across close+reopen via the existing `session.json` (no per-file
  sidecar). Duplicate Tab carries strike state into the new tab.
- Removed the v0.4.4 markdown rescan plumbing (`TIMER_MD_STRIKE`, the debounced
  rehighlight loop, `md_strike_pending`/`md_strike_timer` state, the "Too many
  strike matches" status flag) and the `textops/markdown_strike` module.
  Indicator runs are written/read via the restored Scintilla
  `SCI_INDICATOR*` query APIs.
- Added a line-number margin on every editor. Gutter width auto-sizes from the
  current line count via `SCI_TEXTWIDTH(STYLE_LINENUMBER, …)` and re-fits on
  edit so the digits never clip.
- Added a click-sensitive fold margin with plus/minus box markers (Scintilla
  `SC_MASK_FOLDERS` on margin 1). Enabled Lexilla fold properties (`fold`,
  `fold.compact=0`, plus html/preprocessor/comment variants) so all 9 wired
  lexers (cpp/js/json/yaml/powershell/python/html/xml/css/props) get folds for
  free.
- Added Markdown heading-based folds. `.md` / `.markdown` routes to a new
  `LexerKind::Markdown`; fold levels are recomputed on `SCN_MODIFIED` from `#`
  through `######` depth (large-file gated). Lexer styling stays minimal for
  now.
- Contracted folds show inline ` ⋯ N lines ` in a boxed display style. Manual
  `SCN_MARGINCLICK` handling calls `SCI_TOGGLEFOLDSHOWTEXT` so the count is
  computed at toggle time. Existing `Alt+H` Hide-Lines / `Alt+Shift+H`
  Unhide-All commands keep working alongside the new fold UI.
- Themed gutter, line numbers, and fold markers from the existing tab/editor
  theme so light/dark mode switches recolor everything consistently.
- Added `/.claude` and `/agents` to `.gitignore` to stop local working
  directories from cluttering `git status`.

## [0.4.4] - 2026-05-15

- Fixed `Ctrl+S` not saving: the accelerator was bound only to `Ctrl+Shift+S`
  (Save All), so plain `Ctrl+S` fell through to Scintilla and inserted a
  control-character glyph into the editor. Added a dedicated
  `Ctrl+S -> IDM_FILE_SAVE` accelerator.
- Added a persistent close 'x' button on every tab in all three placements
  (Top, Left, Right). Clicking the 'x' routes through the existing `close_tab`
  flow, preserving the dirty-document save prompt.
- Made active-tab highlighting consistent across placements. Top tabs are now
  owner-drawn (`TCS_OWNERDRAWFIXED`) and the vertical `ListView` custom-draw
  handler explicitly fills the full row, so the selected/hover state reads
  cleanly in both modes and tracks `tab_host.theme` (light/dark).
- Replaced the manual toggle-strikethrough command with a markdown-driven
  `Strikeout` command: selected text is wrapped in `~~...~~` and the editor
  re-scans documents on edit to apply the strike indicator to all matching
  spans, with a `Too many strike matches` status flag past the cap. Removed
  the now-unused Scintilla indicator value APIs and session-stored strike
  ranges.
- Added a debounced 250 ms strikethrough re-highlight timer
  (`TIMER_MD_STRIKE`) keyed off `SCN_MODIFIED` to keep editor responsiveness
  during rapid typing.
- Added a `CLAUDE.md` guide at the repo root so future Claude Code sessions
  can orient quickly to build commands and module layout.

## [0.4.3] - 2026-03-03

- Added a `View -> Dark Mode` toggle so users can switch between light and dark themes.
- Persisted editor theme choice in `settings.json` via a new `editor_dark` field.
- Fixed startup theme initialization to honor persisted settings instead of forcing dark mode.

## [0.4.2] - 2026-03-03

- Added selection-driven Smart Highlight using Scintilla container indicators
  (`INDIC_ROUNDBOX`) with theme-aware colors/alpha and bounded
  `SCI_SEARCHINTARGET` scanning.
- Added temporary line folding commands in `View`: `Hide Lines` and
  `Unhide All Lines`, including keyboard shortcuts (`Alt+H`, `Alt+Shift+H`).
- Added document-tab keyboard cycling with wrap-around for
  `Ctrl+Tab` / `Ctrl+Shift+Tab` and `Ctrl+PageDown` / `Ctrl+PageUp`.
- Introduced Large File Mode restrictions with configurable threshold and
  toggles in `settings.json`, including smart-highlight suppression by default
  and optional global word-wrap deactivation.
- Updated status/title indicators to surface Large File Mode state and
  smart-highlight truncation ("Too many matches").
- Added unit tests for new settings fields/clamping and large-file/token helper logic.

## [0.4.1] - 2026-03-03

- Fixed CI failures for `cargo fmt --check` and `cargo clippy -- -D warnings`
  on the `v0.4.0` line.
- Aligned release gating with CI by adding `fmt` and `clippy` checks to
  `.github/workflows/release.yml` before tests/build/publish.
- Validated the updated CI pipeline end-to-end on `main` with all required jobs green.

## [0.4.0] - 2026-03-03

- Introduced a `TabStripHost` architecture that supports three tab placements:
  `Top`, `Left`, and `Right`, while keeping document logic unchanged.
- Added persisted UI settings in `%LOCALAPPDATA%\Rivet\settings.json`:
  `tab_placement` (`top|left|right`) and `vertical_tab_width_px`.
- Replaced vertical `ListBox` tabs with a custom-drawn `ListView`-based vertical
  tab panel to avoid unsupported Win32 `TCS_VERTICAL` behavior under ComCtl32 v6.
- Implemented vertical tab theming via `NM_CUSTOMDRAW` with explicit light/dark
  palette colors for background, selection, hover, and text.
- Added/updated `View -> Tabs -> Top|Left|Right` menu controls with checked
  radio-style behavior and persistent placement updates.
- Kept `Ctrl+Alt+T` placement cycling and wired it through the new placement model.
- Implemented splitter drag resize with capture-based behavior and persisted width.
- Switched child-window layout positioning to `SetWindowPos` for tabs, splitter,
  status bar, and editor windows.
- Added placement-agnostic tab context hit testing for both top `TabCtrl` and
  vertical `ListView` tabs.
- Standardized dirty tab label rendering in both tab modes with a trailing `*`.
- Added targeted settings tests for serialization shape, defaults, roundtrip,
  and width clamping.

## [0.3.1] - 2026-03-02

- Added `Edit -> Go To Line...` with `Ctrl+G` and Scintilla `SCI_GOTOLINE` navigation,
  including 1-based line input prefilled from the current caret line and clamped to file bounds.
- Completed core Find/Replace behavior with standard keyflow:
  `Ctrl+F`, `Ctrl+H`, `F3`, `Shift+F3`, wrap-around, match case, whole word,
  and `Replace` now advancing to the next match after replacement.
- Kept `Replace All` as a single undo step via grouped Scintilla undo actions.
- Fixed CI clippy gating issue (`collapsible_if`) so `cargo clippy -- -D warnings`
  passes in GitHub Actions.

## [0.3.0] - 2026-03-01

- Implemented Notepad++-style `remember_session` + `session_snapshot_periodic_backup`
  behavior with default-on periodic backups and no save prompts on exit when enabled.
- Added crash-resilient atomic writes for backup and session files
  (`ReplaceFileW` with `MoveFileExW` fallback), plus startup cleanup for stale temp files.
- Implemented backup-first restore semantics for dirty tabs at shutdown and full-tab
  session restoration (named and untitled documents).
- Added global `View` menu with checkable toggles for `Word Wrap` and `Always On Top`,
  with persisted settings and startup re-application.
- Upgraded find/replace internals to `SCI_SEARCHINTARGET`-based search with grouped
  `Replace All` undo behavior and Notepad++-style replace flow.
- Updated status bar fields to show authoritative editor state:
  `Ln/Col`, `Sel`, `EOL`, `ENC`, and dirty indicator.
- Added `Help -> About Rivet` modal with version, git SHA, build UTC, source URL,
  and local data directory, including copy-to-clipboard action.
- Added build metadata injection in `build.rs`
  (`RIVET_VERSION`, `RIVET_GIT_SHA`, `RIVET_BUILD_UTC`, `RIVET_SOURCE_URL`).
- Hardened CI with separate `fmt`, `clippy`, `test`, and scheduled RustSec `cargo audit`
  workflow jobs.
- Added release compliance assets:
  `NOTICE.txt` and `THIRD_PARTY_NOTICES/Scintilla-Lexilla-License.txt`,
  and included them in portable + installer packaging.

## [0.2.1] - 2026-03-01

- Added a tab-bar right-click context menu with tab-scoped actions:
  `Save`, `Save As...`, `Duplicate Tab`, `Close`, `Close Others`,
  `Close Tabs to the Left`, and `Close Tabs to the Right`.
- Implemented tab hit-testing on right click and selection handoff so actions
  apply to the clicked tab.
- Expanded the editor right-click context menu with standard commands:
  `Undo`, `Redo`, `Cut`, `Copy`, `Paste`, `Delete`, and `Select All`,
  while keeping text transform and trim actions available.
- Added command enable/disable logic for editor context actions using Scintilla
  capability queries (`SCI_CANUNDO`, `SCI_CANREDO`, `SCI_CANPASTE`,
  and selection-state checks).
- Added Scintilla wrapper functions/constants needed for context-menu command
  state and delete behavior.

## [0.2.0] - 2026-03-01

- Added parent-owned editor context menu with exactly three commands:
  `Uppercase`, `Lowercase`, and `Trim Leading + Trailing Whitespace`.
- Disabled Scintilla default popup (`SCI_USEPOPUP(SC_POPUP_NEVER)`) so context
  menu behavior is consistent and app-controlled.
- Added Scintilla key bindings for text transforms:
  `Ctrl+U` (lowercase) and `Ctrl+Shift+U` (uppercase).
- Added `Edit -> Copy to Clipboard` operations:
  `Copy Full Path`, `Copy Filename`, and `Copy Directory Path`.
- Added enable/disable command state logic so no-op actions are greyed out:
  selection-based transform enablement and saved-path-based copy enablement.
- Added pure command/text logic modules and unit tests for trim semantics,
  copy-path behavior, and command enablement decisions.
- Added/updated CI to enforce `cargo fmt --check`,
  `cargo clippy -- -D warnings`, and `cargo test` on Windows.

## [0.1.2] - 2026-02-25

- Removed the editor's left gutter/padding for a flush text area.
- Improved dark-mode caret visibility.
- Focus editor automatically when selecting tabs.
- Suppressed the console window for release builds.

## [0.1.1] - 2026-02-25

- Vertical tab layout with resizable sidebar and layout cycling.
- Status bar enhancements with line/column and word count.
- Word wrap enabled by default with a toggle.
- Added always-on-top toggle and new file/save all commands.
- Multi-size app icon embedded and installer polish.
- Added unit tests for core text/session/find logic and CI release size reporting.

## [0.1.0] - 2026-02-24

- Win32 scaffolding with Scintilla editor host.
- File I/O with encoding and EOL preservation.
- Tabs, session restore, and edit commands.
- Find/replace and find-in-files with cancellation.
- Lexilla-backed syntax highlighting for a curated set.
- Editor dark mode toggle and per-monitor DPI awareness v2.
- Local logging with rotation and opt-in verbosity.
