/// <reference types="node" />

/**
 * 异步生成验证码。
 *
 * @param w 宽度
 * @param h 高度
 * @param num 字符数量
 * @returns [WebP 图像的 Buffer 数组, SVG 图例数组, 坐标数组]
 */
export function captcha(
  w: number,
  h: number,
  num: number
): Promise<[Buffer, Array<string>, Array<[number, number, number]>]>
