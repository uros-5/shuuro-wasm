#!/bin/bash
cd .. && wasm-pack build shuuro-wasm --out-dir pkg --target web --release
cd shuuro-wasm
rm -rf "DIR/ui-lishuuro/src/plugins/shuuro-wasm"
cp -r pkg "DIR/ui-lishuuro/src/plugins/shuuro-wasm" 