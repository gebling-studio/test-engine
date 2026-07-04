use anyhow::Result;
use test_engine::{
    OnceEvent,
    dispatch::from_main,
    refs::Weak,
    ui::{
        BLACK, Container, GREEN, ImageView, Label, ModalView, RED, Setup, Shadow, Size, UIColor, ViewData,
        ViewTest, WHITE, view, view_test,
    },
    ui_test::{check_colors, set_record_probe_count},
};

// A real looking dialog over a frosted backdrop: the modal blurs the
// whole scene behind it and dims it with the scrim tint, while the
// dialog itself stays crisp.
#[view]
struct BlurModal {
    event: OnceEvent,

    #[init]
    title: Label,
    yes:   Label,
    no:    Label,
}

impl Setup for BlurModal {
    fn setup(self: Weak<Self>) {
        self.set_color(WHITE);
        self.set_corner_radius(16);
        self.set_shadow(Shadow::default());

        self.title.set_text("Blurred behind?");
        self.title.set_text_size(28);
        self.title.place().lrt(12).h(60);

        self.yes.set_text("Yes");
        self.yes.set_color(GREEN);
        self.yes.set_corner_radius(8);
        self.yes.place().size(100, 36).b(16).l(20);

        self.no.set_text("No");
        self.no.set_color(RED);
        self.no.set_corner_radius(8);
        self.no.place().size(100, 36).b(16).r(20);
    }
}

impl ModalView for BlurModal {
    fn modal_event(&self) -> &OnceEvent<()> {
        &self.event
    }

    fn modal_size() -> Size {
        (260, 160).into()
    }

    fn modal_scrim_color() -> UIColor {
        BLACK.with_alpha(0.4).into()
    }

    fn modal_blur() -> f32 {
        25.0
    }
}

// The same busy background as the scrim test: strips, a photo and
// text, so the blur and tint have every pipeline behind them.
#[view_test]
struct ModalBlurTest {
    #[init]
    white: Container,
    red:   Container,
    photo: ImageView,
    text:  Label,
}

impl Setup for ModalBlurTest {
    fn setup(self: Weak<Self>) {
        self.white.set_color(WHITE);
        self.white.place().tl(0).size(200, 600);

        self.red.set_color(RED);
        self.red.place().t(0).l(200).size(120, 600);

        self.photo.set_image("cat.png");
        self.photo.place().t(40).l(340).size(220, 160);

        self.text.set_text("Behind the modal");
        self.text.set_text_size(40);
        self.text.place().t(480).l(220).size(360, 60);
    }
}

fn check_blurred() -> Result<()> {
    check_colors(
        r"
            4    4 - 203 203 203
            212    4 - 203 119 119
            288    4 - 195  30  37
            592    4 -  71  98 118
            328   32 - 151  90 102
            388   32 - 127 121 131
            440   60 - 156 133 132
            188   72 - 203 171 171
            348   76 - 165 130 134
            284   80 - 198  29  34
            532   80 - 148 118 117
            224  124 - 203  88  88
            332  128 - 167 109 114
            284  144 - 198  27  33
            432  144 - 162 134 119
            500  144 - 144 117 101
            352  184 - 153 121 125
            220  188 - 203  98  98
            540  200 - 101 103 109
            4  216 - 203 203 203
            420  224 - 255 255 255
            308  260 - 255 255 255
            208  264 - 255 255 255
            284  264 - 255 255 255
            376  272 - 255 255 255
            428  296 - 255 255 255
            224  332 -   0 255   0
            228  332 -   0 255   0
            348  332 - 255   0   0
            288  340 -   0 255   0
            372  348 - 255   0   0
            228  352 -   0 255   0
            352  352 - 255   0   0
            408  352 - 255   0   0
            192  356 -   0 255   0
            320  356 - 255   0   0
            592  368 -  69  98 118
            300  384 - 167  40  50
            188  424 - 203 171 171
            256  440 - 202  21  21
            328  452 - 138  78  94
            216  500 - 201 109 109
            292  524 - 180  32  41
            592  544 -  69  98 118
            188  556 - 203 171 171
            4  592 - 203 203 203
            236  592 - 203  58  58
            332  592 - 131  82  99
        ",
    )
}

fn check_restored() -> Result<()> {
    check_colors(
        r"
            4    4 - 255 255 255
            204    4 - 255   0   0
            592    4 -  89 124 149
            424   44 - 232 192 193
            512   48 - 220 174 176
            344   60 - 236 200 204
            468   72 - 223 177 177
            552   84 - 207 153 153
            344  112 - 229 187 189
            500  120 - 183 150 119
            556  124 - 200 150 151
            388  132 - 235 205 195
            492  132 - 195 155 130
            540  144 - 170 142 121
            520  148 - 169 137 116
            80  160 - 255 255 255
            540  160 - 153 125 103
            556  160 - 147 119 105
            548  164 - 155 127 103
            300  168 - 255   0   0
            536  168 - 156 128 106
            488  176 - 186 154 133
            524  188 - 171 136 114
            364  196 - 219 161 160
            456  196 - 205 170 150
            204  280 - 255   0   0
            4  316 - 255 255 255
            340  336 -  89 124 149
            592  344 -  89 124 149
            464  380 -  89 124 149
            204  396 - 255   0   0
            68  456 - 255 255 255
            364  496 -   0   0   0
            256  500 - 255   0   0
            260  500 - 255   0   0
            328  504 -   0   0   0
            536  504 -  89 124 149
            356  508 -  89 124 149
            512  508 -  89 124 149
            256  512 - 255   0   0
            260  512 - 255   0   0
            388  512 -  89 124 149
            416  512 -   0   0   0
            488  512 -  89 124 149
            328  516 -   0   0   0
            364  516 -   0   0   0
            4  592 - 255 255 255
            592  592 -  89 124 149
        ",
    )
}

impl ViewTest for ModalBlurTest {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        set_record_probe_count(48);

        let modal = from_main(BlurModal::prepare_modally);

        check_blurred()?;

        modal.hide_modal(());

        check_restored()?;

        Ok(())
    }
}

pub async fn test_modal_blur() -> Result<()> {
    run_ui_test()
}
