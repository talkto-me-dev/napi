import { createRequire } from 'node:module'
import { platform, arch } from 'node:process'
import { existsSync } from 'node:fs'

const require = createRequire(import.meta.url),
  isMusl = () => existsSync('/lib/ld-musl-' + (arch === 'x64' ? 'x86_64' : 'aarch64') + '.so.1')

let binding
try {
  binding = require('./_tmpl.node')
} catch {
  const pkgName =
    '@3-/_tmpl-' +
    platform +
    '-' +
    arch +
    (platform === 'linux'
      ? isMusl()
        ? '-musl'
        : '-gnu'
      : platform === 'win32'
        ? '-msvc'
        : '')

  try {
    binding = require(pkgName)
  } catch (err) {
    throw new Error(
      'Unsupported OS: ' +
        platform +
        ', architecture: ' +
        arch +
        '. Failed to load native binding ' +
        pkgName +
        '. Error: ' +
        err.message
    )
  }
}

export default binding._tmpl
