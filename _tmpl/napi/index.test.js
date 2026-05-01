import { test, expect } from "vitest";
import _tmpl from "./index.js";

test("sum", () => {
  expect(_tmpl(1, 2)).toBe(3);
});
