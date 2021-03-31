#!/bin/bash

TARGET_DIR=./target/wasm32-unknown-unknown/release
TARGET_ORIG=$TARGET_DIR/stepflow_wasm.wasm
TARGET=$TARGET_DIR/stepflow.wasm
NPM_PKG_SRC=./src-pkg
NPM_PKG_TARGET_DIR=./pkg

# clean
rm -rf $NPM_PKG_TARGET_DIR

# build release
cargo build --target wasm32-unknown-unknown --release
echo BUILD
ls -lh $TARGET_ORIG

# wasm-opt
wasm-opt -o $TARGET -Oz $TARGET_ORIG
echo WASM-OPT
ls -lh $TARGET

# wasm-strip
wasm-strip $TARGET
echo WASM-STRIP
ls -lh $TARGET

# gzip just to see the size
gzip -f -k $TARGET
echo ZIPPED
ls -lh $TARGET.gz

# prepare npm package
mkdir $NPM_PKG_TARGET_DIR
cp $TARGET $NPM_PKG_TARGET_DIR
cp $NPM_PKG_SRC/* $NPM_PKG_TARGET_DIR

# compile Typescript sources
cd src-ts
tsc
cd ..
