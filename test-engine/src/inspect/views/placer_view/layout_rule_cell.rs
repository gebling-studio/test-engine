use gm::{LossyConvert, color::LIGHT_GRAY};
use refs::{Rglica, ToRglica, Weak};
use ui::{CheckBox, LayoutRule, Setup, TextField, UIEvent, ViewData, ViewFrame};
use ui_proc::view;

use crate::inspect::views::AnchorView;

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate::ui;
}

#[view]
pub struct LayoutRuleCell {
    pub editing_ended: UIEvent,

    rule: Rglica<LayoutRule>,

    #[init]
    anchor:  AnchorView,
    value:   TextField,
    enabled: CheckBox,
}

impl Setup for LayoutRuleCell {
    fn setup(mut self: Weak<Self>) {
        self.anchor.place().l(5).center_y().custom(move |frame| {
            let height = self.height() * 0.8;
            frame.size = (height, height).into();
        });

        self.value.steal_appearance(self.enabled);
        self.value.set_text_color(LIGHT_GRAY).set_text_size(20).integer_only();
        let selected_color = *self.value.color();
        self.value.set_selected_color(selected_color.increase_by(0.05));

        self.value.place().at_right(self.anchor, 8).w(88).relative_height(self, 0.6);
        self.value.editing_ended.val(move |val| {
            let new_val: f32 = val.parse().unwrap();
            self.rule.set_offset(new_val);
            self.editing_ended.trigger(());
        });

        self.enabled.place().at_right(self.value, 8).size(28, 28);
        self.enabled.on_change(move |on| {
            self.rule.enabled = on;
        });
    }
}

impl LayoutRuleCell {
    pub fn set_rule(mut self: Weak<Self>, rule: &LayoutRule) {
        if let Some(anchor) = rule.side() {
            self.anchor.set_anchor(anchor);
            self.value.set_text(LossyConvert::<i64>::lossy_convert(rule.offset()));
            self.enabled.set_on(rule.enabled);
        } else {
            dbg!(&rule);
        }

        self.rule = rule.to_rglica();
    }
}
