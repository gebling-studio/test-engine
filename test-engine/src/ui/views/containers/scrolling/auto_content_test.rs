use anyhow::Result;
use hreads::{from_main, wait_for_next_frame};
use refs::Weak;

use crate::{
    self as test_engine,
    gm::{
        LossyConvert,
        color::{BLUE, GREEN, YELLOW},
    },
    ui::{Container, ScrollView, Setup, ViewData, ViewSubviews, ViewTest, view_test},
    ui_test::{check_colors, inject_scroll, inject_touches},
};

/// Covers the automatic content size: a scroll view whose content size
/// was never set gets its width from the viewport and its height from
/// the lowest subview edge, hidden subviews not counting.
#[view_test]
struct AutoContentTest {
    #[init]
    scroll: ScrollView,
}

impl Setup for AutoContentTest {
    fn setup(self: Weak<Self>) {
        self.scroll.place().tl(0).size(300, 300);

        // The last row gets its own color, so the bottom clamped state
        // is visually distinct from the top in a repeating stripe
        // pattern.
        for i in 0..10_u32 {
            let row = self.scroll.add_view::<Container>();
            row.set_color(if i == 9 {
                GREEN
            } else if i.is_multiple_of(2) {
                BLUE
            } else {
                YELLOW
            });
            row.place().t(50.0 * i.lossy_convert()).lr(0).h(50);
        }

        let hidden = self.scroll.add_view::<Container>();
        hidden.place().t(500).lr(0).h(300);
        hidden.set_hidden(true);
    }
}

impl ViewTest for AutoContentTest {
    fn perform_test(mut view: Weak<Self>) -> Result<()> {
        wait_for_next_frame();
        wait_for_next_frame();

        from_main(move || {
            assert_eq!(view.scroll.content.content_size, (300, 500).into());
            assert!(view.scroll.get_scroll_content_offset().abs() < f32::EPSILON);
        });

        check_colors(
            r"
               4    4 -   0   0 231
             220    4 -   0   0 231
             592    4 -  89 124 149
             296    8 -   0   0 231
             444    8 -  89 124 149
             148   12 -   0   0 231
              76   16 -   0   0 231
              20   76 - 255 255   0
             160   80 - 255 255   0
             232   84 - 255 255   0
              92   88 - 255 255   0
              16  148 -   0   0 231
             480  152 -  89 124 149
              84  156 - 255 255   0
             160  156 - 255 255   0
             296  164 - 255 255   0
             228  168 - 255 255   0
               4  220 -   0   0 231
             168  224 -   0   0 231
              88  228 -   0   0 231
             296  228 -   0   0 231
             236  236 -   0   0 231
              12  296 - 255 255   0
             164  296 - 255 255   0
             592  296 -  89 124 149
             304  300 -  89 124 149
             488  444 -  89 124 149
               4  456 -  89 124 149
             200  460 -  89 124 149
             340  552 -  89 124 149
              88  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        // Way past the content: the offset clamps so the last row's
        // bottom lands exactly on the viewport bottom.
        inject_touches("150 150 m");
        inject_scroll(-1000);

        from_main(move || {
            assert!((view.scroll.get_scroll_content_offset() + 200.0).abs() < f32::EPSILON);
        });

        check_colors(
            r"
               4    4 -   0   0 231
             136    4 -   0   0 231
             440    4 -  89 124 149
             204    8 -   0   0 231
              68   16 -   0   0 231
             256   52 - 255 255   0
             164   56 - 255 255   0
             104   72 - 255 255   0
              32   92 - 255 255   0
             188  108 -   0   0 231
             592  116 -  89 124 149
             296  124 -   0   0 231
             108  148 -   0   0 231
               4  184 - 255 255   0
             236  184 - 255 255   0
             296  192 - 255 255   0
             164  196 - 255 255   0
              84  236 -   0   0 231
              36  252 -   0 255   0
             132  252 -   0 255   0
             180  252 -   0 255   0
             280  252 -   0 255   0
             228  268 -   0 255   0
              12  296 -   0 255   0
              76  296 -   0 255   0
             152  296 -   0 255   0
             304  300 -  89 124 149
             540  352 -  89 124 149
             404  504 -  89 124 149
               4  524 -  89 124 149
             220  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        // A manual height wins over the automatic one and stays.
        from_main(move || {
            view.scroll.set_content_height(1000);
        });

        wait_for_next_frame();
        wait_for_next_frame();

        from_main(move || {
            assert!((view.scroll.content.content_size.height - 1000.0).abs() < f32::EPSILON);
        });

        Ok(())
    }
}
