use std::collections::BTreeSet;
use std::ops::Bound::*;

pub fn previous_elem<T: Ord>(tree: &BTreeSet<T>, val: T) -> Option<&T> {
    tree.range((Unbounded, Excluded(val))).next_back()
}

pub fn next_elem<T: Ord>(tree: &BTreeSet<T>, val: T) -> Option<&T> {
    tree.range((Excluded(val), Unbounded)).next()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn previous_elem_retrieval() {
        let mut btree_set = BTreeSet::new();

        btree_set.insert(32);
        btree_set.insert(40);
        btree_set.insert(18);

        assert_eq!(
            previous_elem(&btree_set, 39),
            Option::from(&32)
        );
        assert_eq!(previous_elem(&btree_set, 18), None);
        assert_eq!(
            previous_elem(&btree_set, 44),
            Option::from(&40)
        );
        assert_eq!(
            previous_elem(&btree_set, 32),
            Option::from(&18)
        );
    }

    #[test]
    fn next_elem_retrieval() {
        let mut btree_set = BTreeSet::new();

        btree_set.insert(32);
        btree_set.insert(40);
        btree_set.insert(18);

        assert_eq!(next_elem(&btree_set, 21), Option::from(&32));
        assert_eq!(next_elem(&btree_set, 32), Option::from(&40));
        assert_eq!(next_elem(&btree_set, 15), Option::from(&18));
        assert_eq!(next_elem(&btree_set, 40), None);
    }
}
