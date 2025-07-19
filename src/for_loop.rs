pub fn sum(nums: Vec<i32>) -> i32 {
    nums.iter().sum()
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(i);
    }
    v
}
