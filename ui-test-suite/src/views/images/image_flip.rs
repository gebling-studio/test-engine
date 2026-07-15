use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{ImageView, Setup, ViewFrame, ViewSubviews, ui_test, view},
    ui_test::{UITest, check_colors},
};

#[view]
struct ImageFlip {
    #[init]
    tl: ImageView,
    tr: ImageView,
    bl: ImageView,
    br: ImageView,
}

impl Setup for ImageFlip {
    fn setup(mut self: Weak<Self>) {
        self.apply_to::<ImageView>(|i| {
            i.set_image("cat.png");
        });

        self.tl.set_frame((50, 50, 150, 150));
        self.tr.set_frame((250, 50, 150, 150));
        self.tr.flip_x = true;
        self.bl.set_frame((50, 250, 150, 150));
        self.bl.flip_y = true;
        self.br.set_frame((250, 250, 150, 150));
        self.br.flip_x = true;
        self.br.flip_y = true;
    }
}

#[ui_test]
pub fn test_image_flip() -> Result<()> {
    let _view = UITest::start::<ImageFlip>();

    check_colors(
        r"
            592    4 -  89 124 149
            52   52 - 234 199 205
            180   52 - 217 175 176
            312   52 - 223 183 184
            396   52 - 234 199 205
            280  128 - 179 144 122
            52  144 - 222 166 167
            184  148 - 172 144 122
            252  156 - 164 136 115
            380  160 - 240 210 200
            192  188 - 185 155 131
            280  188 - 166 136 112
            116  192 - 209 162 142
            336  192 - 208 161 143
            336  256 - 207 160 142
            592  256 -  89 124 149
            280  260 - 166 134 111
            188  264 - 175 145 121
            52  268 - 222 168 168
            156  288 - 198 165 146
            256  288 - 153 126 105
            396  308 - 223 167 168
            196  340 - 203 153 152
            252  344 - 203 153 152
            52  376 - 238 199 204
            156  396 - 220 178 179
            272  396 - 220 174 176
            384  396 - 231 192 197
            556  424 -  89 124 149
            4  592 -  89 124 149
            328  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )?;

    Ok(())
}
