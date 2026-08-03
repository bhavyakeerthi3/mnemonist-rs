//! Port of mnemonist set.js utility tests (tests/original/set.js).

use indexmap::IndexSet;
use mnemonist::set_ops;
use serde_json::{json, Value};

fn set(values: impl IntoIterator<Item = Value>) -> IndexSet<Value> {
    values.into_iter().collect()
}

#[test]
fn intersection_two_sets() {
    let a = set([json!(1), json!(2), json!(3)]);
    let b = set([json!(2), json!(3), json!(4)]);
    let i = set_ops::intersection(&[a, b]).unwrap();
    assert_eq!(
        i.iter().cloned().collect::<Vec<_>>(),
        vec![json!(2), json!(3)]
    );
}

#[test]
fn intersection_variadic() {
    let a = set([json!(1), json!(2), json!(3), json!(4)]);
    let b = set([json!(2), json!(3), json!(4)]);
    let c = set([json!(1), json!(4)]);
    let d = set([json!(4), json!(5), json!(6)]);
    let i = set_ops::intersection(&[a, b, c, d]).unwrap();
    assert_eq!(i.iter().cloned().collect::<Vec<_>>(), vec![json!(4)]);
}

#[test]
fn union_two_sets() {
    let a = set([json!(1), json!(2), json!(3)]);
    let b = set([json!(2), json!(3), json!(4)]);
    let u = set_ops::union(&[a, b]).unwrap();
    assert_eq!(
        u.iter().cloned().collect::<Vec<_>>(),
        vec![json!(1), json!(2), json!(3), json!(4)]
    );
}

#[test]
fn union_variadic() {
    let a = set([json!(1), json!(2), json!(3), json!(4)]);
    let b = set([json!(2), json!(3), json!(4)]);
    let c = set([json!(1), json!(4)]);
    let d = set([json!(4), json!(5), json!(6)]);
    let u = set_ops::union(&[a, b, c, d]).unwrap();
    assert_eq!(
        u.iter().cloned().collect::<Vec<_>>(),
        vec![json!(1), json!(2), json!(3), json!(4), json!(5), json!(6)]
    );
}

#[test]
fn difference() {
    let a = set([json!(1), json!(2), json!(3), json!(4), json!(5)]);
    let b = set([json!(2), json!(3)]);
    let d = set_ops::difference(&a, &b);
    assert_eq!(
        d.iter().cloned().collect::<Vec<_>>(),
        vec![json!(1), json!(4), json!(5)]
    );
}

#[test]
fn symmetric_difference() {
    let a = set([json!(1), json!(2), json!(3)]);
    let b = set([json!(3), json!(4), json!(5)]);
    let s = set_ops::symmetric_difference(&a, &b);
    assert_eq!(
        s.iter().cloned().collect::<Vec<_>>(),
        vec![json!(1), json!(2), json!(4), json!(5)]
    );
}

#[test]
fn is_subset_and_superset() {
    let a = set([json!(1), json!(2)]);
    let b = set([json!(1), json!(2), json!(3)]);
    let c = set([json!(2), json!(4)]);
    assert!(set_ops::is_subset(&a, &b));
    assert!(!set_ops::is_subset(&c, &b));
    assert!(set_ops::is_superset(&b, &a));
    assert!(!set_ops::is_superset(&b, &c));
}

#[test]
fn mutating_ops() {
    let mut a = set([json!(1), json!(2)]);
    set_ops::add(&mut a, &set([json!(2), json!(3)]));
    assert_eq!(
        a.iter().cloned().collect::<Vec<_>>(),
        vec![json!(1), json!(2), json!(3)]
    );

    let mut a = set([json!(1), json!(2)]);
    set_ops::subtract(&mut a, &set([json!(2), json!(3)]));
    assert_eq!(a.iter().cloned().collect::<Vec<_>>(), vec![json!(1)]);

    let mut a = set([json!(1), json!(2)]);
    set_ops::intersect(&mut a, &set([json!(2), json!(3)]));
    assert_eq!(a.iter().cloned().collect::<Vec<_>>(), vec![json!(2)]);

    let mut a = set([json!(1), json!(2)]);
    set_ops::disjunct(&mut a, &set([json!(2), json!(3)]));
    assert_eq!(
        a.iter().cloned().collect::<Vec<_>>(),
        vec![json!(1), json!(3)]
    );
}

#[test]
fn sizes_and_metrics() {
    let a = set([json!(1), json!(2), json!(3)]);
    let b = set([json!(2), json!(3), json!(4)]);
    let empty: IndexSet<Value> = IndexSet::new();

    assert_eq!(set_ops::intersection_size(&a, &b), 2);
    assert_eq!(set_ops::intersection_size(&a, &empty), 0);
    assert_eq!(set_ops::union_size(&a, &b), 4);
    assert_eq!(set_ops::union_size(&a, &empty), 3);
    assert_eq!(set_ops::jaccard(&a, &b), 0.5);
    assert_eq!(set_ops::jaccard(&a, &empty), 0.0);

    let contact = set_ops::chars_set("contact");
    let context = set_ops::chars_set("context");
    assert!((set_ops::jaccard(&contact, &context) - 4.0 / 7.0).abs() < f64::EPSILON);
    assert!((set_ops::overlap(&contact, &context) - 4.0 / 5.0).abs() < f64::EPSILON);
}
