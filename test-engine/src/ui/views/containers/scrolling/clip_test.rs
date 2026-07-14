use anyhow::Result;
use hreads::from_main;
use refs::Weak;

use crate::{
    self as test_engine,
    gm::color::{BROWN, Color, GRAY_BLUE, GREEN, LIGHT_BLUE, ORANGE, PURPLE, RED, TURQUOISE, WHITE, YELLOW},
    ui::{ImageView, Label, ScrollView, Setup, ViewData, ViewFrame, ViewSubviews, ViewTest, view_test},
    ui_test::{check_colors, inject_scroll, inject_touches, set_record_probe_count},
};

const ITEM_COLORS: [Color; 8] = [GREEN, YELLOW, ORANGE, PURPLE, TURQUOISE, LIGHT_BLUE, RED, BROWN];

// A header above and a footer below the scroll, with background gaps
// between them and the scroll edges. The content is a list of colored
// text items and a photo, three times taller than the scroll. While
// scrolling, items get cut mid element at both edges. Nothing may ever
// draw over the header, the footer or the gaps.
#[view_test]
struct ScrollClipTest {
    #[init]
    header: Label,
    footer: Label,
    scroll: ScrollView,
}

impl Setup for ScrollClipTest {
    fn setup(mut self: Weak<Self>) {
        self.header.set_text("HEADER").set_color(GRAY_BLUE).set_text_color(WHITE);
        self.header.place().tl(0).size(600, 100);

        self.footer.set_text("FOOTER").set_color(GRAY_BLUE).set_text_color(WHITE);
        self.footer.place().t(500).l(0).size(600, 100);

        self.scroll.set_content_size((400, 900));
        self.scroll.place().t(150).l(100).size(400, 300);

        for (i, color) in ITEM_COLORS.into_iter().enumerate() {
            let item = self.scroll.add_view::<Label>();
            item.set_color(color);
            item.set_text(format!("ITEM {i}"));
            item.set_frame((0, 100 * i, 400, 100));
        }

        let photo = self.scroll.add_view::<ImageView>();
        photo.set_image("cat.png");
        photo.set_frame((0, 800, 400, 100));
    }
}

// The gaps between the scroll edges and the header and footer stay
// pure background in every scroll position.
fn check_no_leak() -> Result<()> {
    check_colors(
        r"
             300  115 -  89 124 149
             300  145 -  89 124 149
             110  130 -  89 124 149
             490  130 -  89 124 149
             300  455 -  89 124 149
             300  490 -  89 124 149
             110  470 -  89 124 149
             490  470 -  89 124 149
        ",
    )
}

// The list at rest: ITEM 0..3 visible, cut only at the scroll edges.
fn check_start() -> Result<()> {
    check_colors(
        r"
             4    4 -  89 124 149
             592    4 -  89 124 149
             252   40 - 255 255 255
             356   44 -  89 124 149
             252   48 - 255 255 255
             308   48 -  89 124 149
             336   52 -  89 124 149
             252   56 - 255 255 255
             196  152 -   0 255   0
             412  152 -   0 255   0
             496  152 -   0 255   0
             104  192 -   0 255   0
             288  192 -   0 255   0
             340  196 -   0 255   0
             344  196 -   0 255   0
             288  200 -   0 255   0
             292  200 -   0 255   0
             340  200 -   0 255   0
             344  200 -   0 255   0
             436  228 -   0 255   0
             592  240 -  89 124 149
             204  244 -   0 255   0
             136  248 -   0 255   0
             288  292 - 255 255   0
             292  292 - 255 255   0
             288  300 - 255 255   0
             292  300 - 255 255   0
             4  304 -  89 124 149
             152  320 - 255 255   0
             440  328 - 255 255   0
             360  344 - 255 255   0
             592  368 -  89 124 149
             288  392 - 255 203   0
             292  392 - 255 203   0
             292  400 - 255 203   0
             104  448 - 255 203   0
             204  448 - 255 203   0
             384  448 - 255 203   0
             488  448 - 255 203   0
             244  540 -  89 124 149
             312  540 - 255 255 255
             336  540 -  89 124 149
             264  548 -  89 124 149
             288  548 -  89 124 149
             312  548 - 255 255 255
             312  556 - 255 255 255
             4  592 -  89 124 149
             560  592 -  89 124 149
        ",
    )
}

// After scrolling up 150: ITEM 1 cut at the top edge, ITEM 4 at the bottom.
fn check_items_cut() -> Result<()> {
    check_colors(
        r"
             4    4 -  89 124 149
             592    4 -  89 124 149
             252   40 - 255 255 255
             356   44 -  89 124 149
             308   48 -  89 124 149
             336   52 -  89 124 149
             252   56 - 255 255 255
             156  152 - 255 255   0
             288  152 - 255 255   0
             292  152 - 255 255   0
             468  152 - 255 255   0
             380  156 - 255 255   0
             4  160 -  89 124 149
             288  240 - 255 203   0
             292  240 - 255 203   0
             404  244 - 255 203   0
             288  252 - 255 203   0
             292  252 - 255 203   0
             104  276 - 255 203   0
             224  304 - 255   0 255
             496  304 - 255   0 255
             288  340 - 255   0 255
             292  340 - 255   0 255
             288  352 - 255   0 255
             292  352 - 255   0 255
             344  352 - 255   0 255
             176  360 - 255   0 255
             432  364 - 255   0 255
             104  372 - 255   0 255
             496  404 -   0 255 255
             376  432 -   0 255 255
             180  440 -   0 255 255
             260  440 -   0 255 255
             272  440 -   0 255 255
             288  440 -   0 255 255
             292  440 -   0 255 255
             104  448 -   0 255 255
             260  448 -   0 255 255
             276  448 -   0 255 255
             460  448 -   0 255 255
             244  540 -  89 124 149
             312  540 - 255 255 255
             336  540 -  89 124 149
             288  548 -  89 124 149
             312  548 - 255 255 255
             312  556 - 255 255 255
             4  592 -  89 124 149
             592  592 -  89 124 149
        ",
    )
}

// The photo enters from the bottom and is cut at the scroll edge.
fn check_photo_cut() -> Result<()> {
    check_colors(
        r"
             4    4 -  89 124 149
             592    4 -  89 124 149
             252   40 - 255 255 255
             356   44 -  89 124 149
             308   48 -  89 124 149
             252   56 - 255 255 255
             104  152 -   0 218 255
             212  152 -   0 218 255
             316  152 -   0 218 255
             400  152 -   0 218 255
             496  192 - 255   0   0
             108  228 - 255   0   0
             288  232 - 255   0   0
             204  236 - 255   0   0
             288  240 - 255   0   0
             292  240 - 255   0   0
             340  240 - 255   0   0
             344  240 - 255   0   0
             444  288 - 255   0   0
             144  296 - 218 170 124
             592  296 -  89 124 149
             288  332 - 218 170 124
             292  332 - 218 170 124
             288  340 - 218 170 124
             292  340 - 218 170 124
             204  360 - 218 170 124
             104  364 - 218 170 124
             428  392 - 221 175 177
             264  396 - 230 190 191
             384  400 - 222 176 176
             496  400 - 217 167 170
             324  408 - 222 176 176
             356  408 - 222 176 176
             168  416 - 228 188 189
             468  432 - 202 152 151
             492  432 - 203 151 153
             104  444 - 232 182 185
             228  444 - 229 192 183
             420  444 - 180 144 122
             440  444 - 198 148 147
             488  448 - 200 150 151
             244  540 -  89 124 149
             312  540 - 255 255 255
             336  540 -  89 124 149
             288  548 -  89 124 149
             312  556 - 255 255 255
             4  592 -  89 124 149
             592  592 -  89 124 149
        ",
    )
}

// All the way down: the photo bottom aligns with the scroll bottom,
// ITEM 5 is cut at the top edge.
fn check_bottom() -> Result<()> {
    check_colors(
        r"
             592    4 -  89 124 149
             252   40 - 255 255 255
             356   44 -  89 124 149
             308   48 -  89 124 149
             252   56 - 255 255 255
             4  148 -  89 124 149
             172  152 - 255   0   0
             440  152 - 255   0   0
             104  176 - 255   0   0
             288  192 - 255   0   0
             288  200 - 255   0   0
             292  200 - 255   0   0
             344  200 - 255   0   0
             496  240 - 255   0   0
             104  252 - 218 170 124
             200  252 - 218 170 124
             408  264 - 218 170 124
             288  292 - 218 170 124
             292  292 - 218 170 124
             288  300 - 218 170 124
             292  300 - 218 170 124
             180  352 - 233 194 199
             420  352 - 222 176 178
             248  356 - 233 193 194
             104  368 - 236 197 202
             340  368 - 223 177 177
             468  392 - 202 152 151
             444  396 - 202 152 151
             132  404 - 226 176 177
             420  404 - 180 144 122
             496  408 - 201 151 152
             444  424 - 167 136 115
             464  424 - 154 126 104
             468  424 - 154 126 104
             472  428 - 154 126 104
             424  432 - 152 124 102
             384  440 - 168 137 116
             464  440 - 171 141 117
             152  444 - 223 178 173
             224  444 - 223 187 175
             104  448 - 221 165 166
             496  448 - 180 152 130
             244  540 -  89 124 149
             312  540 - 255 255 255
             336  540 -  89 124 149
             312  556 - 255 255 255
             4  592 -  89 124 149
             592  592 -  89 124 149
        ",
    )
}

impl ViewTest for ScrollClipTest {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        set_record_probe_count(48);

        let offset = move || from_main(move || view.scroll.get_scroll_content_offset());

        check_start()?;
        check_no_leak()?;

        // Scroll in small steps: ITEM 1 gets cut mid text at the top
        // edge, ITEM 4 at the bottom edge.
        inject_touches("300 300 m");
        for _ in 0..5 {
            inject_scroll(-30);
        }
        assert!((offset() + 150.0).abs() < f32::EPSILON);
        check_items_cut()?;
        check_no_leak()?;

        // The photo enters from the bottom and is cut at the edge.
        for _ in 0..2 {
            inject_scroll(-205);
        }
        assert!((offset() + 560.0).abs() < f32::EPSILON);
        check_photo_cut()?;
        check_no_leak()?;

        // All the way down: the photo bottom aligns with the scroll
        // bottom, ITEM 5 is cut at the top.
        inject_scroll(-1000);
        assert!((offset() + 600.0).abs() < f32::EPSILON);
        check_bottom()?;
        check_no_leak()?;

        Ok(())
    }
}
