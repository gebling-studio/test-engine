use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Container, GREEN, ImageView, Setup, UIImages, ViewData, ViewSubviews, view},
    ui_test::{UITest, helpers::check_colors},
};

#[view]
struct ImageOnView {
    image: Weak<ImageView>,

    #[init]
    container: Container,
}

impl Setup for ImageOnView {
    fn setup(mut self: Weak<Self>) {
        self.container.set_color(GREEN).place().size(200, 200).tl(100);

        self.image = self.container.add_view();

        self.image.set_image(UIImages::rb()).place().size(100, 100).center();
    }
}

pub async fn test_image_on_view() -> Result<()> {
    UITest::start::<ImageOnView>();

    check_colors(
        r"
            416    4 -  89 124 149
            592    4 -  89 124 149
            104  104 -   0 255   0
            296  108 -   0 255   0
            200  116 -   0 255   0
            152  152 -   0 255   0
            476  152 -  89 124 149
            248  156 -  68  68  68
            224  180 -  68  68  68
            296  180 -   0 255   0
            104  188 -   0 255   0
            220  200 -  68  68  68
            248  200 -  68  68  68
            200  204 -  68  68  68
            224  220 -  68  68  68
            188  236 -  68  68  68
            248  236 -  68  68  68
            296  240 -   0 255   0
            108  244 -   0 255   0
            160  248 -  68  68  68
            216  248 -  68  68  68
            104  296 -   0 255   0
            188  296 -   0 255   0
            248  296 -   0 255   0
            592  296 -  89 124 149
            304  300 -  89 124 149
            4  420 -  89 124 149
            452  444 -  89 124 149
            152  476 -  89 124 149
            4  592 -  89 124 149
            300  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )?;

    Ok(())
}
