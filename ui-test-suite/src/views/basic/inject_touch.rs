use std::sync::atomic::{AtomicU16, Ordering};

use test_engine::{
    refs::Weak,
    ui::{Button, Setup, ViewData, ui_test, view},
    ui_test::{UITest, inject_touches},
};

static COUNTER: AtomicU16 = AtomicU16::new(0);

#[view]
struct InjectTouch {
    #[init]
    button: Button,
}

impl Setup for InjectTouch {
    fn setup(self: Weak<Self>) {
        self.button.place().size(200, 100);
        self.button.set_text("bress");
        self.button.on_tap(|| COUNTER.fetch_add(1, Ordering::Relaxed));
    }
}

#[ui_test]
pub fn test_inject_touch() {
    COUNTER.store(0, Ordering::Relaxed);

    UITest::start::<InjectTouch>();

    let mut touches = String::new();

    for _ in 0..100 {
        touches += r"
            5  5  b
            5  5  e
    ";
    }

    inject_touches(touches);

    assert_eq!(COUNTER.load(Ordering::Relaxed), 100);

    for _ in 0..10 {
        inject_touches(
            r"
            5  5  b
            5  5  e
    ",
        );
    }

    assert_eq!(COUNTER.load(Ordering::Relaxed), 110);
}
