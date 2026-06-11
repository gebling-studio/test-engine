#![feature(test)]

extern crate test;

use refs::Own;
use test::{Bencher, black_box};
use ui::UIEvent;

fn bench_trigger(bencher: &mut Bencher, subscribers: usize) {
    hreads::set_current_thread_as_main();

    let event: UIEvent<usize> = UIEvent::default();
    let owners: Vec<Own<usize>> = (0..subscribers).map(Own::new).collect();

    for owner in &owners {
        event.val(owner.weak(), |value| {
            black_box(value);
        });
    }

    bencher.iter(|| event.trigger(black_box(1)));
}

#[bench]
fn trigger_1_subscriber(bencher: &mut Bencher) {
    bench_trigger(bencher, 1);
}

#[bench]
fn trigger_4_subscribers(bencher: &mut Bencher) {
    bench_trigger(bencher, 4);
}

#[bench]
fn trigger_32_subscribers(bencher: &mut Bencher) {
    bench_trigger(bencher, 32);
}
