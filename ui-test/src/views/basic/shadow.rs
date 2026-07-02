use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{
        BLACK, BLUE, Container, CornerRadii, GREEN, LIGHTER_GRAY, RED, Setup, Shadow, ViewData, ViewTest,
        WHITE, YELLOW, view_test,
    },
    ui_test::{check_colors, set_record_probe_count},
};

#[view_test]
struct ShadowTest {
    #[init]
    under_ghost: Container,
    plain:       Container,
    offset_card: Container,
    round:       Container,
    red_glow:    Container,
    ghost:       Container,
}

impl Setup for ShadowTest {
    fn setup(self: Weak<Self>) {
        self.plain.set_color(GREEN);
        self.plain.place().tl(60).size(120, 80);
        self.plain.set_shadow(Shadow {
            offset: (0, 0).into(),
            radius: 30.0,
            color:  BLACK,
        });

        self.offset_card.set_color(BLUE);
        self.offset_card.place().t(60).l(300).size(120, 80);
        self.offset_card.set_shadow(Shadow {
            offset: (25, 25).into(),
            radius: 20.0,
            color:  BLACK,
        });

        self.round.set_color(YELLOW);
        self.round.place().t(250).l(60).size(100, 100);
        self.round.set_corner_radii(CornerRadii::all(40));
        self.round.set_shadow(Shadow {
            offset: (0, 0).into(),
            radius: 25.0,
            color:  BLACK.with_alpha(0.8),
        });

        self.red_glow.set_color(WHITE);
        self.red_glow.place().t(250).l(300).size(100, 100);
        self.red_glow.set_shadow(Shadow {
            offset: (0, 0).into(),
            radius: 30.0,
            color:  RED,
        });

        // A hidden view must not cast a shadow. The backdrop catches
        // probes where the shadow would fall, pinning its absence.
        self.under_ghost.set_color(LIGHTER_GRAY);
        self.under_ghost.place().t(430).l(40).size(140, 140);

        self.ghost.set_color(GREEN);
        self.ghost.place().t(450).l(60).size(100, 100);
        self.ghost.set_shadow(Shadow {
            offset: (0, 0).into(),
            radius: 30.0,
            color:  BLACK,
        });
        self.ghost.set_hidden(true);
    }
}

impl ViewTest for ShadowTest {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        set_record_probe_count(64);

        check_colors(
            r"
             592    4 -  89 124 149
              64   64 -   0 255   0
             152   64 -   0 255   0
             304   64 -   0   0 231
             336   64 -   0   0 231
             416   64 -   0   0 231
             108   80 -   0 255   0
             360   84 -   0   0 231
             448   84 -  71 100 121
             184   96 -  70  98 119
              88  104 -   0 255   0
             116  108 -   0 255   0
             336  108 -   0   0 231
             388  108 -   0   0 231
             424  108 -   0   0   0
              56  112 -  68  96 117
             144  112 -   0 255   0
             304  116 -   0   0 231
             100  136 -   0 255   0
             424  136 -   0   0   0
             184  144 -  72 102 123
             344  144 -   0   0   1
             356  144 -   0   0   0
             368  144 -   0   0   0
             396  144 -   0   0   0
             412  144 -   0   0   0
             448  152 -  71 100 120
             592  224 -  89 124 149
             332  236 - 141 113 136
             372  240 - 158 107 129
             412  240 - 131 115 139
             312  244 - 175 100 121
             116  248 -  71 100 121
              80  252 -  72 102 123
             296  260 - 183  96 117
             148  264 - 255 255   0
             404  272 - 179  98 119
             296  284 - 183  96 117
              72  288 - 255 255   0
             108  296 - 255 255   0
             280  300 - 115 119 143
             412  300 - 145 111 134
             156  304 - 255 255   0
              64  324 - 255 255   0
             296  332 - 183  96 117
             412  332 - 145 111 134
             140  340 - 255 255   0
             100  348 - 255 255   0
             320  356 - 171 102 123
             388  356 - 171 102 123
             288  360 - 132 115 139
             352  364 - 137 114 137
             592  388 -  89 124 149
             112  432 - 243 243 243
              44  444 - 243 243 243
             176  456 - 243 243 243
             300  484 -  89 124 149
             476  488 -  89 124 149
              44  504 - 243 243 243
             104  504 - 243 243 243
              44  568 - 243 243 243
             160  568 - 243 243 243
             360  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        Ok(())
    }
}

pub async fn test_shadow() -> Result<()> {
    run_ui_test()
}
