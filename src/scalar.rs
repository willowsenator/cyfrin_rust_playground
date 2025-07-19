pub fn eq(x: char, y: char) -> bool {
    x == y
}

pub fn add(x: f32, y: f32, z: f32) -> f32 {
    x + y + z
}

pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    let x = x as f32;
    let y = y as f32;

    x + y + z
}
