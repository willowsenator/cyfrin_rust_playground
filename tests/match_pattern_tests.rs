use cyfrin_playground::match_pattern::{num_to_string, unwrap_or_default, unwrap_result};

#[test]
fn test_num_to_string() {
    assert_eq!(num_to_string(0), "zero");
    assert_eq!(num_to_string(1), "one");
    assert_eq!(num_to_string(2), "two");
    assert_eq!(num_to_string(3), "three");
    assert_eq!(num_to_string(4), "other");
    assert_eq!(num_to_string(5), "other");
}

#[test]
fn test_unwrap_or_default() {
    assert_eq!(unwrap_or_default(Some(1), 0), 1);
    assert_eq!(unwrap_or_default(Some(2), 0), 2);
    assert_eq!(unwrap_or_default(None, 0), 0);
}

#[test]
fn test_unwrap_result() {
    assert_eq!(unwrap_result(Ok(42), 0), 42);
    assert_eq!(unwrap_result(Err("error".to_string()), 0), 0);
    assert_eq!(unwrap_result(Err("another error".to_string()), 100), 100);
}
