use cyfrin_playground::generic_type::{first, last};

#[test]
fn test_first() {
    assert_eq!(first(('a', false)), 'a');
    assert_eq!(first((1u32, 0i32)), 1u32);
}

#[test]
fn test_last() {
    assert_eq!(last(('a', "hello")), "hello");
    assert_eq!(last((1u32, 0i32)), 0i32);
}

#[test]
fn test_rectangle() {
    let rect = cyfrin_playground::generic_type::Rectangle {
        top: 10,
        left: 5,
        width: 20,
        height: 15,
    };
    assert_eq!(rect.top, 10);
    assert_eq!(rect.left, 5);
    assert_eq!(rect.width, 20);
    assert_eq!(rect.height, 15);

    let rect_f64 = cyfrin_playground::generic_type::Rectangle {
        top: 10.5,
        left: 5.5,
        width: 20.0,
        height: 15.0,
    };
    assert_eq!(rect_f64.top, 10.5);
    assert_eq!(rect_f64.left, 5.5);
    assert_eq!(rect_f64.width, 20.0);
    assert_eq!(rect_f64.height, 15.0);
}