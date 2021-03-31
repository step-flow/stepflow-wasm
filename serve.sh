#!/bin/bash

TARGET_DIR=target/wasm32-unknown-unknown/release
cp index.html $TARGET_DIR
/usr/local/bin/serve $TARGET_DIR
