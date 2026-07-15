use anyhow::Result;
use test_engine::{
    dispatch::{from_main, wait_for_next_frame},
    refs::Weak,
    ui::{
        BLACK, BLUE, Container, GREEN, RED, Rect, Setup, ViewData, ViewFrame, ViewSubviews, ViewTest, WHITE,
        YELLOW, view,
    },
    ui_test::check_colors,
};

#[view]
struct FlowWrap {
    boxes: Vec<Weak<Container>>,

    #[init]
    flow: Container,
}

impl Setup for FlowWrap {
    fn setup(mut self: Weak<Self>) {
        self.flow.set_color(BLACK);
        self.flow.place().tl(20).w(300).all(10).all_wrap();

        for (width, height, color) in [
            (100, 50, RED),
            (120, 40, GREEN),
            (150, 60, BLUE),
            (80, 30, YELLOW),
            (340, 20, WHITE),
        ] {
            let container = self.flow.add_view::<Container>();
            container.set_color(color);
            container.place().size(width, height);
            self.boxes.push(container);
        }
    }
}

fn assert_frame(frame: Rect, expected: (f32, f32, f32, f32), name: &str) {
    let (x, y, width, height) = expected;
    assert!(
        (frame.x() - x).abs() < 0.1
            && (frame.y() - y).abs() < 0.1
            && (frame.width() - width).abs() < 0.1
            && (frame.height() - height).abs() < 0.1,
        "{name}: expected {expected:?}, got {frame:?}"
    );
}

impl ViewTest for FlowWrap {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        check_initial_wrap(view)?;
        check_wrap_after_hide(view)?;
        check_wrap_after_resize(view)?;

        Ok(())
    }
}

fn check_initial_wrap(view: Weak<FlowWrap>) -> Result<()> {
    check_colors(
        r"
              56   24 - 255   0   0
             116   24 - 255   0   0
             156   24 -   0 255   0
             208   24 -   0 255   0
             248   28 -   0 255   0
             104   56 - 255   0   0
             180   56 -   0 255   0
             316   60 -   0   0   0
             272   64 -   0   0   0
              72   68 - 255   0   0
              24   72 -   0   0   0
             184   84 - 255 255   0
             232   84 - 255 255   0
             116   92 -   0   0 231
             204  100 - 255 255   0
              60  108 -   0   0 231
             256  108 - 255 255   0
             312  116 -   0   0   0
              24  128 -   0   0 231
             120  136 -   0   0 231
             168  136 -   0   0 231
              76  148 -   0   0   0
             356  152 - 255 255 255
              36  168 - 255 255 255
             144  168 - 255 255 255
             224  168 - 255 255 255
             288  168 - 255 255 255
             300  300 -  89 124 149
             592  308 -  89 124 149
               4  592 -  89 124 149
             300  592 -  89 124 149
             592  592 -  89 124 149
            ",
    )?;

    let (frames, flow) = from_main(move || {
        let frames: Vec<Rect> = view.boxes.iter().map(|b| *b.frame()).collect();
        (frames, *view.flow.frame())
    });

    assert_frame(frames[0], (0.0, 0.0, 100.0, 50.0), "first in row");
    assert_frame(frames[1], (110.0, 0.0, 120.0, 40.0), "second in row");
    assert_frame(frames[2], (0.0, 60.0, 150.0, 60.0), "wrapped to second row");
    assert_frame(frames[3], (160.0, 60.0, 80.0, 30.0), "second row neighbor");
    assert_frame(
        frames[4],
        (0.0, 130.0, 340.0, 20.0),
        "oversized child got own row",
    );
    assert_frame(flow, (20.0, 20.0, 300.0, 150.0), "container sized to content");

    Ok(())
}

fn check_wrap_after_hide(view: Weak<FlowWrap>) -> Result<()> {
    from_main(move || {
        view.boxes[1].set_hidden(true);
    });

    wait_for_next_frame();

    check_colors(
        r"
             592    4 -  89 124 149
              56   24 - 255   0   0
             116   24 - 255   0   0
             224   24 -   0   0 231
             276   24 -   0   0 231
             316   28 -   0   0   0
              24   36 - 255   0   0
             112   60 - 255   0   0
             196   64 -   0   0 231
              80   68 - 255   0   0
             316   68 -   0   0   0
              24   72 -   0   0   0
             140   76 -   0   0 231
             260   76 -   0   0 231
              96   92 - 255 255   0
              72   96 - 255 255   0
              24  100 - 255 255   0
              48  112 - 255 255   0
             196  112 -   0   0   0
              92  116 - 255 255   0
             240  128 -   0   0   0
             316  128 -   0   0   0
              24  148 - 255 255 255
             124  148 - 255 255 255
             168  148 - 255 255 255
             280  148 - 255 255 255
             356  148 - 255 255 255
             300  300 -  89 124 149
             592  308 -  89 124 149
               4  592 -  89 124 149
             300  592 -  89 124 149
             592  592 -  89 124 149
            ",
    )?;

    let (frames, flow) = from_main(move || {
        let frames: Vec<Rect> = view.boxes.iter().map(|b| *b.frame()).collect();
        (frames, *view.flow.frame())
    });

    assert_frame(frames[0], (0.0, 0.0, 100.0, 50.0), "first after hide");
    assert_frame(frames[2], (110.0, 0.0, 150.0, 60.0), "moved up after hide");
    assert_frame(frames[3], (0.0, 70.0, 80.0, 30.0), "wrapped after hide");
    assert_frame(frames[4], (0.0, 110.0, 340.0, 20.0), "last row after hide");
    assert_frame(flow, (20.0, 20.0, 300.0, 130.0), "height follows hidden child");

    Ok(())
}

fn check_wrap_after_resize(view: Weak<FlowWrap>) -> Result<()> {
    from_main(move || {
        view.boxes[1].set_hidden(false);
        view.flow.place().w(500);
    });

    wait_for_next_frame();

    check_colors(
        r"
              56   24 - 255   0   0
             116   24 - 255   0   0
             248   24 -   0 255   0
             316   24 -   0   0 231
             408   24 -   0   0 231
             436   24 - 255 255   0
             472   24 - 255 255   0
             448   48 - 255 255   0
             496   48 - 255 255   0
             180   52 -   0 255   0
             140   56 -   0 255   0
             220   56 -   0 255   0
             368   56 -   0   0 231
              64   64 - 255   0   0
             104   68 - 255   0   0
             408   68 -   0   0 231
              24   72 -   0   0   0
             272   76 -   0   0 231
             328   88 -   0   0   0
             516   92 -   0   0   0
             180  100 - 255 255 255
              76  104 - 255 255 255
              36  108 - 255 255 255
             136  108 - 255 255 255
             224  108 - 255 255 255
             400  108 -   0   0   0
             472  108 -   0   0   0
             300  300 -  89 124 149
             592  336 -  89 124 149
               4  592 -  89 124 149
             300  592 -  89 124 149
             592  592 -  89 124 149
            ",
    )?;

    let (frames, flow) = from_main(move || {
        let frames: Vec<Rect> = view.boxes.iter().map(|b| *b.frame()).collect();
        (frames, *view.flow.frame())
    });

    assert_frame(frames[0], (0.0, 0.0, 100.0, 50.0), "first after resize");
    assert_frame(frames[1], (110.0, 0.0, 120.0, 40.0), "second after resize");
    assert_frame(frames[2], (240.0, 0.0, 150.0, 60.0), "third fits after resize");
    assert_frame(frames[3], (400.0, 0.0, 80.0, 30.0), "fourth fits after resize");
    assert_frame(
        frames[4],
        (0.0, 70.0, 340.0, 20.0),
        "wide child wrapped after resize",
    );
    assert_frame(flow, (20.0, 20.0, 500.0, 90.0), "container re-wrapped on resize");

    Ok(())
}
