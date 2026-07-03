use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{BLUE, Container, GREEN, Hover, RED, Setup, ViewData, ViewTest, ViewTouch, YELLOW, view_test},
    ui_test::{check_colors, inject_touches},
};

#[view_test]
struct HoverTest {
    log: Vec<(&'static str, bool)>,

    #[init]
    first:  Container,
    second: Container,
    top:    Container,
    ghost:  Container,
}

impl Setup for HoverTest {
    fn setup(self: Weak<Self>) {
        self.first.set_color(BLUE);
        self.first.place().tl(20).size(100, 100);
        self.first.enable_hover();
        self.first.touch().hovered.val(self, move |hovered| {
            let mut this = self;
            this.log.push(("first", hovered));
            this.first.set_color(if hovered { GREEN } else { BLUE });
        });

        self.second.set_color(RED);
        self.second.place().t(20).l(140).size(100, 100);
        self.second.enable_hover();
        self.second.touch().hovered.val(self, move |hovered| {
            let mut this = self;
            this.log.push(("second", hovered));
        });

        // Overlaps first. Registered later, so it wins the overlap.
        self.top.set_color(YELLOW);
        self.top.place().tl(40).size(60, 60);
        self.top.enable_hover();
        self.top.touch().hovered.val(self, move |hovered| {
            let mut this = self;
            this.log.push(("top", hovered));
        });

        self.ghost.place().t(300).l(20).size(100, 100);
        self.ghost.set_hidden(true);
        self.ghost.enable_hover();
        self.ghost.touch().hovered.val(self, move |hovered| {
            let mut this = self;
            this.log.push(("ghost", hovered));
        });
    }
}

impl ViewTest for HoverTest {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        // Cursor position and hover state can leak from previous tests.
        from_main(|| Hover::clear());
        from_main(move || {
            let mut this = view;
            this.log.clear();
        });

        inject_touches("30 30 m");

        from_main(move || {
            assert_eq!(view.log, vec![("first", true)]);
            assert!(view.first.is_hovered());
            assert!(!view.second.is_hovered());
        });

        check_colors(
            r"
             592    4 -  89 124 149
              24   24 -   0 255   0
             100   24 -   0 255   0
             144   24 - 255   0   0
             236   24 - 255   0   0
             192   36 - 255   0   0
              64   44 - 255 255   0
              88   44 - 255 255   0
              24   52 -   0 255   0
             116   64 -   0 255   0
              88   68 - 255 255   0
             224   68 - 255   0   0
              44   72 - 255 255   0
              68   72 - 255 255   0
             180   80 - 255   0   0
              72   92 - 255 255   0
              44   96 - 255 255   0
              96   96 - 255 255   0
             200  112 - 255   0   0
              24  116 -   0 255   0
              56  116 -   0 255   0
              80  116 -   0 255   0
             116  116 -   0 255   0
             160  116 - 255   0   0
             236  116 - 255   0   0
             444  152 -  89 124 149
             300  300 -  89 124 149
             592  300 -  89 124 149
              64  356 -  89 124 149
               4  592 -  89 124 149
             300  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        // Both first and top are under the cursor. Topmost wins,
        // exit fires before enter.
        inject_touches("70 70 m");

        from_main(move || {
            assert_eq!(view.log, vec![("first", true), ("first", false), ("top", true)]);
            assert!(!view.first.is_hovered());
            assert!(view.top.is_hovered());
        });

        check_colors(
            r"
             592    4 -  89 124 149
              24   24 -   0   0 231
             100   24 -   0   0 231
             144   24 - 255   0   0
             236   24 - 255   0   0
             192   36 - 255   0   0
              64   44 - 255 255   0
              88   44 - 255 255   0
              24   52 -   0   0 231
             116   64 -   0   0 231
              88   68 - 255 255   0
             224   68 - 255   0   0
              44   72 - 255 255   0
              68   72 - 255 255   0
             180   80 - 255   0   0
              72   92 - 255 255   0
              44   96 - 255 255   0
              96   96 - 255 255   0
             200  112 - 255   0   0
              24  116 -   0   0 231
              56  116 -   0   0 231
              80  116 -   0   0 231
             116  116 -   0   0 231
             160  116 - 255   0   0
             236  116 - 255   0   0
             444  152 -  89 124 149
             300  300 -  89 124 149
             592  300 -  89 124 149
              64  356 -  89 124 149
               4  592 -  89 124 149
             300  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        inject_touches("170 70 m");

        from_main(move || {
            assert_eq!(
                view.log,
                vec![
                    ("first", true),
                    ("first", false),
                    ("top", true),
                    ("top", false),
                    ("second", true)
                ]
            );
            assert!(view.second.is_hovered());
        });

        // Empty space. The hovered view gets an exit.
        inject_touches("350 70 m");

        from_main(move || {
            assert_eq!(*view.log.last().unwrap(), ("second", false));
            assert!(!view.second.is_hovered());
        });

        // A hidden view never hovers.
        let len_before_ghost = from_main(move || view.log.len());
        inject_touches("70 330 m");

        from_main(move || {
            assert_eq!(view.log.len(), len_before_ghost);
            assert!(!view.ghost.is_hovered());
        });

        // Cursor leaving the window clears hover.
        inject_touches("30 30 m");
        from_main(|| Hover::clear());

        from_main(move || {
            assert_eq!(*view.log.last().unwrap(), ("first", false));
            assert!(!view.first.is_hovered());
        });

        Ok(())
    }
}

pub async fn test_hover() -> Result<()> {
    run_ui_test()
}
