use anyhow::Result;
use instant::Instant;
use test_engine::{
    dispatch::from_main,
    ui::{
        Container, NavigationView, PRESENT_ANIMATION_DURATION, RED, Setup, TouchStack, ViewController,
        ViewData, view,
    },
    ui_test::{UITest, helpers::check_colors},
};

#[view]
struct PresentTestView {}

pub async fn test_navigation_view() -> Result<()> {
    let present = PresentTestView::new();

    let view = present.weak();

    UITest::set(
        NavigationView::with_view(present),
        600,
        600,
        true,
        "Present".to_string(),
    );

    check_colors(
        r#"
            4    4 -  89 124 149
            444    4 -  89 124 149
            592    4 -  89 124 149
            296    8 -  89 124 149
            148   12 -  89 124 149
            228   84 -  89 124 149
            12  148 -  89 124 149
            444  152 -  89 124 149
            592  152 -  89 124 149
            156  156 -  89 124 149
            300  156 -  89 124 149
            84  228 -  89 124 149
            228  228 -  89 124 149
            372  228 -  89 124 149
            8  296 -  89 124 149
            448  296 -  89 124 149
            156  300 -  89 124 149
            300  300 -  89 124 149
            592  300 -  89 124 149
            228  372 -  89 124 149
            372  372 -  89 124 149
            516  372 -  89 124 149
            4  444 -  89 124 149
            152  444 -  89 124 149
            444  444 -  89 124 149
            296  448 -  89 124 149
            588  448 -  89 124 149
            448  588 -  89 124 149
            4  592 -  89 124 149
            152  592 -  89 124 149
            300  592 -  89 124 149
            592  592 -  89 124 149
        "#,
    )?;

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    let now = Instant::now();

    let presented = from_main(move || {
        let presented = Container::new();
        presented.set_color(RED);

        view.present(presented)
    });

    presented.recv()?;

    let duration_error = now.elapsed().as_secs_f32() - PRESENT_ANIMATION_DURATION;
    let allowed_error = 0.032;

    check_colors(
        r#"
            4    4 - 255 255 255
            444    4 - 255 255 255
            592    4 - 255 255 255
            296    8 - 255 255 255
            148   12 - 255 255 255
            228   84 - 255 255 255
            12  148 - 255 255 255
            444  152 - 255 255 255
            592  152 - 255 255 255
            156  156 - 255 255 255
            300  156 - 255 255 255
            84  228 - 255 255 255
            228  228 - 255 255 255
            372  228 - 255 255 255
            8  296 - 255 255 255
            448  296 - 255 255 255
            156  300 - 255 255 255
            300  300 - 255 255 255
            592  300 - 255 255 255
            228  372 - 255 255 255
            372  372 - 255 255 255
            516  372 - 255 255 255
            4  444 - 255 255 255
            152  444 - 255 255 255
            444  444 - 255 255 255
            296  448 - 255 255 255
            588  448 - 255 255 255
            448  588 - 255 255 255
            4  592 - 255 255 255
            152  592 - 255 255 255
            300  592 - 255 255 255
            592  592 - 255 255 255
        "#,
    )?;

    assert!(
        duration_error < allowed_error,
        "Duration error is: {duration_error}. Allowed: {allowed_error}"
    );

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    check_colors(
        r#"
            4    4 - 255 255 255
            444    4 - 255 255 255
            592    4 - 255 255 255
            296    8 - 255 255 255
            148   12 - 255 255 255
            228   84 - 255 255 255
            12  148 - 255 255 255
            444  152 - 255 255 255
            592  152 - 255 255 255
            156  156 - 255 255 255
            300  156 - 255 255 255
            84  228 - 255 255 255
            228  228 - 255 255 255
            372  228 - 255 255 255
            8  296 - 255 255 255
            448  296 - 255 255 255
            156  300 - 255 255 255
            300  300 - 255 255 255
            592  300 - 255 255 255
            228  372 - 255 255 255
            372  372 - 255 255 255
            516  372 - 255 255 255
            4  444 - 255 255 255
            152  444 - 255 255 255
            444  444 - 255 255 255
            296  448 - 255 255 255
            588  448 - 255 255 255
            448  588 - 255 255 255
            4  592 - 255 255 255
            152  592 - 255 255 255
            300  592 - 255 255 255
            592  592 - 255 255 255
        "#,
    )?;

    Ok(())
}
