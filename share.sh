#!/bin/bash
cd .. && wasm-pack build shuuro-wasm --out-dir pkg --target web --release
cd shuuro-wasm
# rm -rf "/home/uros/Documents/programiranje/vue/ui-lishuuro/src/plugins/shuuro-wasm"
# cp -r pkg "/home/uros/Documents/programiranje/vue/ui-lishuuro/src/plugins/shuuro-wasm"