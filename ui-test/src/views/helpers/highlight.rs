use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{BLUE, GREEN, HighlightView, Setup, view},
    ui_test::{UITest, check_colors},
};

#[view]
struct HighLightTestView {
    #[init]
    highlight: HighlightView,
}

impl Setup for HighLightTestView {
    fn setup(mut self: Weak<Self>) {
        self.highlight.set((200, 200), GREEN, BLUE);
    }
}

pub async fn test_highlight() -> Result<()> {
    UITest::start::<HighLightTestView>();

    check_colors(
        r#"
            392    4 -  89 124 149
            592    4 -  89 124 149
            128  128 -   0 255   0
            168  128 -   0 255   0
            204  128 -   0 255   0
            236  128 -   0   0 231
            268  128 -   0   0 231
            196  156 -   0 255   0
            236  160 -   0   0 231
            268  160 -   0   0 231
            156  164 -   0 255   0
            452  180 -  89 124 149
            236  192 -   0   0 231
            136  200 -   0 255   0
            268  212 -   0   0 231
            160  224 -   0 255   0
            192  236 -   0 255   0
            128  240 -   0 255   0
            240  240 -   0   0 231
            268  240 -   0   0 231
            160  268 -   0 255   0
            212  268 -   0 255   0
            240  268 -   0   0 231
            268  268 -   0   0 231
            300  300 -  89 124 149
            592  300 -  89 124 149
            4  392 -  89 124 149
            444  444 -  89 124 149
            180  452 -  89 124 149
            4  592 -  89 124 149
            300  592 -  89 124 149
            592  592 -  89 124 149
        "#,
    )?;

    Ok(())
}
