# Engine gaps

Missing engine features, found by porting a real app. Each entry lists the current state
in code, what is needed, and what it blocks. When one of these lands it needs UI tests
like any other engine change.

The driver app is skaityk-te at `~/dev/apps/skaityk-te`, github.com/gebling-studio/skaityk-te.
It is a rewrite of skaityk, a Tauri and Vue reader app for Lithuanian learners at
`~/dev/apps/skaityk`. Full visual and functional parity with the original is the
acceptance bar. The news feed part is done with the engine as is. The reader and the
app-wide look need the features below.

Already proven sufficient during the port, for reference: `TableView` with columns and
`bottom_reached` paging, `ImageMode::AspectFill`, `Image::download`, SVG in textures,
`size_changed`, `ModalView`, `OnDisk` storage, `NavigationView` push and pop,
`Window::set_title`.

Landed from this list already, each with UI tests: font wiring, an app-wide default via
`Font::set_default` plus per-label `Label::set_font`, and named keys in `Keymap`, arrows,
enter and friends via `NamedKey`.

## 1. Flow-wrap layout container

The largest gap. The core interaction of the driver app sits on it.

- Current: the placer has side, anchor, center and tiling rules,
  `deps/ui/src/layout/placer/setup.rs`. Tiling knows Background, LeftHalf, RightHalf,
  Vertically, Horizontally, Distribute. Nothing wraps. `TableView` is the only
  content-driven container and it is vertical only.
- Needed: a container that lays out subviews left to right in row order, wraps to the
  next row when the width is exceeded, sizes rows by the tallest child, and re-wraps on
  resize. Children need intrinsic sizes, which for text means gap 2.
- Blocks: the skaityk reader word grid. One sentence renders as a wrapping row of
  tappable word views. Tapping a word shows its translation in a mini label above the
  word at half font size. A second panel shows a gray slot per word, sized by word
  length, that reveals the translated word. None of this can be laid out today.

## 2. Label content measurement

- Current: `Label` has no content size API, multiline text just draws into whatever
  frame the layout gives it, `deps/ui/src/views/basic/label.rs`. No auto-height, no
  fit-to-text.
- Needed: measure text at a font size to get content size, then a label mode or placer
  rule where height follows the text. Combined with `ScrollView` for overflow. If gap 6,
  the text stack rework, ever happens it provides this natively, check before building
  it twice.
- Blocks: the reader panels. Sentences can be long and the font size goes up to 144, the
  panel must grow and scroll. Also feeds gap 1, word views in the flow grid must size
  themselves to their word.

## 3. Runtime theming

- Current: `Style` applies once, at view setup, `deps/ui/src/style.rs`.
  `apply_globally` affects only views created after the call. No re-style of a live view
  tree, no OS theme detection.
- Needed: a theme switch that re-applies colors to existing views, plus system light and
  dark detection with a change event. winit exposes the window theme and a ThemeChanged
  event, so the platform part is mostly plumbing.
- Blocks: dark mode. The original app follows the system theme live.

## 4. Hover events

- Current: the input pipeline is touch only, `deps/ui/src/view/view_touch.rs`. No per-view
  hover tracking from mouse moves.
- Needed: hover enter and exit events on the view under the cursor, desktop only.
- Blocks: desktop polish, card lift on hover, button hover colors.

## 5. Drop shadows

- Current: the rect pipeline draws fill, border and corner radius. No shadow.
- Needed: shadow rendering under rounded rects plus shadow parameters on `ViewData`.
- Blocks: card elevation, the original uses a small resting shadow and a larger hover one.

## 6. Text stack rework

Found by the FontZoo emoji page. Parked until a real need, the items above come first.

- Current: text renders through wgpu_text, glyph_brush and ab_glyph — outline glyphs
  only, a single channel atlas tinted by the text color. Color emoji tables, CBDT, sbix
  and COLR, are ignored, so emoji render monochrome via the bundled `NotoEmoji.ttf`.
  There is no shaping, multi codepoint emoji and ligatures do not combine.
- Needed: migrate label rendering to cosmic-text with swash. Brings shaping, font
  fallback chains, color emoji in every format, and native text measurement, which
  subsumes gap 2. Large: replaces the glyph atlas and `draw_label`, and invalidates
  every recorded text expectation in the UI tests.
- Blocks: colorful emoji, complex scripts. Nothing in the driver app today.

## 7. Small niceties

- `TableView` cell spacing. Worked around in skaityk-te with a transparent cell and an
  inset card subview.
- Per-corner radius. The original rounds only the top corners of a card image.
- Native modal backdrop. `ModalView` shows no scrim, the original dims and blurs behind
  the dialog. An app can fake the dim with a fullscreen translucent container.
- Backdrop blur for the sticky header. Render pass work, lowest priority.

## Suggested order

Flow-wrap, then label measurement, then theming, then hover and shadows, then the
niceties. Flow-wrap and measurement unlock the reader, which is the whole point of the
driver app. The text stack rework waits for a real need for color emoji or complex
scripts, but check it before building label measurement, it provides that natively.
