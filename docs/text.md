# Text rendering

How labels turn into pixels, and the knobs that exist to match other renderers,
added while making the skaityk port pixel identical to its WebKit original.

## Pipeline

`draw_label` in `ui_drawer.rs` builds a `Section` per label and queues it on the
label's `Font`. Each `Font` owns a `wgpu_text::TextBrush` for rasterization and a
`rustybuzz::Face` for shaping. Glyphs are positioned by `ShapedLayout`
(`deps/window/src/text/shaped_layout.rs`), a custom `GlyphPositioner` that shapes
every line with rustybuzz and hands pre-positioned glyphs to glyph_brush.

Shaping through rustybuzz exists because ab_glyph reads only the legacy `kern`
table. Modern fonts, SF Pro included, keep kerning in `GPOS`, so the builtin
glyph_brush layout renders them with no kerning at all. rustybuzz applies GPOS,
GSUB and variation aware kerning like CoreText and browsers do.

## Sizes are pixels per em

`Label::text_size` means pixels per em, the CSS convention. ab_glyph `PxScale`
means ascent minus descent, a different unit. `Font::em_scale()` converts, both
the drawer and `Font::measure` multiply by it. For fonts whose ascent minus
descent equals their units per em, Helvetica, nothing changes. For SF Pro the
difference is 18 percent.

## Variable fonts

`Font::with_variations(name, data, &[(*b"wght", 550.0), (*b"opsz", 17.0)])` loads
a variable font instance with axes pinned. Each combination is its own managed
instance, cache under a name that includes the values. Axis values apply to both
the raster font and the shaping face. A missing axis is an error.

## Letter spacing

`Label::set_letter_spacing(points)` adds tracking between glyphs, applied by
`ShapedLayout` after kerning, mirrored in `Font::measure`. `Button` forwards it,
and `set_font`, to its internal label. Needed to match platforms that apply the
font's `trak` curve automatically, macOS does for the system font.

## Line handling

`\n` always breaks lines, single line labels included. Multiline labels
additionally wrap greedily at spaces to the label width. A single word wider
than the bound overflows, same as the builtin layout did. Vertical alignment
is always center.

## Matching other renderers

The engine blends glyph coverage in linear space, browsers blend perceptually.
Light text on dark renders slightly thicker here, dark on light slightly
thinner, so a pixel perfect port tunes `wght` per polarity instead of using the
nominal weight. Measuring workflow, scripts and the trak table details live in
the test-engine skill's migration chapter, next to this repo's users.
