#[derive(Debug, PartialEq)]
pub enum MathError {
    DivByZero,
}

pub fn div(x: u32, y: u32) -> Result<u32, MathError> {
    match y {
        0 => Err(MathError::DivByZero),
        _ => Ok(x / y),
    }
}

pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
    match v.get(i) {
        Some(&value) => value,
        None => default_val,
    }
}
