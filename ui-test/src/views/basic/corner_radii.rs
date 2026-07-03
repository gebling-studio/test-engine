use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{
        BLUE, BROWN, Container, CornerRadii, GRAY, GREEN, ImageView, ORANGE, PURPLE, RED, Setup, TURQUOISE,
        ViewData, ViewTest, YELLOW, view_test,
    },
    ui_test::{check_colors, set_record_probe_count},
};

// Each rounded view sits on a backdrop of a unique color, so the cut
// corners expose that color and the recorder pins probes there. The
// backdrop fields come first, so they stay behind their views.
#[view_test]
struct CornerRadiiTest {
    #[init]
    under_top:    Container,
    under_mixed:  Container,
    under_border: Container,
    under_image:  Container,
    under_grad:   Container,
    top_round:    Container,
    mixed:        Container,
    bordered:     Container,
    image:        ImageView,
    grad:         Container,
}

impl Setup for CornerRadiiTest {
    fn setup(self: Weak<Self>) {
        for (backdrop, color, x, y) in [
            (self.under_top, ORANGE, 20, 20),
            (self.under_mixed, PURPLE, 180, 20),
            (self.under_border, GRAY, 340, 20),
            (self.under_image, TURQUOISE, 20, 180),
            (self.under_grad, BROWN, 180, 180),
        ] {
            backdrop.set_color(color);
            backdrop.place().t(y).l(x).size(140, 140);
        }

        self.top_round.set_color(BLUE);
        self.top_round.place().tl(20).size(140, 140);
        self.top_round.set_corner_radii(CornerRadii::top(60));

        self.mixed.set_color(YELLOW);
        self.mixed.place().t(20).l(180).size(140, 140);
        self.mixed.set_corner_radii(CornerRadii {
            top_left: 60.0,
            bottom_right: 60.0,
            ..CornerRadii::default()
        });

        self.bordered.set_color(GREEN).set_border_color(RED).set_border_width(6);
        self.bordered.place().t(20).l(340).size(140, 140);
        self.bordered.set_corner_radii(CornerRadii::bottom(50));

        self.image.set_image("cat.png");
        self.image.place().t(180).l(20).size(140, 140);
        self.image.set_corner_radii(CornerRadii::top(60));

        self.grad.set_gradient(RED, BLUE);
        self.grad.place().t(180).l(180).size(140, 140);
        self.grad.set_corner_radii(CornerRadii::bottom(60));
    }
}

impl ViewTest for CornerRadiiTest {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        set_record_probe_count(96);

        check_colors(
            r"
              24   24 - 255 203   0
             108   24 -   0   0 231
             188   24 - 255   0 255
             272   24 - 255 255   0
             364   24 - 255   0   0
             468   24 - 255   0   0
             416   28 -   0 255   0
             184   52 - 255   0 255
             384   60 -   0 255   0
              68   64 -   0   0 231
             432   64 -   0 255   0
             344   72 - 255   0   0
             292   76 - 255 255   0
             472   76 -   0 255   0
             184   92 - 255 255   0
             408   96 -   0 255   0
             344  100 - 255   0   0
              28  104 -   0   0 231
             108  104 -   0   0 231
             248  108 - 255 255   0
             316  124 - 255   0 255
             348  128 - 255   0   0
             472  128 - 255   0   0
             356  144 - 255   0   0
             348  148 - 188 188 188
             468  148 - 188 188 188
             292  152 - 255   0 255
             372  152 - 255   0   0
             448  152 - 255   0   0
             316  156 - 255   0 255
             408  156 - 255   0   0
              24  184 -   0 255 255
              52  184 -   0 255 255
              88  184 - 224 184 185
             116  184 - 221 179 180
             156  184 -   0 255 255
             184  188 - 248   0  62
             232  188 - 248   0  62
             260  188 - 248   0  62
             300  188 - 248   0  62
              36  196 -   0 255 255
             136  200 - 208 160 158
             208  204 - 234   0 105
             108  208 - 222 176 176
             156  208 -   0 255 255
             276  208 - 231   0 112
              28  216 - 234 194 195
             316  216 - 223   0 126
             184  220 - 219   0 132
             244  220 - 219   0 132
             268  228 - 211   0 144
             156  232 - 203 153 152
             212  232 - 207   0 149
             292  236 - 203   0 154
              48  240 - 241 214 203
             108  244 -  14   4   3
             188  244 - 194   0 163
             144  252 - 201 151 152
              28  256 - 226 176 177
             236  256 - 179   0 177
             268  256 - 179   0 177
             316  260 - 174   0 181
              52  264 - 233 203 193
             292  264 - 169   0 185
             156  268 - 196 146 147
             196  268 - 163   0 188
              96  272 - 200 164 140
              36  276 - 240 211 203
             148  276 - 161 133 111
             252  276 - 151   0 196
             276  284 - 138   0 203
             136  288 - 161 131 107
             228  288 - 130   0 206
             200  292 - 123   0 210
             244  292 - 123   0 210
              40  296 - 237 207 197
             304  296 - 114   0 213
             260  300 - 104   0 216
             116  304 - 174 142 121
             136  308 - 172 140 117
             232  308 -  81   0 223
             288  308 -  81   0 223
              44  312 - 233 200 191
              80  312 - 211 163 141
             152  312 - 187 156 135
             216  312 -  65   0 226
             244  312 -  65   0 226
             260  312 -  65   0 226
             184  316 - 218 170 124
             272  316 -  44   0 229
             316  316 - 218 170 124
             592  352 -  89 124 149
             432  404 -  89 124 149
             348  552 -  89 124 149
             104  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        Ok(())
    }
}

pub async fn test_corner_radii() -> Result<()> {
    run_ui_test()
}
