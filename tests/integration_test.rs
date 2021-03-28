use std::vec;

use hyperid::{HyperId};

#[test]
fn should_generate_always_different_ids() {
    let mut all_ids = vec![];
    let mut hyperid = HyperId::default();

    for _ in 0..1024 {
        let id = hyperid.generate();
        assert_eq!(false, all_ids.contains(&id));
        all_ids.push(id);
    }
}