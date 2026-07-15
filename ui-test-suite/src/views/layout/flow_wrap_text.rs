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
                196   24 -   0 255   0
                240   24 -   0 255   0
                284   24 -   0 255   0
                328   24 -   0 255   0
                408   24 -   0 255   0
                116   28 -   0 255   0
                148   28 -   0 255   0
                48   32 -   0   0   0
                364   36 -   0 255   0
                80   40 -   0 255   0
                168   56 -   0   0   0
                208   56 -   0   0   0
                264   56 -   0   0   0
                304   56 -   0   0   0
                416   76 -   0   0   0
                104   80 -   0 255   0
                64   84 -   0 255   0
                24   88 -   0 255   0
                140   88 -   0   0   0
                184   88 -   0 255   0
                220   88 -   0 255   0
                284   88 -   0 255   0
                328   88 -   0 255   0
                368   88 -   0   0   0
                408  272 -  89 124 149
                592  304 -  89 124 149
                60  340 -  89 124 149
                300  424 -  89 124 149
                4  592 -  89 124 149
                200  592 -  89 124 149
                396  592 -  89 124 149
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
                128   24 -   0 255   0
                240   24 -   0 255   0
                300   24 -   0 255   0
                408   24 -   0 255   0
                48   32 -   0   0   0
                184   40 -   0 255   0
                352   48 -   0 255   0
                284   72 -   0 255   0
                44   76 -   0 255   0
                112   76 -   0 255   0
                416   80 -   0   0   0
                592   84 -  89 124 149
                228   96 -   0   0   0
                168  104 -   0 255   0
                368  104 -   0 255   0
                76  112 -   0   0   0
                28  120 -   0 255   0
                128  120 -   0 255   0
                292  124 -   0 255   0
                104  156 -   0   1   0
                52  164 -   0 255   0
                136  168 -   0   0   0
                220  168 -   0   0   0
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
