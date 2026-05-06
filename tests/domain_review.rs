use ledger_dev_build_plane::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 62, slack: 23, drag: 13, confidence: 82 };
    assert_eq!(review_score(case), 190);
    assert_eq!(review_lane(case), "ship");
}
