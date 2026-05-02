#!/usr/bin/env bash
set -e

# Build the native module
npm run build:debug

# Run tests
npm test
