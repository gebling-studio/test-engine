mod anchor_view;
mod parsing;
mod placer_view;

use anyhow::Result;

use crate::inspect::{
    anchor_view::test_anchor_view, parsing::test_inspect_parsing, placer_view::test_placer_view,
};

pub(crate) async fn test_inspect() -> Result<()> {
    use crate::run_test_unit;

    run_test_unit!(test_placer_view);
    run_test_unit!(test_anchor_view);
    run_test_unit!(test_inspect_parsing);

    Ok(())
}
