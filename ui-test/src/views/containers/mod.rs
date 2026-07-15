use crate::views::containers::movable_view::test_movable_view;

mod movable_view;

pub async fn test_containers() -> anyhow::Result<()> {
    use crate::run_test_unit;

    run_test_unit!(test_movable_view);
    Ok(())
}
