use std::env::temp_dir;

use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Container, GREEN, Setup, ViewData, ViewFrame, view},
    ui_test::{UITest, check_colors},
};

#[view]
struct TestColorChecker {
    #[init]
    view: Container,
}

impl Setup for TestColorChecker {
    fn setup(self: Weak<Self>) {
        self.view.set_frame((80, 200, 20, 20)).set_color(GREEN);
    }
}

pub async fn test_color_checker() -> Result<()> {
    let _view = UITest::start::<TestColorChecker>();

    check_colors(
        r#"
              76  215 -  89 124 149
              90  214 -   0 255   0
             112  213 -  89 124 149
        "#,
    )?;

    let error = check_colors(
        r#"
              76  215 -  89 124 149
              90  214 -   0   0 255
             112  213 -  89 124 149
        "#,
    )
    .err()
    .unwrap()
    .to_string();

    assert!(error.starts_with(
        r"
        Test: Test color checker has failed.
        Color diff is too big: 510. Max: 45. Position: Point { x: 90.0, y: 214.0 }.
        Expected: r: 0, g: 0, b: 255, a: 255, got: r: 0, g: 255, b: 0, a: 255.
          90  214 -   0   0 255 ->   0 255   0"
    ));

    let screenshot_path = temp_dir().join("ui_test_Test_color_checker.png");

    assert!(error.contains(&format!("Failure screenshot: {}", screenshot_path.display())));
    assert!(error.contains("View tree"));
    assert!(screenshot_path.exists());

    check_colors(
        r#"
              10  225 -  89 124 149
              36  218 -   0   0 255
              68  216 -  89 124 149
              89  209 -   0 255   0
             117  211 -  89 124 149
             136  211 -   0 255   0
             187  210 -  89 124 149
             109  299 -  89 124 149
             100  266 -   0   0 255
              95  231 -  89 124 149
              95  213 -   0 255   0
              90  188 -  89 124 149
              80  158 -   0   0 255
              52  121 -  89 124 149
              13  131 -  89 124 149
        "#,
    )?;

    Ok(())
}
