use ledger_dev_build_plane::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 84, capacity: 73, latency: 20, risk: 6, weight: 5 };
    assert_eq!(score(signal), 159);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 93, capacity: 79, latency: 26, risk: 14, weight: 10 };
    assert_eq!(score(signal), 129);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 88, capacity: 70, latency: 26, risk: 6, weight: 12 };
    assert_eq!(score(signal), 174);
    assert_eq!(classify(signal), "review");
}
