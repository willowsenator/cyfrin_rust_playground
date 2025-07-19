use cyfrin_playground::question::sum;

#[test]
fn test_sum() {
    let nums = vec!["1", "2", "3"];
    assert_eq!(sum(&nums), Ok(6));

    let nums: Vec<&'static str> = vec!["1", "2", "3", "a"];
    assert_eq!(
        sum(&nums),
        Err("Failed to parse string into u32".to_string())
    );
}
