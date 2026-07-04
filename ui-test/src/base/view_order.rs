use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{BLACK, BLUE, Container, GREEN, RED, Setup, ViewData, ViewSubviews, view},
    ui_test::{UITest, helpers::check_colors},
};

#[view]
pub struct ViewOrder {
    #[init]
    view_1: Container,
    view_2: Container,
    view_3: Container,
    view_4: Container,
}

impl Setup for ViewOrder {
    fn setup(self: Weak<Self>) {
        self.view_1.set_color(RED).place().size(200, 200);
        self.view_2.set_color(GREEN).place().size(200, 200).tl(100);
        self.view_3.set_color(BLUE).place().size(200, 200).tl(200);
        self.view_4.set_color(BLACK).place().size(200, 200).tl(300);
    }
}

pub async fn test_view_order() -> Result<()> {
    let view = UITest::start::<ViewOrder>();

    assert_eq!(
        view.dump_subviews(),
        vec![
            "ViewOrder.view_1: Container".to_string(),
            "ViewOrder.view_2: Container".to_string(),
            "ViewOrder.view_3: Container".to_string(),
            "ViewOrder.view_4: Container".to_string()
        ]
    );

    assert_eq!(view.view_1.view_label(), "ViewOrder.view_1: Container");
    assert_eq!(view.view_2.view_label(), "ViewOrder.view_2: Container");
    assert_eq!(view.view_3.view_label(), "ViewOrder.view_3: Container");
    assert_eq!(view.view_4.view_label(), "ViewOrder.view_4: Container");

    assert_eq!(view.subviews()[0].label(), view.view_1.view_label());
    assert_eq!(view.subviews()[1].label(), view.view_2.view_label());
    assert_eq!(view.subviews()[2].label(), view.view_3.view_label());
    assert_eq!(view.subviews()[3].label(), view.view_4.view_label());

    check_colors(
        r#"
            4    4 - 255   0   0
            392    4 -  89 124 149
            592    4 -  89 124 149
            196   32 - 255   0   0
            96   48 - 255   0   0
            156   92 - 255   0   0
            12  100 - 255   0   0
            296  104 -   0 255   0
            212  140 -   0 255   0
            104  148 -   0 255   0
            592  180 -  89 124 149
            4  196 - 255   0   0
            240  196 -   0 255   0
            304  196 -  89 124 149
            396  212 -   0   0 231
            160  220 -   0 255   0
            224  288 -   0   0 231
            104  296 -   0 255   0
            356  296 -   0   0 231
            468  304 -   0   0   0
            288  324 -   0   0 231
            204  368 -   0   0 231
            496  388 -   0   0   0
            280  396 -   0   0 231
            408  396 -   0   0   0
            4  428 -  89 124 149
            344  428 -   0   0   0
            432  464 -   0   0   0
            364  496 -   0   0   0
            496  496 -   0   0   0
            4  592 -  89 124 149
            196  592 -  89 124 149
        "#,
    )?;

    Ok(())
}
