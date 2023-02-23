# gltf-viewer
A simple project for viewing gltf files in rust

To build for web:
```
cargo run --target wasm32-unknown-unknown
```

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/
```

```
cargo install -f wasm-bindgen-cli
```