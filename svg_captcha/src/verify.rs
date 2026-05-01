/// Click tolerance in pixels.
///
/// 点击容差（像素）。
const PADDING: i32 = 15;

/// Verifies if the provided clicks match the positions of the target icons.
///
/// 验证提供的点击是否匹配目标图标的位置。
pub fn verify(clicks: &[(i32, i32)], positions: &[(i32, i32, u32)]) -> bool {
    clicks.len() == positions.len()
        && clicks
            .iter()
            .zip(positions)
            .all(|(&(cx, cy), &(px, py, sz))| {
                let sz = sz as i32;
                cx >= px - PADDING
                    && cx <= px + sz + PADDING
                    && cy >= py - PADDING
                    && cy <= py + sz + PADDING
            })
}
