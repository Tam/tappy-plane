echo Clear previous build
rm -rf wasm

echo Build rust wasm binary
time cargo build --profile wasm-release --target wasm32-unknown-unknown

echo Bind wasm
time wasm-bindgen --out-name tappy-plane --out-dir wasm --target web target/wasm32-unknown-unknown/wasm-release/tappy-plane.wasm

echo Optimise wasm
time wasm-opt -Oz --output optimized.wasm wasm/tappy-plane_bg.wasm

echo Store optimised wasm
mv optimized.wasm wasm/tappy-plane_bg.wasm

echo Link assets
ln -s ./assets ./wasm/assets

echo Tidy generated files
rm wasm/*.ts
time uglifyjs wasm/tappy-plane.js -c -m -o wasm/tappy-plane.js --module
