use test_engine::{
    Event,
    generate::noise::{TerrainData, TerrainParams, generate_terrain},
    gm::LossyConvert,
    level::LevelManager,
    refs::{Own, Weak},
    ui::{
        AddLabel, Anchor,
        Anchor::{Top, X},
        BLACK, Button, DrawingView, Image, ImageView, Label, NumberView, Point, PointsPath, Setup, Size,
        TURQUOISE, ViewData, ViewTouch, WHITE, view,
    },
};

use crate::{
    interface::{polygon_view::PolygonView, test_game_view::HAS_BACK_BUTTON},
    levels::NoiseLevel,
};

#[view]
pub struct NoiseView {
    seed: u32,

    on_back: Event,

    islands: Vec<Vec<Point>>,

    #[init]
    drawing_view:   DrawingView,
    threshold_view: NumberView,
    x_view:         NumberView,
    y_view:         NumberView,
    size_view:      NumberView,
    skip_view:      NumberView,
    image_view:     ImageView,
    counter_label:  Label,
    update_level:   Button,
    polygon:        PolygonView,
}

impl NoiseView {
    fn update_image(mut self: Weak<Self>) {
        let resolution: Size<u32> = (100, 100).into();

        let (image, islands) = generate_image(TerrainParams {
            seed: self.seed,
            resolution,
            size: (self.size_view.value(), self.size_view.value()).into(),
            position: (self.x_view.value(), self.y_view.value()).into(),
            threshold: self.threshold_view.value().lossy_convert(),
            skip: self.skip_view.value().lossy_convert(),
        });

        self.counter_label.set_text(format!("{}", islands.len()));

        self.image_view.set_image(image);

        self.drawing_view.remove_all_paths();

        for island in &islands {
            self.drawing_view.add_path(island.iter().map(|a| *a * 20.0), BLACK);
        }

        self.islands = islands;

        self.drawing_view
            .add_path(PointsPath::circle_triangles_with((200, 100), 50, 5), TURQUOISE);
    }

    fn update_level(self: Weak<Self>) {
        LevelManager::downcast_level::<NoiseLevel>().add_islands(
            self.islands
                .iter()
                .map(|p| p.iter().map(|p| (p.x, -p.y).into()).collect())
                .collect(),
        );

        let biggest_size = self.islands.iter().map(Vec::len).max().unwrap();

        if biggest_size < 5 {
            return;
        }

        let smallest_island = self.islands.iter().find(|i| i.len() == biggest_size).unwrap().clone();

        self.polygon.display_points(smallest_island);
    }

    pub fn on_back(self: Own<Self>, callback: impl FnMut() + Send + 'static) -> Own<Self> {
        self.on_back.sub(callback);
        self
    }
}

impl Setup for NoiseView {
    fn setup(self: Weak<Self>) {
        const WIDTH: u32 = 100;

        LevelManager::set_level(NoiseLevel::default());

        self.drawing_view.place().back();

        self.enable_touch_low_priority();
        self.touch().up_inside.sub(self, move || self.update_image());

        let update_image = move |_| self.update_image();

        self.threshold_view
            .set_color(WHITE)
            .set_value(124.0)
            .set_step(2.0)
            .add_label("there")
            .on_change(update_image)
            .place()
            .size(WIDTH, 150)
            .bl(10);

        self.x_view
            .set_color(WHITE)
            .set_value(65.0)
            .set_step(0.5)
            .add_label("x")
            .on_change(update_image);
        self.x_view
            .place()
            .size(WIDTH, 150)
            .b(10)
            .anchor(Anchor::Left, self.threshold_view, 10);

        self.y_view
            .set_color(WHITE)
            .set_value(8.0)
            .set_step(0.5)
            .add_label("y")
            .on_change(update_image);
        self.y_view.place().size(WIDTH, 150).b(10).anchor(Anchor::Left, self.x_view, 10);

        self.size_view
            .set_color(WHITE)
            .set_value(6.0)
            .set_step(2.0)
            .add_label("size")
            .on_change(update_image);
        self.size_view
            .place()
            .size(WIDTH, 150)
            .b(10)
            .anchor(Anchor::Left, self.y_view, 10);

        self.skip_view
            .set_color(WHITE)
            .set_min(1.0)
            .set_step(1.0)
            .set_value(6.0)
            .add_label("size")
            .on_change(update_image);
        self.skip_view
            .place()
            .size(WIDTH, 150)
            .b(10)
            .anchor(Anchor::Left, self.size_view, 10);

        self.update_level.set_text("Level");
        self.update_level
            .place()
            .anchor(Top, self.counter_label, 10)
            .same([Anchor::Width, Anchor::Height, X], self.counter_label);
        self.update_level.on_tap(move || {
            self.update_level();
        });

        self.image_view.place().size(400, 400).br(0);

        self.apply_style(HAS_BACK_BUTTON);

        self.counter_label.place().t(200).r(5).size(100, 50);

        self.polygon.place().size(800, 800).center_x();

        self.update_image();
    }
}

fn generate_image(
    TerrainParams {
        seed,
        resolution,
        size,
        position,
        threshold,
        skip,
    }: TerrainParams,
) -> (Weak<Image>, Vec<Vec<Point>>) {
    let TerrainData { pixels, islands } = generate_terrain(TerrainParams {
        seed,
        resolution,
        size,
        position,
        threshold,
        skip,
    });

    let image_name = format!("noise_image_{seed}_{resolution}_{size}_{position}_{threshold}");

    (
        Image::from_raw_data(
            pixels,
            image_name,
            (resolution.width, resolution.height).into(),
            1,
        ),
        islands,
    )
}
