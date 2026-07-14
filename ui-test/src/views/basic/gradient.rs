use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{
        Anchor::{Left, Top, X},
        BLACK, Button, Container, GREEN, PURPLE, RED, Setup, TURQUOISE, ViewData, ViewSubviews, WHITE, view,
    },
    ui_test::{UITest, check_colors},
};

#[view]
struct Gradient {
    button: Weak<Button>,

    #[init]
    grad_1: Container,
    grad_2: Container,
    grad_3: Container,

    button_container: Container,
}

impl Setup for Gradient {
    fn setup(mut self: Weak<Self>) {
        self.grad_1.set_gradient(RED, GREEN).place().tl(20).size(100, 100);

        self.grad_2.set_gradient(TURQUOISE, PURPLE).set_corner_radius(28);
        self.grad_2.place().t(20).size(100, 200).anchor(Left, self.grad_1, 20);

        self.grad_3.set_gradient(WHITE, BLACK).set_corner_radius(20);
        self.grad_3.place().t(20).size(200, 100).anchor(Left, self.grad_2, 20);

        self.button_container
            .place()
            .same([X], self.grad_1)
            .anchor(Top, self.grad_2, 40)
            .size(280, 100);

        self.button = self.button_container.add_view();

        self.button.place().back();
        self.button.set_text("Button").set_gradient(WHITE, RED).set_corner_radius(40);
    }
}

pub async fn test_gradient() -> Result<()> {
    UITest::start::<Gradient>();

    check_colors(
        r"
             592    4 -  89 124 149
              52   24 - 250  60   0
             288   24 - 250 250 250
              88   36 - 236 113   0
             144   40 -  90 243 255
             204   44 -  98 241 255
             396   44 - 225 225 225
              24   52 - 214 154   0
             316   60 - 203 203 203
             444   72 - 183 183 183
             356   84 - 161 161 161
              80   88 - 152 216   0
             288   96 - 133 133 133
             404  100 - 122 122 122
              28  104 - 110 237   0
             144  108 - 178 197 255
             108  116 -  52 251   0
             236  144 - 207 165 255
             168  180 - 231 123 255
             224  204 - 246  79 255
             184  216 - 253  36 255
             264  264 - 255 250 250
              24  288 - 255 220 220
             180  308 - 255 190 190
             100  312 - 255 183 183
             128  312 - 255 183 183
              64  316 - 255 176 176
             296  328 - 255 152 152
             236  340 - 255 122 122
              48  348 - 255  95  95
             144  352 - 255  77  77
             592  592 -  89 124 149
        ",
    )?;

    Ok(())
}
