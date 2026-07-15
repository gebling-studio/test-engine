use anyhow::Result;
use test_engine::{
    OnceEvent,
    dispatch::from_main,
    refs::Weak,
    ui::{
        BLACK, Container, GREEN, ImageView, Label, ModalView, RED, Setup, Shadow, Size, UIColor, View,
        ViewData, ViewTest, WHITE, WeakView, view,
    },
    ui_test::{check_colors, set_record_probe_count},
};

// Both modals look like a real dialog: a white rounded card with a
// shadow, a title and two buttons. Only the scrim override differs.
#[view]
struct DimModal {
    event: OnceEvent,

    #[init]
    title: Label,
    yes:   Label,
    no:    Label,
}

impl Setup for DimModal {
    fn setup(self: Weak<Self>) {
        dialog_look(self.weak_view(), self.title, self.yes, self.no, "Are you sure?");
    }
}

impl ModalView for DimModal {
    fn modal_event(&self) -> &OnceEvent<()> {
        &self.event
    }

    fn modal_size() -> Size {
        (260, 160).into()
    }

    fn modal_scrim_color() -> UIColor {
        BLACK.with_alpha(0.4).into()
    }
}

#[view]
struct DefaultModal {
    event: OnceEvent,

    #[init]
    title: Label,
    yes:   Label,
    no:    Label,
}

impl Setup for DefaultModal {
    fn setup(self: Weak<Self>) {
        dialog_look(self.weak_view(), self.title, self.yes, self.no, "No scrim here");
    }
}

impl ModalView for DefaultModal {
    fn modal_event(&self) -> &OnceEvent<()> {
        &self.event
    }

    fn modal_size() -> Size {
        (260, 160).into()
    }
}

fn dialog_look(card: WeakView, title: Weak<Label>, yes: Weak<Label>, no: Weak<Label>, text: &str) {
    card.set_color(WHITE);
    card.set_corner_radius(16);
    card.set_shadow(Shadow::default());

    title.set_text(text);
    title.set_text_size(28);
    title.place().lrt(12).h(60);

    yes.set_text("Yes");
    yes.set_color(GREEN);
    yes.set_corner_radius(8);
    yes.place().size(100, 36).b(16).l(20);

    no.set_text("No");
    no.set_color(RED);
    no.set_corner_radius(8);
    no.place().size(100, 36).b(16).r(20);
}

// The background imitates a real screen: color strips, a photo and
// text. The scrim has to dim all of it while the modal on top stays
// untouched.
#[view]
struct ModalScrimTest {
    #[init]
    white: Container,
    red:   Container,
    photo: ImageView,
    text:  Label,
}

impl Setup for ModalScrimTest {
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

// The opt-in scrim dims everything behind the modal: the strips, the
// photo and the text. The modal itself stays undimmed on top.
fn check_dimmed() -> Result<()> {
    check_colors(
        r"
            4    4 - 203 203 203
            264    4 - 203   0   0
            592    4 -  69  98 118
            344   44 - 187 157 163
            428   48 - 184 152 152
            516   56 - 174 137 137
            556   88 - 162 122 121
            204  104 - 203   0   0
            492  120 - 144 119  96
            316  128 - 203   0   0
            404  132 - 172 142 127
            552  140 - 156 115 116
            556  160 - 116  93  82
            488  176 - 148 122 105
            532  196 - 148 123 104
            348  216 -  86 120 144
            428  220 -  87 121 146
            164  228 - 251 251 251
            284  260 - 255 255 255
            284  264 - 255 255 255
            160  300 - 253 253 253
            420  300 - 255 255 255
            592  324 -  69  98 118
            224  332 -   0 255   0
            228  332 -   0 255   0
            260  332 -   0 255   0
            348  332 - 255   0   0
            4  336 - 203 203 203
            224  336 -   0 255   0
            372  348 - 255   0   0
            228  352 -   0 255   0
            352  352 - 255   0   0
            192  356 -   0 255   0
            284  360 -   0 255   0
            436  376 -  88 122 147
            188  392 - 253 253 253
            332  392 -  88 123 148
            520  496 -   0   0   0
            256  500 - 203   0   0
            536  504 -  69  98 118
            356  508 -  69  98 118
            388  508 -  69  98 118
            416  512 -   0   0   0
            488  512 -  69  98 118
            328  516 -   0   0   0
            520  516 -   0   0   0
            4  592 - 203 203 203
            592  592 -  69  98 118
        ",
    )
}

// Hiding the modal removes the scrim with it, everything is back to
// full brightness.
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

// The default scrim is transparent, a modal without the override
// leaves the background untouched.
fn check_undimmed_modal() -> Result<()> {
    check_colors(
        r"
            4    4 - 255 255 255
            316    4 - 255   0   0
            592    4 -  89 124 149
            388   64 - 229 189 190
            484   64 - 223 177 177
            556   80 - 207 157 156
            204  108 - 255   0   0
            344  120 - 229 183 186
            500  120 - 183 150 119
            520  148 - 169 137 116
            404  160 - 222 190 169
            540  160 - 153 125 103
            556  160 - 147 119 105
            536  172 - 158 128 104
            476  188 - 195 163 142
            516  188 - 166 136 112
            344  196 - 221 167 167
            432  196 - 208 161 143
            556  196 - 180 152 130
            4  232 - 255 255 255
            240  260 - 255 255 255
            276  260 - 255 255 255
            240  264 - 255 255 255
            276  264 - 255 255 255
            484  308 -  89 124 149
            224  332 -   0 255   0
            228  332 -   0 255   0
            348  332 - 255   0   0
            224  336 -   0 255   0
            288  340 -   0 255   0
            372  348 - 255   0   0
            228  352 -   0 255   0
            352  352 - 255   0   0
            592  352 -  89 124 149
            192  356 -   0 255   0
            260  360 -   0 255   0
            20  412 - 255 255 255
            364  496 -   0   0   0
            256  500 - 255   0   0
            328  504 -   0   0   0
            536  504 -  89 124 149
            356  508 -  89 124 149
            388  512 -  89 124 149
            416  512 -   0   0   0
            488  512 -  89 124 149
            328  516 -   0   0   0
            4  592 - 255 255 255
            204  592 - 255   0   0
        ",
    )
}

impl ViewTest for ModalScrimTest {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        set_record_probe_count(48);

        let modal = from_main(DimModal::prepare_modally);

        check_dimmed()?;

        modal.hide_modal(());

        check_restored()?;

        let modal = from_main(DefaultModal::prepare_modally);

        check_undimmed_modal()?;

        modal.hide_modal(());

        Ok(())
    }
}
