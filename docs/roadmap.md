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
`Font::set_default` plus per-label `Label::set_font`, named keys in `Keymap`, arrows,
enter and friends via `NamedKey`, and label content measurement, `Label::content_size`
and `size_for_width` on top of `Font::measure`, with fit-to-text placer rules
`fit_text_width`, `fit_text_height` and `fit_text` that compose with anchors and
min and max clamps. Flow-wrap layout landed as the `all_wrap` placer tiling rule.
Subviews flow left to right in declaration order and wrap into rows, children keep
their own sizes including fit-to-text, hidden children take no space, and the
container height follows the content. Row and item gaps come from `all(margin)`.
This unblocked the skaityk reader word grid. Runtime theming landed as `DynamicColor`
light and dark pairs accepted by every color setter. `Theme` picks the effective look,
`ThemeMode` follows the OS or forces one. A switch re-resolves bound colors on the
live view tree in one walk and fires `UIEvents::theme_changed`, the draw path keeps
reading plain resolved colors. The OS theme arrives through winit `ThemeChanged` and
is read once at startup. This unblocked dark mode.

## 1. Hover events

- Current: the input pipeline is touch only, `deps/ui/src/view/view_touch.rs`. No per-view
  hover tracking from mouse moves.
- Needed: hover enter and exit events on the view under the cursor, desktop only.
- Blocks: desktop polish, card lift on hover, button hover colors.

## 2. Drop shadows

- Current: the rect pipeline draws fill, border and corner radius. No shadow.
- Needed: shadow rendering under rounded rects plus shadow parameters on `ViewData`.
- Blocks: card elevation, the original uses a small resting shadow and a larger hover one.

## 3. Text stack rework

Found by the FontZoo emoji page. Parked until a real need, the items above come first.

- Current: text renders through wgpu_text, glyph_brush and ab_glyph — outline glyphs
  only, a single channel atlas tinted by the text color. Color emoji tables, CBDT, sbix
  and COLR, are ignored, so emoji render monochrome via the bundled `NotoEmoji.ttf`.
  There is no shaping, multi codepoint emoji and ligatures do not combine.
- Needed: migrate label rendering to cosmic-text with swash. Brings shaping, font
  fallback chains, color emoji in every format, and native text measurement, which
  would replace the `glyph_bounds` based `Font::measure`. Large: replaces the glyph
  atlas and `draw_label`, and invalidates every recorded text expectation in the UI
  tests.
- Blocks: colorful emoji, complex scripts. Nothing in the driver app today.

## 4. Small niceties

- `TableView` cell spacing. Worked around in skaityk-te with a transparent cell and an
  inset card subview.
- Per-corner radius. The original rounds only the top corners of a card image.
- Native modal backdrop. `ModalView` shows no scrim, the original dims and blurs behind
  the dialog. An app can fake the dim with a fullscreen translucent container.
- Backdrop blur for the sticky header. Render pass work, lowest priority.

## Suggested order

Hover, then shadows, then the niceties. The reader layout and dark mode are
unblocked, so what remains is desktop polish and small visual parity gaps. The
text stack rework waits for a real need for color emoji or complex scripts.
