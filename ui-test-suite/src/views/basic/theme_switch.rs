use anyhow::Result;
use test_engine::{
    dispatch::{from_main, wait_for_next_frame},
    refs::Weak,
    ui::{
        BLACK, BLUE, Color, Container, DynamicColor, Label, RED, Setup, Theme, ThemeMode, UIEvents, ViewData,
        ViewSubviews, ViewTest, WHITE, YELLOW, view_test,
    },
    ui_test::check_colors,
};

const BACKGROUND: DynamicColor = DynamicColor::new(WHITE, Color::rgb(0.2, 0.2, 0.2));
const BORDER: DynamicColor = DynamicColor::new(BLUE, YELLOW);
const TEXT: DynamicColor = DynamicColor::new(BLACK, WHITE);

#[view_test]
struct ThemeSwitch {
    switches: Vec<Theme>,
    added:    Weak<Container>,

    #[init]
    themed: Container,
    plain:  Container,
    label:  Label,
}

impl Setup for ThemeSwitch {
    fn setup(self: Weak<Self>) {
        self.themed.set_color(BACKGROUND).set_border_color(BORDER).set_border_width(10);
        self.themed.place().tl(20).size(200, 200);

        self.plain.set_color(RED);
        self.plain.place().t(20).l(240).size(100, 100);

        self.label.set_color(BACKGROUND);
        self.label.set_text("Theme").set_text_size(32).set_text_color(TEXT);
        self.label.place().t(240).l(20).size(200, 50);

        UIEvents::theme_changed().val(self, move |theme| {
            let mut this = self;
            this.switches.push(theme);
        });
    }
}

fn check_initial_light_theme(view: Weak<ThemeSwitch>) -> Result<()> {
    // In human mode the OS theme may be dark. Start from a known state.
    from_main(move || {
        Theme::set_mode(ThemeMode::System);
        Theme::set_system(Theme::Light);
        let mut this = view;
        this.switches.clear();
    });

    wait_for_next_frame();

    check_colors(
        r"
             592    4 -  89 124 149
              88   24 -   0   0 231
             148   24 -   0   0 231
             288   24 - 255   0   0
             336   24 - 255   0   0
              32   32 - 255 255 255
             244   32 - 255   0   0
             304   64 - 255   0   0
             160   84 - 255 255 255
             260   88 - 255   0   0
             336   88 - 255   0   0
              96   92 - 255 255 255
              24  100 -   0   0 231
             308  116 - 255   0   0
             212  120 -   0   0 231
              84  144 - 255 255 255
              32  164 - 255 255 255
             128  172 - 255 255 255
             216  204 -   0   0 231
             164  212 -   0   0 231
              76  216 -   0   0 231
             116  268 - 255 255 255
             160  268 - 255 255 255
             592  268 -  89 124 149
              24  288 - 255 255 255
             216  288 - 255 255 255
             388  336 -  89 124 149
             532  428 -  89 124 149
             132  452 -  89 124 149
             300  532 -  89 124 149
               4  592 -  89 124 149
             592  592 -  89 124 149
            ",
    )?;

    from_main(move || {
        assert_eq!(Theme::current(), Theme::Light);
        assert_eq!(*view.themed.color(), WHITE);
        assert_eq!(*view.themed.border_color(), BLUE);
        assert_eq!(*view.label.text_color(), BLACK);
        assert_eq!(*view.plain.color(), RED);
    });

    Ok(())
}

fn check_dark_theme(view: Weak<ThemeSwitch>) -> Result<()> {
    from_main(|| Theme::set_system(Theme::Dark));
    wait_for_next_frame();

    check_colors(
        r"
             580    4 -  89 124 149
              24   24 - 255 255   0
             152   24 - 255 255   0
             244   24 - 255   0   0
             336   24 - 255   0   0
              88   28 - 255 255   0
             288   28 - 255   0   0
             248   68 - 255   0   0
             292   72 - 255   0   0
             336   72 - 255   0   0
              28   96 - 255 255   0
             268   96 - 255   0   0
             244  116 - 255   0   0
             292  116 - 255   0   0
             336  116 - 255   0   0
             492  132 -  89 124 149
             212  152 - 255 255   0
              24  164 - 255 255   0
             216  200 - 255 255   0
              76  212 - 255 255   0
             140  212 - 255 255   0
              24  216 - 255 255   0
             592  248 -  89 124 149
             116  268 - 124 124 124
             160  268 - 124 124 124
             392  332 -  89 124 149
             548  420 -  89 124 149
             132  452 -  89 124 149
             300  532 -  89 124 149
               4  592 -  89 124 149
             440  592 -  89 124 149
             592  592 -  89 124 149
            ",
    )?;

    from_main(move || {
        assert_eq!(Theme::current(), Theme::Dark);
        assert_eq!(*view.themed.color(), BACKGROUND.dark);
        assert_eq!(*view.themed.border_color(), YELLOW);
        assert_eq!(*view.label.text_color(), WHITE);
        assert_eq!(*view.plain.color(), RED);
        assert_eq!(view.switches, vec![Theme::Dark]);
    });

    Ok(())
}

// A view created while dark resolves against the dark theme.
fn check_view_added_in_dark(view: Weak<ThemeSwitch>) {
    from_main(move || {
        let mut this = view;
        let added = this.add_view::<Container>();
        added.set_color(BACKGROUND);
        added.place().t(140).l(240).size(100, 100);
        this.added = added;
    });

    wait_for_next_frame();

    from_main(move || {
        assert_eq!(*view.added.color(), BACKGROUND.dark);
    });
}

// Forced light wins over the dark system theme.
fn check_forced_light(view: Weak<ThemeSwitch>) -> Result<()> {
    from_main(|| Theme::set_mode(ThemeMode::Light));
    wait_for_next_frame();

    check_colors(
        r"
             496    4 -  89 124 149
              88   24 -   0   0 231
             288   24 - 255   0   0
             336   24 - 255   0   0
              32   32 - 255 255 255
             144   32 - 255 255 255
             244   32 - 255   0   0
             304   64 - 255   0   0
             260   88 - 255   0   0
             336   88 - 255   0   0
             104   96 - 255 255 255
              28  100 -   0   0 231
             308  116 - 255   0   0
             212  120 -   0   0 231
             592  132 -  89 124 149
              24  168 -   0   0 231
             124  168 - 255 255 255
             284  184 - 255 255 255
             216  204 -   0   0 231
             160  212 -   0   0 231
              72  216 -   0   0 231
             336  236 - 255 255 255
             116  268 - 255 255 255
             160  268 - 255 255 255
              24  288 - 255 255 255
             216  288 - 255 255 255
             536  360 -  89 124 149
             376  392 -  89 124 149
             132  452 -  89 124 149
             300  532 -  89 124 149
               4  592 -  89 124 149
             592  592 -  89 124 149
            ",
    )?;

    from_main(move || {
        assert_eq!(Theme::current(), Theme::Light);
        assert_eq!(*view.themed.color(), WHITE);
        assert_eq!(*view.themed.border_color(), BLUE);
        assert_eq!(*view.label.text_color(), BLACK);
        assert_eq!(*view.added.color(), WHITE);
        assert_eq!(view.switches, vec![Theme::Dark, Theme::Light]);
    });

    Ok(())
}

// Back to following the system, which is still dark.
fn check_back_to_system(view: Weak<ThemeSwitch>) {
    from_main(|| Theme::set_mode(ThemeMode::System));
    wait_for_next_frame();

    from_main(move || {
        assert_eq!(Theme::current(), Theme::Dark);
        assert_eq!(*view.themed.color(), BACKGROUND.dark);
        assert_eq!(view.switches, vec![Theme::Dark, Theme::Light, Theme::Dark]);
    });

    // Leave the default state for the tests that follow.
    from_main(|| {
        Theme::set_system(Theme::Light);
        assert_eq!(Theme::current(), Theme::Light);
    });
}

impl ViewTest for ThemeSwitch {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        check_initial_light_theme(view)?;
        check_dark_theme(view)?;
        check_view_added_in_dark(view);
        check_forced_light(view)?;
        check_back_to_system(view);

        Ok(())
    }
}
