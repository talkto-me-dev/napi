use std::f32::consts::PI;

/// 5. 模拟 CRT 显像管波浪扭曲 (Sine Wave Warp)
pub fn apply(data: &mut [u8], w: usize, _h: usize, rng: &mut fastrand::Rng) {
    let amp = rng.i32(1..3);
    if amp > 0 {
        let phase = rng.usize(0..256);
        let freq_shift = rng.usize(0..2);

        let mut sin_lut = [0i32; 256];
        for (i, val) in sin_lut.iter_mut().enumerate() {
            *val = ((i as f32 * PI / 128.0).sin() * amp as f32) as i32;
        }

        for (y, row) in data.chunks_exact_mut(w * 4).enumerate() {
            let offset = unsafe { *sin_lut.get_unchecked(((y << freq_shift) + phase) % 256) };
            if offset != 0 {
                let shift_bytes = (offset.unsigned_abs() as usize) * 4;
                if offset > 0 {
                    row.rotate_right(shift_bytes);
                } else {
                    row.rotate_left(shift_bytes);
                }
            }
        }
    }
}
