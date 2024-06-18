// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>; // associated type - need to also define Item type

//     // methods with default implementations elided
// }

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter(); // needs to be mutable because state is mutated to track progress for loop obsfucates this.

    assert_eq!(v1_iter.next(), Some(&1)); // references from .next() are immutable however
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    // for an iterator with ownership use into_iter, for mutable references use iter_mut
}