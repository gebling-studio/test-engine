#![allow(clippy::float_cmp)]

use anyhow::Result;
use hreads::from_main;
use refs::Weak;

use crate::{
    self as test_engine,
    gm::color::{BROWN, Color, GRAY_BLUE, GREEN, LIGHT_BLUE, ORANGE, PURPLE, RED, TURQUOISE, WHITE, YELLOW},
    ui::{
        ImageView, Label, ScrollView, Setup, ViewData, ViewFrame, ViewSubviews, ViewTest, view_test,
    },
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

impl ViewTest for ScrollClipTest {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        set_record_probe_count(48);

        let offset = move || from_main(move || view.scroll.get_scroll_content_offset());

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
             592  144 -  89 124 149
               4  148 -  89 124 149
             180  152 -   0 255   0
             248  152 -   0 255   0
             444  152 -   0 255   0
             104  172 -   0 255   0
             340  196 -   0 255   0
             344  196 -   0 255   0
             340  200 -   0 255   0
             344  200 -   0 255   0
             292  204 -   0 255   0
             200  248 -   0 255   0
             412  248 -   0 255   0
             496  248 -   0 255   0
             104  252 - 255 255   0
             288  292 - 255 255   0
             288  300 - 255 255   0
             292  300 - 255 255   0
             496  336 - 255 255   0
               4  340 -  89 124 149
             412  344 - 255 255   0
             592  344 -  89 124 149
             184  356 - 255 203   0
             288  392 - 255 203   0
             288  400 - 255 203   0
             104  432 - 255 203   0
             496  440 - 255 203   0
             204  448 - 255 203   0
             388  448 - 255 203   0
             244  540 -  89 124 149
             312  540 - 255 255 255
             336  540 -  89 124 149
             312  544 - 255 255 255
             356  544 -  89 124 149
             264  548 -  89 124 149
             288  548 -  89 124 149
             312  548 - 255 255 255
             312  556 - 255 255 255
               4  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;
        check_no_leak()?;

        // Scroll in small steps: ITEM 1 gets cut mid text at the top
        // edge, ITEM 4 at the bottom edge.
        inject_touches("300 300 m");
        for _ in 0..5 {
            inject_scroll(-30);
        }
        assert_eq!(offset(), -150.0);

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
        )?;
        check_no_leak()?;

        // The photo enters from the bottom and is cut at the edge.
        for _ in 0..2 {
            inject_scroll(-205);
        }
        assert_eq!(offset(), -560.0);

        check_colors(
            r"
               4    4 -  89 124 149
             592    4 -  89 124 149
             252   40 - 255 255 255
             356   44 -  89 124 149
             308   48 -  89 124 149
             252   56 - 255 255 255
             144  152 -   0 218 255
             392  152 -   0 218 255
             496  152 -   0 218 255
             212  156 -   0 218 255
             304  156 -   0 218 255
             444  204 - 255   0   0
             104  208 - 255   0   0
             340  240 - 255   0   0
             344  240 - 255   0   0
             200  244 - 255   0   0
             292  244 - 255   0   0
             464  276 - 255   0   0
             104  280 - 255   0   0
             228  304 - 218 170 124
             380  316 - 218 170 124
             160  324 - 218 170 124
             288  332 - 218 170 124
             288  340 - 218 170 124
             292  340 - 218 170 124
             104  368 - 218 170 124
             428  392 - 221 175 177
             264  396 - 230 190 191
             496  396 - 217 167 170
             168  400 - 231 192 193
             384  400 - 222 176 176
             324  408 - 222 176 176
             356  408 - 222 176 176
             468  432 - 202 152 151
             104  444 - 232 182 185
             228  444 - 229 192 183
             420  444 - 180 144 122
             440  444 - 198 148 147
             472  448 - 200 148 150
             492  448 - 201 151 152
             244  540 -  89 124 149
             312  540 - 255 255 255
             336  540 -  89 124 149
             288  548 -  89 124 149
             312  556 - 255 255 255
             460  588 -  89 124 149
               4  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;
        check_no_leak()?;

        // All the way down: the photo bottom aligns with the scroll
        // bottom, ITEM 5 is cut at the top.
        inject_scroll(-1000);
        assert_eq!(offset(), -600.0);

        check_colors(
            r"
               4    4 -  89 124 149
             512    4 -  89 124 149
             252   40 - 255 255 255
             356   44 -  89 124 149
             308   48 -  89 124 149
             252   56 - 255 255 255
             592  140 -  89 124 149
             204  152 - 255   0   0
             424  152 - 255   0   0
             104  192 - 255   0   0
             340  200 - 255   0   0
             344  200 - 255   0   0
             292  204 - 255   0   0
             480  224 - 255   0   0
             200  252 - 218 170 124
             400  268 - 218 170 124
             288  292 - 218 170 124
             288  300 - 218 170 124
             292  300 - 218 170 124
             128  320 - 218 170 124
             188  352 - 234 195 200
             428  352 - 221 175 177
             280  364 - 225 185 185
             344  364 - 225 179 179
             472  396 - 201 151 150
             440  400 - 201 151 150
             132  404 - 226 176 177
             228  404 - 229 192 183
             420  404 - 180 144 122
             496  412 - 199 147 149
             444  424 - 167 136 115
             464  424 - 154 126 104
             468  424 - 154 126 104
             472  428 - 154 126 104
             424  432 - 152 124 102
             384  440 - 168 137 116
             464  440 - 171 141 117
             152  444 - 223 178 173
             104  448 - 221 165 166
             228  448 - 221 183 172
             496  448 - 180 152 130
             244  540 -  89 124 149
             312  540 - 255 255 255
             336  540 -  89 124 149
             288  548 -  89 124 149
             312  556 - 255 255 255
               4  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;
        check_no_leak()?;

        Ok(())
    }
}
