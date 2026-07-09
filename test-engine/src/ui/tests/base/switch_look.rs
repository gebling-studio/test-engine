use anyhow::Result;
use hreads::{from_main, wait_for_next_frame};
use refs::Weak;

use crate::{
    self as test_engine,
    ui::{Setup, Switch, ViewData, ViewTest, view_test},
    ui_test::{check_colors, inject_touches},
};

/// Pins the iOS style switch look: rounded gray track with a white round
/// knob on the left when off, green track with the knob on the right
/// when on. Also covers the public `set_on`.
#[view_test]
struct SwitchLook {
    #[init]
    off: Switch,
    on:  Switch,
}

impl Setup for SwitchLook {
    fn setup(mut self: Weak<Self>) {
        self.off.place().t(20).l(20).size(64, 32);
        self.on.place().t(80).l(20).size(64, 32);
        self.on.set_on(true);
    }
}

impl ViewTest for SwitchLook {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        wait_for_next_frame();
        wait_for_next_frame();

        from_main(move || {
            assert!(!view.off.on());
            assert!(view.on.on());
        });

        check_colors(
            r"
             592    4 -  89 124 149
              32   24 - 255 255 255
              60   24 - 233 233 234
              72   24 - 233 233 234
             336   24 -  89 124 149
              76   32 - 233 233 234
              24   36 - 255 255 255
              36   36 - 255 255 255
              48   36 - 255 255 255
              68   36 - 233 233 234
              56   40 - 226 226 227
              80   40 - 233 233 234
              36   48 - 255 255 255
              60   48 - 233 233 234
              72   48 - 233 233 234
              52   84 -  51 194  87
              24   88 -  52 199  89
              40   88 -  52 199  89
              76   88 - 255 255 255
              64   92 - 255 255 255
              32   96 -  52 199  89
              52   96 -  46 180  80
              44  100 -  52 198  88
              72  104 - 255 255 255
              32  108 -  52 199  89
              56  108 -  46 178  79
             300  300 -  89 124 149
             592  300 -  89 124 149
              60  352 -  89 124 149
               4  592 -  89 124 149
             300  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        inject_touches(
            "
            52 36 b
            52 36 e
        ",
        );

        from_main(move || {
            assert!(view.off.on());
        });

        check_colors(
            r"
             592    4 -  89 124 149
              28   24 -  52 199  89
              60   28 - 255 255 255
              76   28 - 255 255 255
              44   32 -  52 198  89
              68   36 - 255 255 255
              76   40 - 255 255 255
              28   44 -  52 199  89
              64   44 - 255 255 255
             336   44 -  89 124 149
              48   48 -  51 195  87
              64   84 - 255 255 255
              24   88 -  52 199  89
              44   88 -  52 199  89
              76   92 - 255 255 255
              36   96 -  52 199  89
              56   96 - 255 255 255
              68   96 - 255 255 255
              48  100 -  49 191  85
              28  104 -  52 199  89
              72  104 - 255 255 255
              44  108 -  52 199  89
              56  108 -  46 178  79
             484  152 -  89 124 149
             300  300 -  89 124 149
             592  300 -  89 124 149
              60  352 -  89 124 149
             444  444 -  89 124 149
             152  500 -  89 124 149
               4  592 -  89 124 149
             300  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        from_main(move || {
            let mut this = view;
            this.off.set_on(false);
        });

        from_main(move || {
            assert!(!view.off.on());
        });

        Ok(())
    }
}
