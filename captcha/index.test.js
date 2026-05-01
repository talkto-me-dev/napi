import { test, expect } from 'vitest'
import captcha from './index.js'
import verify from './verify.js'

test('captcha and verify', async () => {
  const [webp, icons, positions] = await captcha(400, 300, 3)
  
  expect(webp instanceof Uint8Array).toBe(true)
  expect(icons.length).toBe(3)
  expect(positions.length).toBe(3)
  
  for (const pos of positions) {
    expect(pos.length).toBe(3)
  }

  // test verify
  const clicks = positions.map(([x, y, sz]) => [Math.round(x + sz/2), Math.round(y + sz/2)])
  expect(verify(clicks, positions)).toBe(true)

  const badClicks = positions.map(([x, y, sz]) => [x + sz + 100, y + sz + 100])
  expect(verify(badClicks, positions)).toBe(false)
}, 60000)
