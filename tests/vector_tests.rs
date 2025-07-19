use cyfrin_playground::vector::{init, two_last_elements};

#[test]
fn test_init() {
    let v = init(1, 2, 3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.get(0), Some(1).as_ref());
    assert_eq!(v.get(1), Some(2).as_ref());
    assert_eq!(v.get(2), Some(3).as_ref());
}

#[test]
fn test_two_last_elements() {
    let v = init(1, 2, 3);
    let last_two = two_last_elements(&v);
    assert_eq!(last_two.len(), 2);
    assert_eq!(last_two[0], 2);
    assert_eq!(last_two[1], 3);
}
