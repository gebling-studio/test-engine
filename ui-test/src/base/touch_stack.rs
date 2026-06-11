use anyhow::Result;
use log::debug;
use test_engine::{
    dispatch::from_main,
    ui::{Alert, Button, TouchStack, ViewData, ViewSubviews, ViewTouch, view},
    ui_test::{UITest, inject_touches},
};

#[view]
struct TouchStackTestView {
    // #[text = a]
    #[init]
    button:  Button,
    // #[text = b]
    button2: Button,
}

pub async fn test_touch_stack() -> Result<()> {
    let view = UITest::start::<TouchStackTestView>();

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    let mut button = from_main(move || view.add_view::<Button>());

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    button.on_tap(|| {});

    assert_eq!(
        TouchStack::dump(),
        vec![vec!["Layer: Root view", button.view_label()]],
    );

    from_main(move || button.remove_from_superview());

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    view.button.on_tap(|| {});

    assert_eq!(
        TouchStack::dump(),
        vec![vec!["Layer: Root view", view.button.view_label()]],
    );

    view.button2.on_tap(|| {});

    assert_eq!(
        TouchStack::dump(),
        vec![vec![
            "Layer: Root view",
            view.button.view_label(),
            view.button2.view_label(),
        ]],
    );

    view.button.disable_touch();
    view.button2.disable_touch();

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    Alert::show("Hello");


    assert_eq!(
        TouchStack::dump(),
        vec![
            vec!["Layer: Root view".to_string()],
            vec!["Layer: Alert".to_string(), "Alert.ok_button: Button".to_string()],
        ],
    );

    inject_touches(
        r"
            320  383  b
            320  383  e
    ",
    );

    assert_eq!(TouchStack::dump(), vec![vec!["Layer: Root view".to_string()]]);

    debug!("Touch stack test: OK");

    Ok(())
}
