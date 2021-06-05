# RBP UI

Risk balanced portfolio Rust/WebAssembly UI

## Development

### Pre-install

```sh
cargo install http-server
npm install -g yarn
```

### Install

Node.js stuff

```sh
yarn
```

### Build css

```sh
npx tailwindcss build styles.css -o pkg/styles.css
```

### Run

Live rebuild

```sh
cargo watch -s 'wasm-pack build --target web && rollup -c'
```

Serve

```sh
http-server
```

Go to http://localhost:8080/
