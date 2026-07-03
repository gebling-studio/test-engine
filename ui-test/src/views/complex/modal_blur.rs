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
         188    4 - 203 171 171
         332    4 - 134  85 101
         592    4 -  71  98 118
         408   32 - 127 120 130
         508   44 - 138 120 127
         316   52 - 172  79  89
         260   56 - 203  16  17
         464   88 - 155 128 117
         556   88 - 127 110 117
         400   96 - 172 145 137
         332  100 - 167 110 116
         268  120 - 202  13  15
         204  124 - 203 139 139
         328  140 - 168 101 107
         432  152 - 162 135 120
         560  180 - 102 103 109
         248  192 - 203  34  34
         328  204 - 151  87  98
           4  216 - 203 203 203
         428  236 - 255 255 255
         208  264 - 255 255 255
         308  264 - 255 255 255
         376  264 - 255 255 255
         268  300 - 255 255 255
         428  308 - 255 255 255
         220  332 -   0 255   0
         224  332 -   0 255   0
         312  332 - 255   0   0
         348  332 - 255   0   0
         224  336 -   0 255   0
         408  340 - 255   0   0
         372  348 - 255   0   0
         352  352 - 255   0   0
         196  360 -   0 255   0
         268  360 -   0 255   0
         420  380 -  58  84 101
         300  384 - 167  40  50
         220  416 - 203  98  98
         592  416 -  69  98 118
         284  456 - 195  25  31
           4  488 - 203 203 203
         204  508 - 203 139 139
         328  540 - 134  76  92
         188  572 - 203 171 171
         248  592 - 203  34  34
         304  592 - 180  50  62
         592  592 -  69  98 118
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
         364  496 -   0   0   0
         252  500 - 255   0   0
         256  500 - 255   0   0
         260  500 - 255   0   0
         364  504 -   0   0   0
         408  504 -   0   0   0
         536  504 -  89 124 149
         328  508 -   2   2   4
         356  508 -  89 124 149
         512  508 -  89 124 149
         260  512 - 255   0   0
         388  512 -  89 124 149
         488  512 -  89 124 149
         280  516 - 255   0   0
         364  516 -   2   2   4
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
