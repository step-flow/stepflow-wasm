#!/bin/bash

# clean target
rm -rf pkg

# build wasm
wasm-pack build --out-dir pkg

# build TS helpers
cd src-ts
tsc
cd ..

# modify the package.json to point to the helpers
cd pkg
mv package.json package.bg.json
jq 'del(.files,.types,.main,.module) + {"module": "index.js", "types": "index.d.ts"}' package.bg.json > package.json
rm package.bg.json

# fix all paths
sed -i'.bak' 's/\.\.\/pkg\//\.\//g' *.ts
sed -i'.bak' 's/\.\.\/pkg\//\.\//g' *.js
rm *.bak

# we're uploading all files
rm .gitignore

# done!
cd ..
