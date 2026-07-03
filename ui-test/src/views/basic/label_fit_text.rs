use anyhow::Result;
use test_engine::{
    dispatch::{from_main, wait_for_next_frame},
    refs::Weak,
    ui::{GREEN, Label, Setup, TextAlignment, ViewData, ViewFrame, ViewTest, YELLOW, view_test},
    ui_test::check_colors,
};

#[view_test]
struct LabelFitText {
    #[init]
    tag:      Label,
    panel:    Label,
    centered: Label,
}

impl Setup for LabelFitText {
    fn setup(self: Weak<Self>) {
        self.tag.set_color(GREEN);
        self.tag.set_alignment(TextAlignment::Left);
        self.tag.set_text("tag").set_text_size(40);
        self.tag.place().tl(20).fit_text();

        self.panel.set_color(YELLOW);
        self.panel.set_multiline(true);
        self.panel
            .set_text("Grumpy wizards make toxic brew for the jovial queen")
            .set_text_size(40);
        self.panel.place().t(120).lr(20).fit_text_height();

        self.centered.set_color(GREEN);
        self.centered.set_text("mid").set_text_size(40);
        self.centered.place().t(400).center_x().fit_text();
    }
}

impl ViewTest for LabelFitText {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        check_colors(
            r"
              64   24 -   0 255   0
              44   40 -   0 255   0
              60   44 -   0 255   0
              80   44 -   0 255   0
             352  124 - 255 255   0
             468  128 -   0   0   0
             168  136 - 255 255   0
             292  136 -   0   0   0
              80  140 -  13  13   0
             528  140 -   0   0   0
             400  144 - 255 255   0
             116  148 -  13  13   0
             228  148 -  13  13   0
             308  164 -   0   0   0
             484  184 - 255 255   0
             160  188 -   0   0   0
             272  188 - 255 255   0
             340  188 - 255 255   0
              24  196 - 255 255   0
             104  196 - 255 255   0
             424  196 -  13  13   0
             576  196 - 255 255   0
             592  360 -  89 124 149
              64  396 -  89 124 149
             272  404 -   0 255   0
             316  404 -   0 255   0
             320  420 -   0 255   0
             292  436 -   0 255   0
             328  436 -   0 255   0
               4  592 -  89 124 149
             192  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        let (tag, panel, centered) =
            from_main(move || (*view.tag.frame(), *view.panel.frame(), *view.centered.frame()));

        assert!(
            tag.origin.x == 20.0 && tag.origin.y == 20.0,
            "fit_text moved the anchored origin: {tag:?}"
        );
        assert!(
            tag.size.width < 200.0 && tag.size.height < 70.0,
            "fitted frame does not hug the text: {tag:?}"
        );
        assert!(
            panel.size.width == 560.0,
            "fit_text_height changed the width set by side rules: {panel:?}"
        );
        assert!(
            (centered.center().x - 300.0).abs() < 1.0,
            "fitted label is not centered: {centered:?}"
        );

        from_main(move || {
            view.tag.set_text("much longer tag");
            view.panel.set_text(
                "Grumpy wizards make toxic brew for the jovial queen and jack, then brew even more",
            );
            view.centered.set_text("wide middle");
        });

        wait_for_next_frame();

        check_colors(
            r"
             320   24 -   0 255   0
             100   36 -   0 255   0
             228   36 -   0 255   0
             272   44 -   0   0   0
              24   56 -   0 255   0
             184   56 -   0 255   0
             392  124 - 255 255   0
             468  128 -   0   0   0
             260  132 -   0   0   0
              64  136 - 255 255   0
             332  144 - 255 255   0
             116  148 -  13  13   0
             216  148 -  13  13   0
             164  176 - 255 255   0
             516  180 -  13  13   0
             232  220 -   0   0   0
             472  220 -   0   0   0
             104  224 - 255 255   0
             308  228 - 255 255   0
              24  236 - 255 255   0
             420  236 - 255 255   0
             576  236 - 255 255   0
             592  368 -  89 124 149
             240  404 -   0 255   0
             356  404 -   0 255   0
               4  412 -  89 124 149
             396  416 -   0 255   0
             304  420 -   0   0   0
             196  436 -   0 255   0
               4  592 -  89 124 149
             356  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        let (grown_tag, grown_panel, grown_centered) =
            from_main(move || (*view.tag.frame(), *view.panel.frame(), *view.centered.frame()));

        assert!(
            grown_tag.size.width > tag.size.width + 50.0,
            "fitted width did not follow longer text: {grown_tag:?}"
        );
        assert!(
            grown_panel.size.height > panel.size.height,
            "fitted height did not grow with more wrapped text: {grown_panel:?}"
        );
        assert!(
            grown_centered.size.width > centered.size.width,
            "centered fitted width did not grow: {grown_centered:?}"
        );
        assert!(
            (grown_centered.center().x - 300.0).abs() < 1.0,
            "label did not stay centered after refit: {grown_centered:?}"
        );

        Ok(())
    }
}

pub async fn test_label_fit_text() -> Result<()> {
    run_ui_test()
}
