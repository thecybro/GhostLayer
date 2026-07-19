#!/bin/bash

cd ghost || exit 1

wasm-pack build --target web

rm -rf ../extension/pkg
cp -r pkg ../extension/
