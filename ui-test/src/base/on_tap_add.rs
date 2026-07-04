use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{
        Button, CellRegistry, Container, GREEN, ImageView, Label, Setup, TURQUOISE, TableData, TableView,
        View, ViewData, ViewSubviews, WHITE,
        ui_test::{helpers::check_colors, inject_touches},
        view,
    },
    ui_test::UITest,
};

#[view]
struct SomeView {
    #[init]
    table:  TableView,
    label:  Label,
    image:  ImageView,
    square: Container,
}

impl Setup for SomeView {
    fn setup(self: Weak<Self>) {
        self.table.set_data_source(self).register_cell::<Label>().place().size(400, 400);
        self.label.set_text("Hello").set_color(GREEN).place().size(200, 200).tr(10);
        self.image.set_image("plus.png").place().size(200, 200).bl(10);
        self.square.set_color(TURQUOISE).place().size(200, 200).br(10);
    }
}

impl TableData for SomeView {
    fn number_of_cells(&self) -> usize {
        2
    }

    fn cell_height(&self, _: usize) -> f32 {
        50.0
    }

    fn setup_cell(&mut self, index: usize, registry: &mut CellRegistry) -> Weak<dyn View> {
        registry.cell::<Label>().set_color(WHITE).set_text(format!("{index}")).weak()
    }
}

#[view]
struct AddOnTap {
    #[init]
    btn: Button,
}

impl Setup for AddOnTap {
    fn setup(self: Weak<Self>) {
        self.btn.set_text("A").place().size(50, 50);
        self.btn.on_tap(move || {
            let view = self.add_view::<SomeView>();
            view.place().size(600, 500).br(5);
        });
    }
}

pub async fn test_add_on_tap() -> Result<()> {
    let view = UITest::start::<AddOnTap>();

    assert_eq!(view.dump_subviews(), vec!["AddOnTap.btn: Button".to_string()]);

    inject_touches(
        "
            25   25   b
            25   25   e
        ",
    );

    assert_eq!(
        view.dump_subviews(),
        vec!["AddOnTap.btn: Button".to_string(), "SomeView".to_string()]
    );

    check_colors(
        r#"
            4    4 - 255 255 255
            292    4 -  89 124 149
            592    4 -  89 124 149
            80   96 - 255 255 255
            396  108 -   0 255   0
            576  112 -   0 255   0
            192  116 - 255 255 255
            196  116 - 255 255 255
            192  120 - 255 255 255
            196  120 - 255 255 255
            16  192 - 255 255 255
            120  192 - 255 255 255
            300  192 - 255 255 255
            512  200 -   0 255   0
            512  204 -   0 255   0
            480  208 -   0 255   0
            512  208 -   0 255   0
            388  300 -   0 255   0
            484  300 -   0 255   0
            580  300 -   0 255   0
            516  388 -   0 255 255
            160  404 -  33 150 243
            592  436 -  89 124 149
            444  440 -   0 255 255
            16  444 -  33 150 243
            296  468 -  89 124 149
            72  524 -  33 150 243
            472  528 -   0 255 255
            160  564 -  33 150 243
            396  580 -   0 255 255
            548  580 -   0 255 255
            4  592 -  89 124 149
        "#,
    )?;

    Ok(())
}
