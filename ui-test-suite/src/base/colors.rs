use anyhow::Result;
use test_engine::{
    gm::Apply,
    refs::Weak,
    ui::{
        Anchor::{Height, Left, Top, Width, X},
        Container, ImageView, Setup, ViewData, ViewTest, WHITE, view,
    },
    ui_test::check_colors,
};

#[view]
struct Colors {
    #[init]
    image: ImageView,

    _1: Container,
    _2: Container,
    _3: Container,
    _4: Container,
}

impl Setup for Colors {
    fn setup(self: Weak<Self>) {
        self.set_color(WHITE);

        self.image.place().tl(20).size(280, 520);
        self.image.set_image("colors.png");

        self._1.set_color((45, 70, 149));
        self._1.place().size(100, 100).t(45).anchor(Left, self.image, 20);

        [self._2, self._3, self._4].apply(|view| {
            view.place().same([Width, Height, X], self._1);
        });

        self._2.set_color((48, 48, 48));
        self._2.place().anchor(Top, self._1, 25);

        self._3.set_color((124, 190, 22));
        self._3.place().anchor(Top, self._2, 25);

        self._4.set_color((172, 71, 212));
        self._4.place().anchor(Top, self._3, 25);
    }
}

impl ViewTest for Colors {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        check_colors(
            r"
            592    4 - 255 255 255
            192   48 -  45  70 149
            396   52 -  45  70 149
            56   92 - 255 255 255
            276   92 -  45  70 149
            348  108 -  45  70 149
            180  128 -  45  70 149
            368  176 -  48  48  48
            240  180 -  48  48  48
            584  196 - 255 255 255
            64  228 - 255 255 255
            372  244 -  48  48  48
            260  256 -  48  48  48
            184  268 -  48  48  48
            324  296 - 124 190  22
            416  296 - 124 190  22
            216  328 - 124 190  22
            116  348 - 255 255 255
            116  352 - 255 255 255
            272  368 - 124 190  22
            592  388 - 255 255 255
            376  392 - 124 190  22
            192  396 - 124 190  22
            4  444 - 255 255 255
            336  448 - 172  71 212
            260  452 - 172  71 212
            180  484 - 172  71 212
            416  484 - 172  71 212
            324  516 - 172  71 212
            240  520 - 172  71 212
            4  592 - 255 255 255
            592  592 - 255 255 255
        ",
        )?;

        Ok(())
    }
}
