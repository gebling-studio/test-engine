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

        rounded_corners_colors()
    }
}

fn rounded_corners_colors() -> Result<()> {
    check_colors(
        r"
                24   24 - 255 203   0
                132   24 - 255 203   0
                208   24 - 255   0 255
                288   24 - 255 255   0
                436   24 - 255   0   0
                392   28 -   0 255   0
                344   32 - 255   0   0
                476   40 - 255   0   0
                184   48 - 255   0 255
                76   52 -   0   0 231
                348   80 -   0 255   0
                464   84 -   0 255   0
                420   88 -   0 255   0
                116   96 -   0   0 231
                36  104 -   0   0 231
                380  108 -   0 255   0
                252  112 - 255 255   0
                344  112 - 255   0   0
                184  120 - 255 255   0
                316  128 - 255   0 255
                472  128 - 255   0   0
                80  132 -   0   0 231
                348  148 - 188 188 188
                468  148 - 188 188 188
                472  148 - 188 188 188
                372  152 - 255   0   0
                448  152 - 255   0   0
                468  152 - 188 188 188
                292  156 - 255   0 255
                316  156 - 255   0 255
                420  156 - 255   0   0
                24  184 -   0 255 255
                44  184 -   0 255 255
                68  184 - 235 195 196
                88  184 - 224 184 185
                156  184 -   0 255 255
                184  188 - 248   0  62
                232  188 - 248   0  62
                260  188 - 248   0  62
                300  188 - 248   0  62
                56  200 - 229 189 189
                136  200 - 208 160 158
                208  204 - 234   0 105
                24  208 -   0 255 255
                108  208 - 222 176 176
                156  208 -   0 255 255
                276  208 - 231   0 112
                316  216 - 223   0 126
                184  220 - 219   0 132
                244  220 - 219   0 132
                268  228 - 211   0 144
                156  232 - 203 153 152
                212  232 - 207   0 149
                292  236 - 203   0 154
                48  240 - 241 214 203
                108  244 -  14   4   3
                236  244 - 194   0 163
                28  256 - 226 176 177
                188  256 - 179   0 177
                260  256 - 179   0 177
                316  260 - 174   0 181
                48  264 - 236 208 197
                156  268 - 196 146 147
                224  268 - 163   0 188
                288  268 - 163   0 188
                64  272 - 214 179 157
                96  272 - 200 164 140
                124  272 - 196 164 143
                36  276 - 240 211 203
                148  276 - 161 133 111
                248  276 - 151   0 196
                140  284 - 167 136 115
                188  284 - 138   0 203
                232  288 - 130   0 206
                264  288 - 130   0 206
                284  288 - 130   0 206
                304  296 - 114   0 213
                140  300 - 164 134 110
                104  304 - 200 168 147
                116  304 - 174 142 121
                204  304 -  93   0 219
                240  308 -  81   0 223
                288  308 -  81   0 223
                44  312 - 233 200 191
                72  312 - 217 179 166
                220  312 -  65   0 226
                256  312 -  65   0 226
                280  312 -  65   0 226
                156  316 - 180 152 130
                272  316 -  44   0 229
                316  316 - 218 170 124
                588  356 -  89 124 149
                4  492 -  89 124 149
                376  520 -  89 124 149
                156  592 -  89 124 149
                592  592 -  89 124 149
            ",
    )
}

pub async fn test_corner_radii() -> Result<()> {
    run_ui_test()
}
