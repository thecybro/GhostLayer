#!/bin/bash
#
# Builds the Chrome Web Store upload package.
#
# Produces dist/ghostlayer-<version>.zip containing only what the extension
# needs at runtime. Test pages, TypeScript definitions and source maps are
# left out.

set -euo pipefail

cd "$(dirname "$0")"

VERSION=$(grep '"version"' extension/manifest.json | head -1 | cut -d'"' -f4)
OUT="dist/ghostlayer-${VERSION}.zip"

echo "Building the WebAssembly module"
# build.sh uses paths relative to the crate, so it has to run from there
(cd ghost && ./build.sh)

rm -rf dist
mkdir -p dist

echo "Packaging version ${VERSION}"
cd extension
zip -r "../${OUT}" . \
  -x "tests/*" \
  -x "settings/*" \
  -x "pkg/*.d.ts" \
  -x "pkg/package.json" \
  -x "pkg/.gitignore" \
  -x "*.map" \
  -x ".gitignore" \
  -x ".DS_Store" \
  -q
cd ..

echo
echo "Wrote ${OUT}"
unzip -l "${OUT}"
