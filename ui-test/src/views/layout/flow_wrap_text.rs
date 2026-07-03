use anyhow::Result;
use test_engine::{
    dispatch::{from_main, wait_for_next_frame},
    refs::Weak,
    ui::{
        BLACK, Container, GREEN, Label, Rect, Setup, ViewData, ViewFrame, ViewSubviews, ViewTest, view_test,
    },
    ui_test::check_colors,
};

#[view_test]
struct FlowWrapText {
    words: Vec<Weak<Label>>,

    #[init]
    sentence: Container,
}

impl Setup for FlowWrapText {
    fn setup(self: Weak<Self>) {
        self.sentence.set_color(BLACK);
        self.sentence.place().tl(20).w(400).all(8).all_wrap();

        for word in "Grumpy wizards make toxic brew for the jovial queen".split(' ') {
            self.add_word(word);
        }
    }
}

impl FlowWrapText {
    fn add_word(mut self: Weak<Self>, text: &str) {
        let word = self.sentence.add_view::<Label>();
        word.set_color(GREEN);
        word.set_text(text).set_text_size(32);
        word.place().fit_text();
        self.words.push(word);
    }
}

fn check_flow(words: &[Rect], sentence: Rect, margin: f32) {
    assert!(
        words[0].x().abs() < 0.1 && words[0].y().abs() < 0.1,
        "first word is not at the origin: {:?}",
        words[0]
    );

    let mut rows = 0;
    let mut bottom: f32 = 0.0;

    for (i, word) in words.iter().enumerate() {
        assert!(
            word.max_x() <= sentence.width() + 0.5,
            "word {i} sticks out of the container: {word:?}"
        );

        bottom = bottom.max(word.max_y());

        if word.x().abs() < 0.1 {
            rows += 1;
        } else {
            let previous = &words[i - 1];
            assert!(
                (word.y() - previous.y()).abs() < 0.1,
                "word {i} is not on the row of its predecessor: {word:?} vs {previous:?}"
            );
            assert!(
                (word.x() - previous.max_x() - margin).abs() < 0.1,
                "word {i} does not follow its predecessor with the margin: {word:?} vs {previous:?}"
            );
        }
    }

    assert!(rows > 1, "the sentence did not wrap");
    assert!(
        (sentence.height() - bottom).abs() < 0.5,
        "container height {} does not match content bottom {bottom}",
        sentence.height()
    );
}

impl ViewTest for FlowWrapText {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        check_colors(
            r"
              56   24 -   0 255   0
             240   24 -   0 255   0
             332   24 -   0 255   0
             416   24 -   0   0   0
             380   28 -   0 255   0
             196   32 -   0 255   0
              92   40 -   0 255   0
             284   40 -   0 255   0
             152   44 -   0 255   0
              24   48 -   0 255   0
              64   56 -   0   0   0
             120   56 -   0   0   0
             316   56 -   0   0   0
             352   56 -   0   0   0
             244   68 -   0 255   0
             416   72 -   0   0   0
             160   80 -   0 255   0
              40   88 -   0 255   0
              84   88 -   0 255   0
             132   88 -   0 255   0
             192   88 -   0   0   0
             284   88 -   0 255   0
             332   88 -   0 255   0
             372   88 -   0   0   0
             592  108 -  89 124 149
             184  280 -  89 124 149
             592  288 -  89 124 149
               4  336 -  89 124 149
             320  412 -  89 124 149
              44  592 -  89 124 149
             240  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        let (words, sentence) = from_main(move || {
            let words: Vec<Rect> = view.words.iter().map(|w| *w.frame()).collect();
            (words, *view.sentence.frame())
        });

        check_flow(&words, sentence, 8.0);

        from_main(move || {
            for word in ["and", "jack", "quickly", "vexed", "the", "sphinx"] {
                view.add_word(word);
            }
        });

        wait_for_next_frame();

        check_colors(
            r"
              88   24 -   0 255   0
             176   24 -   0 255   0
             312   24 -   0 255   0
             364   24 -   0 255   0
             416   24 -   0   0   0
              28   32 -   0 255   0
             244   56 -   0   0   0
             160   72 -   0 255   0
             112   76 -   0 255   0
              56   80 -   0 255   0
             320   80 -   0 255   0
             372   96 -   0   0   0
             592   96 -  89 124 149
             416  116 -   0   0   0
             196  120 -   0 255   0
             240  124 -   0 255   0
             292  124 -   0 255   0
              24  128 -   0 255   0
             148  128 -   0 255   0
             104  156 -   0  13   0
              56  168 -   0 255   0
             180  168 -   0   0   0
             244  168 -   0   0   0
             344  168 -   0   0   0
             416  168 -   0   0   0
             592  344 -  89 124 149
               8  364 -  89 124 149
             296  464 -  89 124 149
               4  592 -  89 124 149
             180  592 -  89 124 149
             416  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        let (grown_words, grown_sentence) = from_main(move || {
            let words: Vec<Rect> = view.words.iter().map(|w| *w.frame()).collect();
            (words, *view.sentence.frame())
        });

        check_flow(&grown_words, grown_sentence, 8.0);

        assert!(
            grown_sentence.height() > sentence.height(),
            "container did not grow with new words: {} vs {}",
            grown_sentence.height(),
            sentence.height()
        );

        Ok(())
    }
}

pub async fn test_flow_wrap_text() -> Result<()> {
    run_ui_test()
}
