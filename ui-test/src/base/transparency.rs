use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    gm::Apply,
    level::LevelManager,
    refs::Weak,
    ui::{ImageView, Setup, ViewData, view},
    ui_test::{UITest, check_colors},
};

use crate::level::SkyboxLevel;

#[view]
struct Transparency {
    #[init]
    background: ImageView,

    view_1: ImageView,
    view_2: ImageView,
    view_3: ImageView,
    view_4: ImageView,
}

impl Setup for Transparency {
    fn setup(self: Weak<Self>) {
        self.background.set_image("gradient.png").place().back();

        self.view_1.set_image("wood-window.png");
        self.view_2.set_image("wood-window.png").place().tl(50);
        self.view_3.set_image("wood-window.png").place().tl(100);
        self.view_4.set_image("wood-window.png").place().tl(150);

        [self.view_1, self.view_2, self.view_3, self.view_4].apply(|v| {
            v.place().size(280, 280);
        });
    }
}

pub async fn test_transparency() -> Result<()> {
    UITest::start::<Transparency>();

    from_main(|| {
        LevelManager::set_level(SkyboxLevel::default());
    });

    from_main(|| {
        LevelManager::stop_level();
    });

    check_colors(
        r#"
            100    4 -  96 209  74
            308    4 -  54 123 128
            592    4 -   3   5 239
            380    8 -  40  93 154
            240   28 -  69 151 101
            512   28 -  20  37 202
            44   36 -  89 124 149
            40   40 -  89 124 149
            592  100 -  44  44 201
            4  152 - 103 188  55
            560  168 -  73  73 167
            4  216 - 109 161  47
            592  224 -  97  97 152
            36  268 - 119 140  43
            560  272 - 117 117 129
            292  308 -  89 124 149
            592  332 - 142 142 114
            4  348 - 142 109  34
            560  380 - 162 162  94
            16  432 - 170  79  31
            592  444 - 189 189  86
            556  548 -  89 124 149
            104  556 - 217  65  38
            212  556 - 219 101  43
            364  556 - 224 159  57
            424  560 - 228 183  63
            4  592 - 230  51  35
            160  592 - 232  84  40
            316  592 - 236 141  52
            444  592 - 242 191  66
            508  592 - 245 217  73
            592  592 - 251 251  83
        "#,
    )?;

    Ok(())
}
