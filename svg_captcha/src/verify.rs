/// Verifies if the provided clicks match the positions of the target icons.
///
/// 验证提供的点击是否匹配目标图标的位置。
pub fn verify(clicks: &[(i32, i32)], positions: &[(i32, i32, u32)]) -> bool {
    if clicks.len() != positions.len() {
        return false;
    }

    for (i, &(cx, cy)) in clicks.iter().enumerate() {
        let (px, py, sz) = positions[i];
        let padding = 15;
        if cx < px - padding
            || cx > px + sz as i32 + padding
            || cy < py - padding
            || cy > py + sz as i32 + padding
        {
            return false;
        }
    }

    true
}
