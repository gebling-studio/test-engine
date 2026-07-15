use anyhow::Result;
use test_engine::{
    gm::Apply,
    refs::Weak,
    ui::{DropDown, Setup, ViewData, ViewTest, view},
    ui_test::{inject_touches, inject_touches_delayed, state::append_state},
};

#[view]
struct DropDownTestView {
    #[init]
    top: DropDown<&'static str>,
    bot: DropDown<&'static str>,
}

impl Setup for DropDownTestView {
    fn setup(mut self: Weak<Self>) {
        [self.top, self.bot].apply(|v| {
            v.on_changed(|val| {
                append_state(format!("{val}\n"));
            });
            v.place().center_x().size(200, 40);
        });

        self.top.place().t(5);
        self.bot.place().b(5);

        self.top.set_values(vec!["Dog", "Cat", "Sheep"]);
        self.bot.set_values(vec!["Car", "Boat", "Plane"]);
    }
}

impl ViewTest for DropDownTestView {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        assert_eq!(view.top.value(), &"Dog");
        assert_eq!(view.bot.value(), &"Car");

        inject_touches_delayed(
            r"
            334  35   b
            334  35   e
            322  68   b
            321  68   e
            352  585  b
            352  585  e
            326  536  b
            326  536  e
        ",
        );

        assert_eq!(view.top.value(), &"Cat");

        inject_touches(
            "
            228  32   b
            228  32   e

        ",
        );

        Ok(())
    }
}
