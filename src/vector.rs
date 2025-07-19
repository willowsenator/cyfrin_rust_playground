use std::vec;

pub fn init(x: u32, y: u32, z: u32) -> Vec<u32> {
    vec![x, y, z]
}

pub fn two_last_elements(v: &Vec<u32>) -> &[u32] {
    &v[v.len() - 2..]
}
