const PADDING = 15;

export default (clicks, positions) => {
  if (clicks.length !== positions.length) {
    return false;
  }
  for (let i = 0; i < clicks.length; ++i) {
    const [cx, cy] = clicks[i],
      [px, py, sz] = positions[i];
    if (
      cx < px - PADDING ||
      cx > px + sz + PADDING ||
      cy < py - PADDING ||
      cy > py + sz + PADDING
    ) {
      return false;
    }
  }
  return true;
};
