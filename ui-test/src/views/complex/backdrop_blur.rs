use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{
        BlurView, Container, ImageView, Label, RED, Setup, ViewData, ViewSubviews, ViewTest, WHITE,
        view_test,
    },
    ui_test::{check_colors, set_record_probe_count},
};

// A frosted header card floating over a busy background: color
// strips, a photo and text. The card is tall enough to cover text and
// the photo top, shows them blurred and tinted, its own title stays
// crisp on top, and the rounded corners cut the blur off.
#[view_test]
struct BackdropBlurTest {
    title: Weak<Label>,

    #[init]
    white:   Container,
    red:     Container,
    photo:   ImageView,
    covered: Label,
    text:    Label,
    header:  BlurView,
}

impl Setup for BackdropBlurTest {
    fn setup(mut self: Weak<Self>) {
        self.white.set_color(WHITE);
        self.white.place().tl(0).size(200, 600);

        self.red.set_color(RED);
        self.red.place().t(0).l(200).size(120, 600);

        self.photo.set_image("cat.png");
        self.photo.place().t(40).l(340).size(220, 160);

        self.covered.set_text("Covered by frost");
        self.covered.set_text_size(32);
        self.covered.place().t(90).l(40).size(400, 60);

        self.text.set_text("Behind the blur");
        self.text.set_text_size(40);
        self.text.place().t(480).l(220).size(360, 60);

        self.header.set_blur_radius(25);
        self.header.set_color(WHITE.with_alpha(0.25));
        self.header.set_corner_radius(20);
        self.header.place().t(12).l(12).r(12).h(160);

        self.title = self.header.add_view();
        self.title.set_text("Frosted header");
        self.title.set_text_size(28);
        self.title.place().lrt(0).h(56);
    }
}

fn check_blurred() -> Result<()> {
    check_colors(
        r"
           4    4 - 255 255 255
         452   16 - 171 176 187
         584   24 - 158 171 184
         212   32 - 255 184 184
         276   36 - 252 139 139
         240   40 - 255 146 146
         292   40 - 245 143 145
         360   40 - 199 186 194
         536   56 - 199 183 187
         416   60 - 216 196 196
         376   80 - 228 204 203
         320   88 - 224 164 169
         184   92 - 250 227 227
         476  104 - 207 186 176
         252  116 - 246 140 140
         400  116 - 228 205 197
         584  116 - 165 172 183
         528  120 - 201 180 175
         364  136 - 227 203 200
         324  148 - 221 166 170
         188  156 - 252 224 224
         272  168 - 252 138 139
         488  176 - 186 154 133
         536  176 - 164 132 107
         376  180 - 237 203 194
         516  184 - 168 138 114
         532  184 - 166 134 111
         428  196 - 213 167 151
         556  196 - 180 152 130
         592  224 -  89 124 149
         240  268 - 255   0   0
           4  300 - 255 255 255
         472  352 -  89 124 149
         336  364 -  89 124 149
         204  392 - 255   0   0
         416  496 -   0   0   0
         276  500 - 255   0   0
         304  504 - 255   0   0
         332  508 -   0   0   0
         376  508 -  89 124 149
         516  508 -   0   0   0
         276  512 - 255   0   0
         288  512 -  13   0   0
         480  512 -  89 124 149
         332  516 -   0   0   0
         444  516 -  89 124 149
           4  592 - 255 255 255
         204  592 - 255   0   0
        ",
    )
}

fn check_restored() -> Result<()> {
    check_colors(
        r"
           4    4 - 255 255 255
         592    4 -  89 124 149
         344   44 - 235 198 205
         512   44 - 223 177 179
         420   48 - 233 193 194
         468   72 - 223 177 177
         380   84 - 229 187 188
         552   84 - 207 153 153
         136  116 - 255 255 255
         188  116 - 255 255 255
         216  116 - 255   0   0
         324  116 -  89 124 149
         156  120 - 255 255 255
         236  120 - 255   0   0
         188  124 - 255 255 255
         260  124 - 255   0   0
         556  124 - 200 150 151
         404  132 - 216 179 160
         492  132 - 195 155 130
         540  144 - 170 142 121
         520  148 - 169 137 116
         540  160 - 153 125 103
         556  160 - 147 119 105
         536  168 - 156 128 106
         524  188 - 171 136 114
         364  196 - 219 161 160
         456  196 - 205 170 150
           4  224 - 255 255 255
         316  252 - 255   0   0
         204  324 - 255   0   0
         440  348 -  89 124 149
         592  364 -  89 124 149
         316  380 - 255   0   0
           4  396 - 255 255 255
         416  496 -   0   0   0
         304  504 - 255   0   0
         444  504 -  89 124 149
         516  504 -   0   0   0
         332  508 -   0   0   0
         516  508 -   0   0   0
         288  512 -  13   0   0
         332  512 -   0   0   0
         376  512 -  89 124 149
         480  512 -  89 124 149
         416  516 -   0   0   0
          88  592 - 255 255 255
         204  592 - 255   0   0
         592  592 -  89 124 149
        ",
    )
}

impl ViewTest for BackdropBlurTest {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        set_record_probe_count(48);

        check_blurred()?;

        from_main(move || {
            view.header.set_hidden(true);
        });

        check_restored()?;

        Ok(())
    }
}

pub async fn test_backdrop_blur() -> Result<()> {
    run_ui_test()
}
