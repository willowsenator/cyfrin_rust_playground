pub fn mul(x: u32, y: u32) -> u32 {
    x * y
}

pub fn div(x: u32, y: u32) -> u32 {
    x / y
}

pub fn min(x: u32, y: u32) -> u32 {
    if x <= y { x } else { y }
}

pub fn max(x: u32, y: u32) -> u32 {
    if x >= y { x } else { y }
}
