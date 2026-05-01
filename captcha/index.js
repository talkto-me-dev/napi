import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
const binding = require('./captcha.node');
export const { captcha } = binding;
