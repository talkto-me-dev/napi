import { createRequire } from 'node:module'
import { platform, arch, report } from 'node:process'

const require = createRequire(import.meta.url)

const isMusl = () => {
  if (!report || typeof report.getReport !== 'function') {
    try {
      const { readFileSync } = require('node:fs')
      return readFileSync('/bin/ls', 'utf8').includes('libc.musl-') || readFileSync('/lib/libc.musl-x86_64.so.1', 'utf8').includes('libc.musl-')
    } catch {
      return true
    }
  } else {
    const { header } = report.getReport()
    return header && header.glibcVersionRuntime === null
  }
}

let binding
try {
  binding = require('./captcha.node')
} catch {
  const pkgName = `@3-/captcha-${platform}-${arch}${
    platform === 'linux' ? (isMusl() ? '-musl' : '-gnu') : platform === 'win32' ? '-msvc' : ''
  }`

  try {
    binding = require(pkgName)
  } catch (err) {
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}. Failed to load native binding ${pkgName}. Error: ${err.message}`)
  }
}

export const { captcha } = binding
