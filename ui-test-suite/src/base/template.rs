use test_engine::{
    ui::{ui_test, view},
    ui_test::UITest,
};

#[view]
struct TemplateView<Value: 'static> {
    value: Value,
}

#[ui_test]
pub fn test_template() {
    let view = UITest::start::<TemplateView<i32>>();

    assert_eq!(view.value, 0);
}
