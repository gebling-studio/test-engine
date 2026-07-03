use anyhow::Result;
use hreads::from_main;
use refs::Weak;

use crate::{
    ui::views::containers::table_view::tests::infinite_scroll::infinite_scroll::InfiniteScrollTest,
    ui_test::{check_colors, inject_scroll, inject_touches},
};

pub(super) fn test_basic_scroll(mut view: Weak<InfiniteScrollTest>) -> Result<()> {
    fn initial_check() -> Result<()> {
        check_colors(
            r"
                     388  110 -  89 124 149
                     375  159 -  85  85 170
                     373  227 -  85  85 170
                     382  318 -  85  85 170
                     385  434 -  51  51  51
                     306  461 -  89 124 149
                      10  512 -  89 124 149
                     127   59 -  89 124 149
                ",
        )
    }

    initial_check()?;

    inject_touches(
        "
            298  61   b
            298  61   e
        ",
    );

    inject_scroll(-400);
    initial_check()?;

    inject_touches(
        "
            311  548  b
            311  548  e
        ",
    );

    inject_scroll(-400);
    initial_check()?;

    from_main(move || {
        view.test_string.clear();
    });

    inject_touches(
        "
         208  165  b
         208  165  e
         383  171  b
         383  171  e
         204  234  b
         204  234  e
         420  242  b
         420  242  e
         204  321  b
         204  321  e
         393  328  b
         394  328  e
         216  401  b
         216  401  e
         400  398  b
         400  398  e
         238  469  b
         238  469  e
         418  467  b
         418  467  e
     ",
    );

    assert_eq!(view.test_string, "|0||1||2||3||4||5||6||7||8||9|");

    inject_touches(
        "
            293  301  b
            293  301  e
        ",
    );

    inject_scroll(-1000);

    from_main(move || {
        view.test_string.clear();
    });

    inject_touches(
        "
        232  139  b
        232  139  e
        350  139  b
        350  139  e
        221  194  b
        221  194  e
        408  202  b
        408  202  e
        219  274  b
        218  274  e
        405  284  b
        405  284  e
        136  361  b
        135  361  e
        399  362  b
        399  362  e
        217  430  b
        217  430  e
        459  454  b
        459  454  e
    ",
    );

    assert_eq!(view.test_string, "|24||25||26||27||28||29||30||31||32||33|");

    Ok(())
}
