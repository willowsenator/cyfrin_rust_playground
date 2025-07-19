pub fn zeros() -> [u32; 100] {
    [0; 100]
}

pub fn first_3(s: &[u32]) -> &[u32] {
    if s.len() < 3 { &s[..] } else { &s[..3] }
}

pub fn last_3(s: &[u32]) -> &[u32] {
    if s.len() < 3 {
        &s[..]
    } else {
        &s[s.len() - 3..]
    }
}
