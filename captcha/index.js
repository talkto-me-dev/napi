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
  const isWindows = platform === 'win32',
    isMac = platform === 'darwin',
    isLinux = platform === 'linux',
    isX64 = arch === 'x64',
    isArm64 = arch === 'arm64'

  let pkgName = ''
  if (isWindows && isX64) pkgName = '@3-/captcha-win32-x64-msvc'
  else if (isWindows && isArm64) pkgName = '@3-/captcha-win32-arm64-msvc'
  else if (isMac && isX64) pkgName = '@3-/captcha-darwin-x64'
  else if (isMac && isArm64) pkgName = '@3-/captcha-darwin-arm64'
  else if (isLinux && isX64) pkgName = isMusl() ? '@3-/captcha-linux-x64-musl' : '@3-/captcha-linux-x64-gnu'
  else if (isLinux && isArm64) pkgName = isMusl() ? '@3-/captcha-linux-arm64-musl' : '@3-/captcha-linux-arm64-gnu'

  if (!pkgName) {
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
  }
  
  binding = require(pkgName)
}

export const { captcha } = binding
