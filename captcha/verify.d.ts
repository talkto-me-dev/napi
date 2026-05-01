/**
 * 验证用户的点击坐标是否匹配验证码。
 *
 * @param clicks 用户点击的坐标数组
 * @param positions 验证码原始的目标坐标数组
 * @returns 是否通过验证
 */
export default function verify(
  clicks: Array<[number, number]>,
  positions: Array<[number, number, number]>
): boolean
