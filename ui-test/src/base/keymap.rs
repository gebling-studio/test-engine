use anyhow::Result;
use test_engine::{
    dispatch::on_main,
    refs::Own,
    ui::{UIManager, view},
    ui_test::{UITest, inject_key},
};

#[view]
struct Keymap {}

pub async fn test_keymap() -> Result<()> {
    let view = UITest::start::<Keymap>();

    let presses = Own::new(0);
    let mut pr = presses.weak();

    assert_eq!(*pr, 0);

    UIManager::keymap().add(view, 'g', move || {
        *pr += 1;
    });

    assert_eq!(*pr, 0);

    inject_key('a');
    assert_eq!(*pr, 0);

    inject_key('b');
    assert_eq!(*pr, 0);

    inject_key('c');
    assert_eq!(*pr, 0);

    inject_key('g');
    assert_eq!(*pr, 1);

    inject_key('g');
    assert_eq!(*pr, 2);

    UITest::start::<Keymap>();

    inject_key('g');
    assert_eq!(*pr, 2);

    inject_key('g');
    assert_eq!(*pr, 2);

    on_main(move || {
        drop(presses);
    });

    Ok(())
}
