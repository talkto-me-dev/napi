import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
const binding = require('./_tmpl.node');
export const { _tmpl } = binding;
