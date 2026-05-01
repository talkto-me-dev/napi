const PADDING = 15

export function verify(clicks, positions) {
  if (clicks.length !== positions.length) {
    return false
  }
  for (let i = 0; i < clicks.length; i++) {
    const [cx, cy] = clicks[i]
    const [px, py, sz] = positions[i]
    if (
      cx < px - PADDING ||
      cx > px + sz + PADDING ||
      cy < py - PADDING ||
      cy > py + sz + PADDING
    ) {
      return false
    }
  }
  return true
}
