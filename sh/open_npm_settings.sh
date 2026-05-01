#!/usr/bin/env bash

# List of packages
PACKAGES=(
  "@3-/captcha"
  "@3-/captcha-darwin-arm64"
  "@3-/captcha-darwin-x64"
  "@3-/captcha-linux-arm64-gnu"
  "@3-/captcha-linux-arm64-musl"
  "@3-/captcha-linux-x64-gnu"
  "@3-/captcha-linux-x64-musl"
  "@3-/captcha-win32-arm64-msvc"
  "@3-/captcha-win32-x64-msvc"
)

# Open each URL in the default browser
for PKG in "${PACKAGES[@]}"; do
  URL="https://www.npmjs.com/package/${PKG}/access"
  echo "Opening ${URL}..."
  open "$URL"
done
