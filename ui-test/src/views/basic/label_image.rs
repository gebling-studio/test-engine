use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{
        Anchor::{Top, X},
        CellRegistry, Container, LIGHT_BLUE, Label, Setup, TableData, TableView, View, ViewData,
        ViewSubviews, WHITE, view,
    },
    ui_test::{UITest, helpers::check_colors},
};

#[view]
struct LabelImage {
    resizing_image: bool,

    #[init]
    label:      Label,
    table_view: TableView,
    container:  Container,
}

impl Setup for LabelImage {
    fn setup(self: Weak<Self>) {
        self.label.set_text("ßšėčыў").set_text_size(110).set_image("cat.png");
        self.label.place().tl(50).w(400).h(200);

        self.table_view.set_data_source(self).register_cell::<Label>();
        self.table_view
            .place()
            .same([X], self.label)
            .anchor(Top, self.label, 40)
            .w(50)
            .h(200);
        self.table_view.set_color(LIGHT_BLUE);

        self.container.place().t(280).l(280).size(200, 200).all_ver();
        self.container.set_color(LIGHT_BLUE);

        self.container
            .add_view::<Label>()
            .set_text("test 1")
            .set_text_size(50)
            .set_text_color(WHITE)
            .set_image("cat.png");
        self.container
            .add_view::<Label>()
            .set_text("test 2")
            .set_text_size(50)
            .set_text_color(WHITE)
            .set_image("cat.png");
    }
}

impl TableData for LabelImage {
    fn cell_height(&self, _: usize) -> f32 {
        50.0
    }

    fn number_of_cells(&self) -> usize {
        4
    }

    fn setup_cell(&mut self, index: usize, registry: &mut CellRegistry) -> Weak<dyn View> {
        let mut cell = registry.cell::<Label>();
        cell.set_text(index);
        cell.set_text_size(50);
        cell.set_text_color(WHITE);
        if self.resizing_image {
            cell.set_resizing_image("button");
        } else {
            cell.set_image("cat.png");
        }
        cell
    }
}

pub async fn test_label_image() -> Result<()> {
    let mut view = UITest::start::<LabelImage>();

    check_colors(
        r#"
            4    4 -  89 124 149
            156   52 - 236 197 202
            316   52 - 222 182 183
            448   52 - 216 170 172
            84  108 -   0   0   0
            220  108 -   0   0   0
            152  136 - 233 205 194
            392  136 -   1   0   0
            268  140 - 188 161 134
            288  172 -   1   0   0
            76  176 -   0   0   0
            384  192 - 163 132 111
            448  220 - 181 149 128
            192  244 - 217 179 166
            356  244 - 167 132 110
            92  300 - 211 157 157
            392  312 - 255 255 255
            476  312 - 204 154 153
            56  320 - 244 217 208
            96  328 - 153 125 103
            336  332 - 255 255 254
            460  360 - 158 130 108
            96  364 - 201 151 150
            416  388 - 225 179 181
            56  420 - 244 217 208
            96  428 - 153 125 103
            368  428 - 255 255 255
            324  436 - 255 255 255
            472  456 - 155 127 106
            84  468 - 255 255 255
            224  592 -  89 124 149
            592  592 -  89 124 149
        "#,
    )?;

    from_main(move || {
        view.label.set_resizing_image("button");
        view.label.set_text_color(WHITE);
    });

    check_colors(
        r#"
            4    4 -  89 124 149
            592    4 -  89 124 149
            312   52 -   0  59 161
            332   52 -   0  57 161
            216   60 -   4  19  66
            60   92 -   5  19  64
            152  104 - 255 255 255
            232  132 - 255 255 255
            424  136 - 254 254 254
            332  148 - 254 254 254
            112  172 - 255 255 255
            256  204 -   4  19  66
            400  296 - 222 176 178
            472  308 - 207 153 153
            56  320 - 244 217 208
            96  328 - 153 125 103
            336  328 - 255 255 254
            428  348 - 198 165 146
            472  356 - 155 127 106
            96  364 - 201 151 150
            284  396 - 235 196 201
            388  396 - 225 179 181
            92  400 - 211 157 157
            476  404 - 209 159 158
            56  420 - 244 217 208
            96  428 - 153 125 103
            448  432 - 195 150 147
            96  464 - 201 151 150
            440  472 - 166 136 112
            312  476 - 229 195 186
            376  476 - 210 172 153
            592  592 -  89 124 149
        "#,
    )?;

    check_colors(
        r#"
            4    4 -  89 124 149
            592    4 -  89 124 149
            312   52 -   0  59 161
            332   52 -   0  57 161
            216   60 -   4  19  66
            60   92 -   5  19  64
            152  104 - 255 255 255
            232  132 - 255 255 255
            424  136 - 254 254 254
            332  148 - 254 254 254
            112  172 - 255 255 255
            256  204 -   4  19  66
            400  296 - 222 176 178
            472  308 - 207 153 153
            56  320 - 244 217 208
            96  328 - 153 125 103
            336  328 - 255 255 254
            428  348 - 198 165 146
            472  356 - 155 127 106
            96  364 - 201 151 150
            284  396 - 235 196 201
            388  396 - 225 179 181
            92  400 - 211 157 157
            476  404 - 209 159 158
            56  420 - 244 217 208
            96  428 - 153 125 103
            448  432 - 195 150 147
            96  464 - 201 151 150
            440  472 - 166 136 112
            312  476 - 229 195 186
            376  476 - 210 172 153
            592  592 -  89 124 149
        "#,
    )?;

    from_main(move || {
        view.resizing_image = true;
        view.table_view.reload_data();
    });

    check_colors(
        r#"
            4    4 -  89 124 149
            592    4 -  89 124 149
            312   52 -   0  59 161
            332   52 -   0  57 161
            188   60 -   5  20  68
            84   72 -   4  18  63
            400  108 - 255 255 255
            200  132 - 255 255 255
            316  132 - 255 255 255
            112  172 - 255 255 255
            264  196 -   4  18  65
            392  200 - 255 255 255
            96  292 -   0 218 255
            400  296 - 222 176 178
            472  308 - 207 153 153
            336  328 - 255 255 254
            72  344 -   5  18  65
            428  348 - 198 165 146
            472  356 - 155 127 106
            96  388 -   0 218 255
            52  396 -   0 218 255
            284  396 - 235 196 201
            388  396 - 225 179 181
            476  404 - 209 159 158
            448  432 - 195 150 147
            96  436 -   0 218 255
            52  440 -   0 218 255
            440  472 - 166 136 112
            312  476 - 229 195 186
            376  476 - 210 172 153
            52  488 -   0 218 255
            592  592 -  89 124 149
        "#,
    )?;

    Ok(())
}
