use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{
        BlurView, Container, ImageView, Label, RED, Setup, ViewData, ViewSubviews, ViewTest, WHITE, view_test,
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
            460   16 - 171 176 187
            576   16 - 158 171 184
            212   32 - 255 184 184
            216   32 - 255 177 177
            276   36 - 252 139 139
            240   40 - 255 146 146
            292   40 - 245 143 145
            360   40 - 199 186 194
            420   60 - 215 196 195
            480   76 - 214 191 187
            544   80 - 199 181 183
            368   84 - 227 203 203
            320   88 - 223 164 169
            184   92 - 249 227 227
            252  116 - 245 140 140
            400  116 - 228 205 197
            520  124 - 202 180 173
            584  132 - 165 172 182
            472  136 - 207 186 174
            324  148 - 221 166 170
            372  152 - 229 205 201
            188  156 - 251 223 223
            488  176 - 186 154 133
            536  176 - 164 132 107
            532  180 - 168 136 111
            532  184 - 166 134 111
            364  196 - 219 161 160
            428  196 - 213 167 151
            556  196 - 180 152 130
            240  268 - 255   0   0
            592  268 -  89 124 149
            4  300 - 255 255 255
            456  356 -  89 124 149
            316  360 - 255   0   0
            204  392 - 255   0   0
            592  404 -  89 124 149
            416  496 -   0   0   0
            276  500 - 255   0   0
            304  504 - 255   0   0
            376  508 -  89 124 149
            480  508 -  89 124 149
            276  512 - 255   0   0
            288  512 -   1   0   0
            332  512 -   0   0   0
            444  516 -  89 124 149
            516  516 -   0   0   0
            4  592 - 255 255 255
        ",
    )
}

fn check_restored() -> Result<()> {
    check_colors(
        r"
            4    4 - 255 255 255
            236    4 - 255   0   0
            592    4 -  89 124 149
            344   44 - 235 198 205
            512   44 - 223 177 179
            420   48 - 233 193 194
            468   72 - 223 177 177
            380   84 - 229 187 188
            552   84 - 207 153 153
            188  116 - 255 255 255
            216  116 - 255   0   0
            324  116 -  89 124 149
            132  120 - 255 255 255
            156  120 - 255 255 255
            236  120 - 255   0   0
            500  120 - 183 150 119
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
            488  176 - 186 154 133
            524  188 - 171 136 114
            364  196 - 219 161 160
            456  196 - 205 170 150
            4  232 - 255 255 255
            316  252 - 255   0   0
            204  324 - 255   0   0
            440  348 -  89 124 149
            592  364 -  89 124 149
            316  384 - 255   0   0
            4  400 - 255 255 255
            416  496 -   0   0   0
            304  504 - 255   0   0
            444  504 -  89 124 149
            516  504 -   0   0   0
            288  512 -   1   0   0
            332  512 -   0   0   0
            376  512 -  89 124 149
            480  512 -  89 124 149
            416  516 -   0   0   0
            92  592 - 255 255 255
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
