#!/usr/bin/env bash
set -e

# Build the native module
bun run build:debug

# Run tests
bun run test
