#!/bin/bash

wasm-pack build --target web

rm -rf ../extension/pkg
cp -r pkg ../extension/
