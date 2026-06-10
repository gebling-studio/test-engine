use std::{ops::Deref, sync::Arc};

use gm::{ToF32, flat::Rect};
use parking_lot::Mutex;

use super::Placer;
use crate::{
    View,
    layout::{Anchor, Tiling, layout_rule::LayoutRule},
    view::{ViewFrame, ViewSubviews},
};

impl Placer {
    pub fn size(&self, width: impl ToF32, height: impl ToF32) -> &Self {
        self.view.weak_view().set_size(width, height);
        self.w(width).h(height)
    }

    pub fn same_size(&self, view: impl Deref<Target = impl View> + Copy) -> &Self {
        self.relative(Anchor::Width, view, 1).relative(Anchor::Height, view, 1)
    }

    pub fn same_x(&self, view: impl Deref<Target = impl View>) -> &Self {
        self.anchor(Anchor::X, view, 1)
    }

    pub fn same_y(&self, view: impl Deref<Target = impl View>) -> &Self {
        self.anchor(Anchor::Y, view, 1)
    }

    pub fn same_width(&self, view: impl Deref<Target = impl View>) -> &Self {
        self.anchor(Anchor::Width, view, 1)
    }

    pub fn same_height(&self, view: impl Deref<Target = impl View>) -> &Self {
        self.anchor(Anchor::Height, view, 1)
    }

    pub fn relative_width(&self, view: impl Deref<Target = impl View>, multiplier: impl ToF32) -> &Self {
        self.relative(Anchor::Width, view, multiplier)
    }

    pub fn relative_height(&self, view: impl Deref<Target = impl View>, multiplier: impl ToF32) -> &Self {
        if !self.has().height {
            self.has().height = true;
            self.rules().insert(
                0,
                LayoutRule::relative(Anchor::Height, multiplier, view.weak_view()),
            );
            return self;
        }

        self.rules().retain(|r| !r.height());
        self.rules().insert(
            0,
            LayoutRule::relative(Anchor::Height, multiplier, view.weak_view()),
        );
        self
    }

    pub fn relative_size(
        &self,
        view: impl Deref<Target = impl View> + Copy,
        multiplier: impl ToF32,
    ) -> &Self {
        self.relative(Anchor::Width, view, multiplier)
            .relative(Anchor::Height, view, multiplier)
    }

    pub fn relative_x(&self, multiplier: impl ToF32) -> &Self {
        self.relative(Anchor::X, self.view.superview().deref(), multiplier)
    }

    pub fn relative_y(&self, multiplier: impl ToF32) -> &Self {
        self.relative(Anchor::Y, self.view.superview().deref(), multiplier)
    }

    pub fn same<const S: usize>(
        &self,
        anchors: [Anchor; S],
        view: impl Deref<Target = impl View> + Copy,
    ) -> &Self {
        for anchor in anchors {
            self.has().width = if anchor.is_width() { true } else { self.has().width };
            self.has().height = if anchor.is_height() {
                true
            } else {
                self.has().height
            };

            self.rules().push(LayoutRule::same(anchor, view.weak_view()));
        }
        self
    }

    pub fn w(&self, w: impl ToF32) -> &Self {
        if !self.has().width {
            self.rules().insert(0, LayoutRule::make(Anchor::Width, w));
            self.has().width = true;
            return self;
        }

        self.rules().retain(|r| !r.width());
        self.rules().insert(0, LayoutRule::make(Anchor::Width, w));
        self
    }

    pub fn h(&self, h: impl ToF32) -> &Self {
        if !self.has().height {
            self.rules().insert(0, LayoutRule::make(Anchor::Height, h));
            self.has().height = true;
            return self;
        }

        self.rules().retain(|r| !r.height());
        self.rules().insert(0, LayoutRule::make(Anchor::Height, h));
        self
    }

    pub fn center(&self) -> &Self {
        self.rules().push(Anchor::Center.into());
        self
    }

    pub fn center_x(&self) -> &Self {
        self.rules().push(Anchor::CenterX.into());
        self
    }

    pub fn center_y(&self) -> &Self {
        self.rules().push(Anchor::CenterY.into());
        self
    }

    pub fn back(&self) -> &Self {
        self.rules().push(Tiling::Background.into());
        self
    }

    pub fn left_half(&self) -> &Self {
        self.rules().push(Tiling::LeftHalf.into());
        self
    }

    pub fn right_half(&self) -> &Self {
        self.rules().push(Tiling::RightHalf.into());
        self
    }

    pub fn all_ver(&self) -> &Self {
        self.all_tiling_rules().push(Tiling::Vertically.into());
        self
    }

    pub fn all_hor(&self) -> &Self {
        self.all_tiling_rules().push(Tiling::Horizontally.into());
        self
    }

    pub fn distribute_ratio<const LEN: usize>(&self, ratios: [impl ToF32; LEN]) -> &Self {
        self.all_tiling_rules()
            .push(Tiling::Distribute(ratios.iter().map(|f| f.to_f32()).collect()).into());
        self
    }

    pub fn all(&self, margin: impl ToF32) -> &Self {
        *self.all_margin.borrow_mut() = margin.to_f32();
        self
    }
}

impl Placer {
    pub fn max_width(&self, w: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::MaxWidth, w));
        self
    }

    pub fn max_height(&self, h: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::MaxHeight, h));
        self
    }

    pub fn min_width(&self, w: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::MinWidth, w));
        self
    }

    pub fn min_height(&self, w: impl ToF32) -> &Self {
        self.rules().push(LayoutRule::make(Anchor::MinHeight, w));
        self
    }
}

impl Placer {
    pub fn anchor(
        &self,
        side: Anchor,
        view: impl Deref<Target = impl View + ?Sized>,
        offset: impl ToF32,
    ) -> &Self {
        assert_ne!(
            view.weak_view().raw(),
            self.view.weak_view().raw(),
            "Trying to anchor View to itself"
        );
        self.rules().push(LayoutRule::anchor(side, offset, view.weak_view()));
        self
    }

    pub fn relative(
        &self,
        side: Anchor,
        view: impl Deref<Target = impl View + ?Sized>,
        ratio: impl ToF32,
    ) -> &Self {
        assert_ne!(
            view.weak_view().raw(),
            self.view.weak_view().raw(),
            "Trying to assign relative View to itself"
        );

        let mut has = self.has();
        has.width = if side.is_width() { true } else { has.width };
        has.height = if side.is_height() { true } else { has.height };

        self.rules().push(LayoutRule::relative(side, ratio, view.weak_view()));
        self
    }
}

impl Placer {
    pub fn dump_rules(&self) {
        let rules = format!("{:?}", self.get_rules());
        let tiling_rules = format!("{:?}", self.get_tiling_rules());
        println!("Rules: {rules}\nAll tiling rules: {tiling_rules}");
    }
}

impl Placer {
    pub fn above(&self, view: impl Deref<Target = impl View> + Copy, offset: impl ToF32) -> &Self {
        self.same([Anchor::Width, Anchor::Height, Anchor::X], view)
            .anchor(Anchor::Bot, view, offset)
    }

    pub fn below(&self, view: impl Deref<Target = impl View> + Copy, offset: impl ToF32) -> &Self {
        self.same([Anchor::Width, Anchor::Height, Anchor::X], view)
            .anchor(Anchor::Top, view, offset)
    }

    pub fn at_right(&self, view: impl Deref<Target = impl View> + Copy, offset: impl ToF32) -> &Self {
        self.same([Anchor::Width, Anchor::Height, Anchor::CenterY], view)
            .anchor(Anchor::Left, view, offset)
    }

    pub fn between(
        &self,
        view_a: impl Deref<Target = impl View> + Copy,
        view_b: impl Deref<Target = impl View> + Copy,
    ) -> &Self {
        self.rules()
            .push(LayoutRule::between(view_a.weak_view(), view_b.weak_view()));
        self
    }

    pub fn between_super(&self, view: impl Deref<Target = impl View> + Copy, anchor: Anchor) -> &Self {
        self.rules().push(LayoutRule::between_super(view.weak_view(), anchor));
        self
    }

    pub fn custom(&self, custom: impl FnMut(&mut Rect) + Send + 'static) -> &Self {
        *self.custom.borrow_mut() = Some(Arc::new(Mutex::new(custom)));
        self
    }
}
