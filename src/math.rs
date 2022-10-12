// https://www.reddit.com/r/rust/comments/vdroh6/comment/iclx9d1/
pub fn quake_rsqrt(number: f32) -> f32 {
    let mut i: i32 = number.to_bits() as i32;
    i = 0x5F375A86_i32.wrapping_sub(i >> 1);
    let y = f32::from_bits(i as u32);
    y * (1.5 - (number * 0.5 * y * y))
}
