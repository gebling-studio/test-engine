use anyhow::Result;
use instant::Instant;
use test_engine::{
    dispatch::from_main,
    refs::{Own, Weak},
    ui::{
        Container, NavigationView, PRESENT_ANIMATION_DURATION, RED, Setup, TouchStack, View, ViewController,
        ViewData, ViewTest, view,
    },
    ui_test::helpers::check_colors,
};

#[view]
struct PresentTestView {}

impl ViewTest for PresentTestView {
    /// Presenting only works from inside a navigation stack, so the root is the
    /// stack and the view under test is its first view.
    fn make_root(view: Own<Self>) -> Own<dyn View> {
        NavigationView::with_view(view)
    }

    fn perform_test(view: Weak<Self>) -> Result<()> {
        check_before_present()?;
        check_present_animation(view)?;
        check_presented_stays()?;

        Ok(())
    }
}

fn check_before_present() -> Result<()> {
    check_colors(
        r"
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
        ",
    )?;

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    Ok(())
}

fn check_present_animation(view: Weak<PresentTestView>) -> Result<()> {
    let now = Instant::now();

    let presented = from_main(move || {
        let presented = Container::new();
        presented.set_color(RED);

        view.present(presented)
    });

    presented.recv()?;

    let duration_error = now.elapsed().as_secs_f32() - PRESENT_ANIMATION_DURATION;
    let allowed_error = 0.04;

    check_colors(
        r"
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
        ",
    )?;

    assert!(
        duration_error < allowed_error,
        "Duration error is: {duration_error}. Allowed: {allowed_error}"
    );

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    Ok(())
}

fn check_presented_stays() -> Result<()> {
    check_colors(
        r"
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
        ",
    )?;

    Ok(())
}
