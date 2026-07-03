use anyhow::Result;
use crate::gm::color::{BLUE, GREEN, RED, YELLOW};
use hreads::from_main;
use refs::Weak;
use crate::ui::{Anchor::Left, Container, ImageView, Setup, ViewData, ViewTest, view_test};

use crate::ui_test::check_colors;

#[view_test]
struct Outline {
    #[init]
    square: Container,
    image:  ImageView,
    wide:   Container,
}

impl Setup for Outline {
    fn setup(self: Weak<Self>) {
        self.square.set_color(BLUE).set_border_width(10).set_border_color(RED);
        self.square.place().size(100, 100).tl(50);

        self.image.set_image("cat.png").set_border_width(5).set_border_color(GREEN);
        self.image.place().size(100, 200).t(50).anchor(Left, self.square, 20);

        self.wide.set_color(YELLOW).set_border_width(25).set_border_color(BLUE);
        self.wide.place().size(200, 100).t(50).anchor(Left, self.image, 20);
    }
}

impl ViewTest for Outline {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        check_colors(
            r"
             120   52 - 255   0   0
             260   56 - 219 173 175
              64   64 -   0   0 231
             176   68 - 234 195 200
             340   76 - 255 255   0
             460   76 - 255 255   0
             220   84 - 226 180 182
             400   88 - 255 255   0
              52  104 - 255   0   0
             100  112 -   0   0 231
             172  112 -   0 255   0
             268  116 -   0 255   0
             320  120 - 255 255   0
             216  132 - 204 178 153
              52  148 - 255   0   0
             148  148 - 255   0   0
             376  148 -   0   0 231
             488  148 -   0   0 231
             256  156 - 200 150 149
             240  172 - 198 162 138
             256  172 - 167 139 117
             268  180 -   0 255   0
             188  188 - 233 205 194
             256  212 - 160 132 108
             216  220 - 209 172 153
             236  220 - 187 155 134
             176  240 - 221 163 162
             252  240 - 168 136 113
             300  300 -  89 124 149
               4  592 -  89 124 149
             300  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        from_main(move || {
            view.square.set_corner_radius(15);
            view.image.set_corner_radius(25);
            view.wide.set_corner_radius(40);
        });

        check_colors(
            r"
             116   52 - 255   0   0
             232   52 -   0 255   0
              56   56 - 255   0   0
             480   68 -   0   0 231
             172   72 -   0 255   0
             388   76 - 255 255   0
             316   92 - 255 255   0
             260   96 - 207 156 155
             436  100 - 255 255   0
             376  120 - 255 255   0
             176  124 - 228 188 189
             216  132 - 204 178 153
             476  136 -   0   0 231
              84  148 - 255   0   0
             344  148 -   0   0 231
             408  148 -   0   0 231
             260  156 - 201 149 151
             256  160 - 199 149 148
             188  172 - 238 210 199
             240  172 - 198 162 138
             260  180 - 170 142 121
             256  212 - 160 132 108
             176  220 - 226 172 170
             236  220 - 187 155 134
             252  228 - 169 139 115
             264  236 -   0 255   0
             256  244 -   0 255   0
             208  248 -   0 255   0
             592  348 -  89 124 149
             300  540 -  89 124 149
               4  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        // crate::ui_test::record_ui_test();

        Ok(())
    }
}
