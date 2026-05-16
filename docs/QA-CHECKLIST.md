# Manual QA Checklist

A short checklist of UI behaviors that don't have automated test coverage
(everything inside `src/platform/win32.rs` is Win32 GUI code). Run through
this before tagging any `v0.x.y` release — the CI workflow only verifies
`fmt`, `clippy`, `test`, and that the portable zip + installer build.

Companion to `docs/RELEASE_CHECKLIST.md`, which covers packaging and
signing.

## Launching

- [ ] `cargo run` launches without an error dialog.
- [ ] The window title bar matches the system theme (dark in dark mode).
- [ ] Menu bar paints with the same theme (light + dark).
- [ ] Status bar paints with the theme; line/col, EOL, encoding, dirty flag
  all show correct values.

## Top tabs (`View → Tabs → Top`)

- [ ] Open 3 files: each tab sizes to fit its filename (no `n…` truncation
  on short names).
- [ ] Active tab uses `selection_bg`; inactive tabs use `bg`; hovered tab
  uses `hover_bg`.
- [ ] Close `×` is visible on every tab and brightens on hover.
- [ ] Clicking `×` closes the tab (dirty-prompt still fires for unsaved
  changes).
- [ ] Opening a file with a very long path: that tab grows to fit; other
  tabs stay at their own widths.
- [ ] Rename or modify a tab so the `*` dirty marker appears/disappears —
  the tab width updates.

## Vertical tabs (`View → Tabs → Left` and `Right`)

- [ ] All open tabs are **visible** (regression guard for v0.4.13–v0.4.14).
- [ ] Only the active tab shows `selection_bg`; others show `bg`.
- [ ] Hovering shows `hover_bg`.
- [ ] Clicking another tab moves the highlight with **no light-mode flash**
  (regression guard for v0.4.16).
- [ ] Close `×` visible on each row and brightens on hover; clicking closes
  the tab.
- [ ] Splitter drag resizes the strip; the new width persists across
  restarts.

## Dark-mode toggle (`View → Dark Mode`)

- [ ] Toggling at runtime repaints the title bar, menu, status bar, editor,
  top tabs, and vertical tabs without needing a restart.
- [ ] Find / Replace / Find-in-Files / Go-To-Line dialogs honor the current
  theme when opened.

## Editing essentials

- [ ] `Ctrl+S` saves (regression guard for v0.4.4 — a stray glyph used to
  be inserted instead).
- [ ] `Ctrl+Tab` and `Ctrl+Shift+Tab` cycle through tabs with wrap-around.
- [ ] Right-click context menu on the editor shows the expected commands
  with proper enable/disable states.
- [ ] Right-click on a tab shows the tab context menu and actions apply to
  the clicked tab, not necessarily the active tab.

## Session restore

- [ ] Close the app with multiple tabs open (some dirty, some clean) — on
  next launch they restore (named + untitled).
- [ ] Force-kill the app process while editing — backup restore prompt
  fires on next launch and content matches.

## Large File Mode

- [ ] Open a file larger than the threshold (default 20 MB) — status bar
  surfaces the Large File Mode flag, word wrap is off, syntax highlighting
  is suppressed.

## Markdown folds

- [ ] Open a `.md` file with `#` headings — fold markers appear in the
  fold gutter; clicking a marker collapses/expands the section.

## Tip

If you find a new bug class that this checklist would miss, add a line.
It's easier to extend a living checklist than to re-derive the list from
six months of release notes.
