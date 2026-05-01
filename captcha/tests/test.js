#!/usr/bin/env bun
import assert from "node:assert";
import { captcha } from "@3-/captcha";
import verify from "@3-/captcha/verify.js";

console.log("测试 @3-/captcha...");

const [webp, icons, positions] = await captcha(400, 300, 3);

assert.ok(webp instanceof Uint8Array, "webp 应该是 Uint8Array");
assert.strictEqual(icons.length, 3, "图标数量应为 3");
assert.strictEqual(positions.length, 3, "位置数量应为 3");

for (const pos of positions) {
  assert.strictEqual(pos.length, 3, "每个位置应包含 3 个坐标 (x, y, sz)");
}

const clicks = positions.map(([x, y, sz]) => [Math.round(x + sz / 2), Math.round(y + sz / 2)]),
  bad_clicks = positions.map(([x, y, sz]) => [x + sz + 100, y + sz + 100]);

assert.strictEqual(verify(clicks, positions), true, "正确的点击验证应返回 true");
assert.strictEqual(verify(bad_clicks, positions), false, "错误的点击验证应返回 false");

console.log("所有测试通过。");
