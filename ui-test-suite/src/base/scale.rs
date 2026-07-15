use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::{Weak, weak_from_ref},
    ui::{
        Button, CellRegistry, Label, Setup, TableData, TableView, UIManager, View, ViewData, ViewSubviews,
        WHITE, ui_test, view,
    },
    ui_test::{UITest, check_colors, inject_touches},
};

#[view]
struct ScaleView {
    data: Vec<String>,

    #[init]
    label:  Label,
    button: Button,
    table:  TableView,

    tr_button: Button,
    bl_button: Button,
    br_button: Button,
}

impl Setup for ScaleView {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text("Label").set_color(WHITE);
        self.label.place().tl(20).size(150, 80);

        self.button.set_text("Button");
        self.button.place().below(self.label, 20);

        self.table.place().size(200, 280).br(20);
        self.table.set_data_source(self).register_cell::<Label>();

        self.tr_button.place().tr(20).size(50, 50);
        self.bl_button.place().bl(20).size(50, 50);
        self.br_button.place().br(20).size(50, 50);

        let mut this = self;
        self.apply_to::<Button>(move |b| {
            let b = weak_from_ref(b);
            b.on_tap(move || {
                this.data.push(b.label().to_string());
            });
        });
    }
}

impl TableData for ScaleView {
    fn number_of_cells(&self) -> usize {
        4
    }

    fn setup_cell(&mut self, index: usize, registry: &mut CellRegistry) -> Weak<dyn View> {
        let cell = registry.cell::<Label>();
        cell.set_text(index);
        cell.set_color(WHITE);
        cell
    }
}

#[ui_test]
pub fn test_scale() -> Result<()> {
    let view = UITest::start::<ScaleView>();

    check_default_scale(view)?;
    check_downscaled(view)?;
    check_upscaled(view)?;

    from_main(move || {
        UIManager::override_scale(1);
    });

    Ok(())
}

fn check_default_scale(view: Weak<ScaleView>) -> Result<()> {
    inject_touches(
        "
            39   40   b
            39   40   e
            61   541  b
            61   541  e
            538  539  b
            538  539  e
            537  60   b
            537  60   e
            551  83   b
            551  83   e
            517  46   b
            518  46   e
            182  167  b
            182  167  e
            78   171  b
            78   171  e
            82   210  b
            82   210  e
            51   518  b
            51   518  e
            88   560  b
            88   560  e
            516  557  b
            516  557  e

        ",
    );

    let data = from_main(move || view.data.clone());

    assert_eq!(
        data,
        [
            "ScaleView.bl_button: Button",
            "ScaleView.br_button: Button",
            "ScaleView.tr_button: Button",
            "ScaleView.button: Button",
        ]
        .map(ToOwned::to_owned)
        .into_iter()
        .collect::<Vec<_>>()
    );

    check_colors(
        r"
            4    4 -  89 124 149
            300    4 -  89 124 149
            592    4 -  89 124 149
            168   24 - 255 255 255
            80   56 - 255 255 255
            100   56 - 255 255 255
            80   64 - 255 255 255
            100   64 - 255 255 255
            532   68 - 255 255 255
            384  108 -  89 124 149
            56  152 - 255 255 255
            104  160 - 255 255 255
            116  160 - 255 255 255
            56  164 - 255 255 255
            592  184 -  89 124 149
            272  240 -  89 124 149
            132  300 -  89 124 149
            392  304 - 255 255 255
            568  304 - 255 255 255
            480  320 - 255 255 255
            4  360 -  89 124 149
            384  392 - 255 255 255
            576  392 - 255 255 255
            224  404 -  89 124 149
            480  468 - 255 255 255
            480  476 - 255 255 255
            572  484 - 255 255 255
            384  496 - 255 255 255
            172  544 -  89 124 149
            24  576 - 255 255 255
            576  576 - 255 255 255
            312  592 -  89 124 149
        ",
    )?;

    Ok(())
}

fn check_downscaled(mut view: Weak<ScaleView>) -> Result<()> {
    from_main(move || {
        UIManager::override_scale(0.6);
        view.data.clear();
    });

    inject_touches(
        "
            53   958  b
            53   958  e
            53   958  b
            53   958  e
            53   956  b
            53   956  e
            948  942  b
            948  942  e
            955  39   b
            955  39   e
            960  95   b
            960  95   e
            899  52   b
            899  52   e
            128  185  b
            128  185  e

        ",
    );

    let data = from_main(move || view.data.clone());

    assert_eq!(
        data,
        [
            "ScaleView.bl_button: Button",
            "ScaleView.bl_button: Button",
            "ScaleView.bl_button: Button",
            "ScaleView.br_button: Button",
            "ScaleView.tr_button: Button",
            "ScaleView.button: Button",
        ]
        .map(ToOwned::to_owned)
        .into_iter()
        .collect::<Vec<_>>()
    );

    check_colors(
        r"
            4    4 -  89 124 149
            216    4 -  89 124 149
            428    4 -  89 124 149
            592    4 -  89 124 149
            96   16 - 255 255 255
            60   36 - 255 255 255
            560   40 - 255 255 255
            16   52 - 255 255 255
            100   56 - 255 255 255
            36   96 - 255 255 255
            68   96 - 255 255 255
            340  108 -  89 124 149
            472  152 -  89 124 149
            200  196 -  89 124 149
            592  224 -  89 124 149
            404  288 -  89 124 149
            4  324 -  89 124 149
            228  360 -  89 124 149
            480  424 - 255 255 255
            584  424 - 255 255 255
            528  432 - 255 255 255
            100  452 -  89 124 149
            560  460 - 255 255 255
            324  468 -  89 124 149
            472  468 - 255 255 255
            524  484 - 255 255 255
            584  496 - 255 255 255
            472  536 - 255 255 255
            536  536 - 255 255 255
            16  584 - 255 255 255
            584  584 - 255 255 255
            248  592 -  89 124 149
        ",
    )?;

    Ok(())
}

fn check_upscaled(mut view: Weak<ScaleView>) -> Result<()> {
    from_main(move || {
        UIManager::override_scale(1.5);
        view.data.clear();
    });

    inject_touches(
        "
            40   389  b
            40   389  e
            44   363  b
            44   363  e
            40   318  b
            40   318  e
            307  356  b
            308  356  e
            347  356  b
            347  356  e
            390  355  b
            390  355  e
            348  86   b
            348  86   e
            352  45   b
            352  45   e
            349  10   b
            349  10   e
            75   112  b
            75   112  e
            74   135  b
            74   135  e
            63   185  b
            63   185  e
            59   215  b
            59   215  e

        ",
    );

    let data = from_main(move || view.data.clone());

    assert_eq!(
        data,
        [
            "ScaleView.bl_button: Button",
            "ScaleView.br_button: Button",
            "ScaleView.tr_button: Button",
            "ScaleView.button: Button",
            "ScaleView.button: Button"
        ]
        .map(ToOwned::to_owned)
        .into_iter()
        .collect::<Vec<_>>()
    );

    check_colors(
        r"
            356    4 -  89 124 149
            568   32 - 255 255 255
            196   72 -   0   0   0
            124   84 - 255 255 255
            180   84 - 255 255 255
            148   92 - 255 255 255
            168   92 -   1   1   1
            92  100 -   0   0   0
            116  100 -   1   1   1
            196  100 -   0   0   0
            424  184 - 255 255 255
            296  188 - 255 255 255
            80  220 -   1   1   1
            92  228 - 255 255 255
            156  240 - 255 255 255
            100  244 -   0   0   0
            180  244 -   0   0   0
            420  248 -   1   1   1
            568  264 - 255 255 255
            416  324 - 255 255 255
            428  324 -   1   1   1
            420  348 -   0   0   0
            4  360 -  89 124 149
            272  380 - 255 255 255
            420  408 -   1   1   1
            568  416 - 255 255 255
            420  424 -   1   1   1
            132  436 -  89 124 149
            568  568 - 255 255 255
            408  584 -  89 124 149
            4  592 -  89 124 149
            248  592 -  89 124 149
        ",
    )?;

    Ok(())
}
