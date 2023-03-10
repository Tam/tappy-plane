echo Clear previous build
rm -rf docs/wasm docs/assets

echo Build rust wasm binary
time cargo build --profile wasm-release --target wasm32-unknown-unknown

echo Bind wasm
time wasm-bindgen --out-name tappy-plane --out-dir docs/wasm --target web target/wasm32-unknown-unknown/wasm-release/tappy-plane.wasm

echo Optimise wasm
time wasm-opt -Oz --output optimized.wasm docs/wasm/tappy-plane_bg.wasm

echo Store optimised wasm
mv optimized.wasm docs/wasm/tappy-plane_bg.wasm

echo Link assets
cp -R ./assets ./docs/assets

echo Tidy generated files
rm docs/wasm/*.ts
time uglifyjs docs/wasm/tappy-plane.js -c -m -o docs/wasm/tappy-plane.js --module
